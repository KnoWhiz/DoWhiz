mod test_support;

use chrono::{Duration, Utc};
use scheduler_module::account_store::{AccountStore, AccountStoreError};
use uuid::Uuid;

fn get_test_account_store() -> Option<AccountStore> {
    let db_url = test_support::require_supabase_db_url("email_verification_e2e")?;
    match AccountStore::new(&db_url) {
        Ok(store) => Some(store),
        Err(e) => {
            eprintln!("Failed to create AccountStore: {}", e);
            None
        }
    }
}

/// Get an existing account for testing, or None if no accounts exist
/// We can't create test accounts because of the auth.users foreign key constraint
fn get_existing_test_account(store: &AccountStore) -> Option<Uuid> {
    // Query for any existing account
    // We'll use a raw query through the store's connection
    // For now, let's use a known test account or skip

    // Try to find the account by looking up a common test pattern
    // Or use environment variable for test account ID
    if let Ok(test_account_id) = std::env::var("TEST_ACCOUNT_ID") {
        if let Ok(uuid) = Uuid::parse_str(&test_account_id) {
            if store.get_account(uuid).ok().flatten().is_some() {
                return Some(uuid);
            }
        }
    }

    eprintln!("Skipping test: No TEST_ACCOUNT_ID env var set. Set TEST_ACCOUNT_ID to an existing account UUID to run these tests.");
    None
}

/// Clean up test email identifiers (but not the account itself)
fn cleanup_test_email(store: &AccountStore, account_id: Uuid, email: &str) {
    let _ = store.delete_identifier(account_id, "email", email);
}

/// Clean up test verification tokens by verifying and then unlinking
fn cleanup_test_token_and_email(store: &AccountStore, account_id: Uuid, token: &str, email: &str) {
    // Try to verify the token (consumes it)
    let _ = store.verify_email_token(token);
    // Then unlink the email
    let _ = store.delete_identifier(account_id, "email", email);
}

#[test]
fn email_verification_token_creation() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-create-{}@example.com", Uuid::new_v4());

    // Create verification token
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create verification token");

    assert_eq!(token.account_id, account_id);
    assert_eq!(token.email, test_email);
    assert!(!token.token.is_empty());
    assert!(token.expires_at > Utc::now());
    assert!(token.expires_at < Utc::now() + Duration::hours(25)); // Within 25 hours

    // Cleanup - verify token to consume it, then unlink
    cleanup_test_token_and_email(&store, account_id, &token.token, &test_email);
}

#[test]
fn email_verification_token_replaces_existing() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-replace-{}@example.com", Uuid::new_v4());

    // Create first token
    let token1 = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create first token");

    // Create second token for same email - should replace the first
    let token2 = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create second token");

    assert_ne!(token1.token, token2.token);

    // Verify first token is invalid now (was replaced)
    let result = store.verify_email_token(&token1.token);
    assert!(matches!(result, Err(AccountStoreError::TokenInvalid)));

    // Second token should still be valid
    let identifier = store
        .verify_email_token(&token2.token)
        .expect("verify second token");
    assert_eq!(identifier.identifier, test_email);

    // Cleanup
    cleanup_test_email(&store, account_id, &test_email);
}

#[test]
fn email_verification_success() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-success-{}@example.com", Uuid::new_v4());

    // Create verification token
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create verification token");

    // Verify the token
    let identifier = store
        .verify_email_token(&token.token)
        .expect("verify token");

    assert_eq!(identifier.account_id, account_id);
    assert_eq!(identifier.identifier_type, "email");
    assert_eq!(identifier.identifier, test_email);
    assert!(identifier.verified);

    // Token should be deleted after use - verifying again should fail
    let result = store.verify_email_token(&token.token);
    assert!(matches!(result, Err(AccountStoreError::TokenInvalid)));

    // Email should now be listed in account identifiers
    let identifiers = store.list_identifiers(account_id).expect("list identifiers");
    assert!(identifiers.iter().any(|id| id.identifier == test_email && id.verified));

    // Account should be findable by the email identifier
    let found_account = store
        .get_account_by_identifier("email", &test_email)
        .expect("lookup by email");
    assert!(found_account.is_some());
    assert_eq!(found_account.unwrap().id, account_id);

    // Cleanup
    cleanup_test_email(&store, account_id, &test_email);
}

#[test]
fn email_verification_invalid_token() {
    let Some(store) = get_test_account_store() else {
        return;
    };

    // Try to verify a non-existent token
    let fake_token = Uuid::new_v4().to_string();
    let result = store.verify_email_token(&fake_token);

    assert!(matches!(result, Err(AccountStoreError::TokenInvalid)));
}

#[test]
fn email_verification_links_to_account() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-link-{}@example.com", Uuid::new_v4());

    // Before verification, email should not be linked
    let not_found = store
        .get_account_by_identifier("email", &test_email)
        .expect("lookup before verification");
    assert!(not_found.is_none());

    // Create and verify token
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create token");

    let _identifier = store.verify_email_token(&token.token).expect("verify token");

    // After verification, email should be linked and verified
    let found = store
        .get_account_by_identifier("email", &test_email)
        .expect("lookup after verification");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, account_id);

    // Cleanup
    cleanup_test_email(&store, account_id, &test_email);
}

#[test]
fn email_verification_multiple_emails_same_account() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let email1 = format!("test-multi1-{}@example.com", Uuid::new_v4());
    let email2 = format!("test-multi2-{}@example.com", Uuid::new_v4());

    // Verify first email
    let token1 = store
        .create_email_verification_token(account_id, &email1)
        .expect("create token 1");
    store.verify_email_token(&token1.token).expect("verify token 1");

    // Verify second email
    let token2 = store
        .create_email_verification_token(account_id, &email2)
        .expect("create token 2");
    store.verify_email_token(&token2.token).expect("verify token 2");

    // Both emails should be linked to the same account
    let found1 = store
        .get_account_by_identifier("email", &email1)
        .expect("lookup email 1");
    let found2 = store
        .get_account_by_identifier("email", &email2)
        .expect("lookup email 2");

    assert!(found1.is_some());
    assert!(found2.is_some());
    assert_eq!(found1.unwrap().id, account_id);
    assert_eq!(found2.unwrap().id, account_id);

    // List identifiers should show both
    let identifiers = store.list_identifiers(account_id).expect("list identifiers");
    assert!(identifiers.iter().any(|id| id.identifier == email1));
    assert!(identifiers.iter().any(|id| id.identifier == email2));

    // Cleanup
    cleanup_test_email(&store, account_id, &email1);
    cleanup_test_email(&store, account_id, &email2);
}

#[test]
fn email_verification_unlink_email() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-unlink-{}@example.com", Uuid::new_v4());

    // Create and verify email
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create token");
    store.verify_email_token(&token.token).expect("verify token");

    // Verify it's linked
    let found = store
        .get_account_by_identifier("email", &test_email)
        .expect("lookup before unlink");
    assert!(found.is_some());

    // Unlink the email
    store
        .delete_identifier(account_id, "email", &test_email)
        .expect("delete identifier");

    // Email should no longer be linked
    let found_after = store
        .get_account_by_identifier("email", &test_email)
        .expect("lookup after unlink");
    assert!(found_after.is_none());

    // Trying to unlink again should fail
    let result = store.delete_identifier(account_id, "email", &test_email);
    assert!(matches!(result, Err(AccountStoreError::NotFound)));
}

#[test]
fn email_verification_token_expires_in_24_hours() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-expiry-{}@example.com", Uuid::new_v4());

    // Create verification token
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create verification token");

    // Verify token expires in approximately 24 hours (within a minute tolerance)
    let expected_expiry = Utc::now() + Duration::hours(24);
    let tolerance = Duration::minutes(1);

    assert!(token.expires_at > expected_expiry - tolerance);
    assert!(token.expires_at < expected_expiry + tolerance);

    // Cleanup
    cleanup_test_token_and_email(&store, account_id, &token.token, &test_email);
}

#[test]
fn email_verification_token_consumed_after_use() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-consumed-{}@example.com", Uuid::new_v4());

    // Create and verify token
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create token");

    // First verification should succeed
    let identifier = store.verify_email_token(&token.token).expect("first verify");
    assert_eq!(identifier.identifier, test_email);

    // Second verification should fail (token consumed)
    let result = store.verify_email_token(&token.token);
    assert!(matches!(result, Err(AccountStoreError::TokenInvalid)));

    // Third verification should also fail
    let result = store.verify_email_token(&token.token);
    assert!(matches!(result, Err(AccountStoreError::TokenInvalid)));

    // Cleanup
    cleanup_test_email(&store, account_id, &test_email);
}

#[test]
fn email_verification_identifier_is_marked_verified() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-verified-flag-{}@example.com", Uuid::new_v4());

    // Create and verify token
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create token");
    let identifier = store.verify_email_token(&token.token).expect("verify token");

    // The returned identifier should be marked as verified
    assert!(identifier.verified);
    assert_eq!(identifier.identifier_type, "email");

    // Also verify through list_identifiers
    let identifiers = store.list_identifiers(account_id).expect("list");
    let found = identifiers.iter().find(|id| id.identifier == test_email);
    assert!(found.is_some());
    assert!(found.unwrap().verified);

    // Cleanup
    cleanup_test_email(&store, account_id, &test_email);
}

#[test]
fn email_verification_empty_token_is_invalid() {
    let Some(store) = get_test_account_store() else {
        return;
    };

    // Empty string token should be invalid
    let result = store.verify_email_token("");
    assert!(matches!(result, Err(AccountStoreError::TokenInvalid)));
}

#[test]
fn email_verification_whitespace_token_is_invalid() {
    let Some(store) = get_test_account_store() else {
        return;
    };

    // Whitespace-only tokens should be invalid
    let result = store.verify_email_token("   ");
    assert!(matches!(result, Err(AccountStoreError::TokenInvalid)));

    let result = store.verify_email_token("\t\n");
    assert!(matches!(result, Err(AccountStoreError::TokenInvalid)));
}

#[test]
fn email_verification_token_is_uuid_format() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-uuid-{}@example.com", Uuid::new_v4());

    // Create verification token
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create verification token");

    // Token should be a valid UUID
    assert!(Uuid::parse_str(&token.token).is_ok(), "Token should be a valid UUID");

    // Cleanup
    cleanup_test_token_and_email(&store, account_id, &token.token, &test_email);
}

#[test]
fn email_verification_tokens_are_unique() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let email1 = format!("test-unique1-{}@example.com", Uuid::new_v4());
    let email2 = format!("test-unique2-{}@example.com", Uuid::new_v4());

    // Create tokens for different emails
    let token1 = store
        .create_email_verification_token(account_id, &email1)
        .expect("create token 1");
    let token2 = store
        .create_email_verification_token(account_id, &email2)
        .expect("create token 2");

    // Tokens should be different
    assert_ne!(token1.token, token2.token);

    // Both tokens should be valid
    let id1 = store.verify_email_token(&token1.token).expect("verify token 1");
    let id2 = store.verify_email_token(&token2.token).expect("verify token 2");

    assert_eq!(id1.identifier, email1);
    assert_eq!(id2.identifier, email2);

    // Cleanup
    cleanup_test_email(&store, account_id, &email1);
    cleanup_test_email(&store, account_id, &email2);
}

#[test]
fn email_verification_relink_same_email() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let test_email = format!("test-relink-{}@example.com", Uuid::new_v4());

    // First: link email
    let token1 = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create token 1");
    store.verify_email_token(&token1.token).expect("verify token 1");

    // Verify it's linked
    let found = store
        .get_account_by_identifier("email", &test_email)
        .expect("lookup");
    assert!(found.is_some());

    // Unlink it
    store
        .delete_identifier(account_id, "email", &test_email)
        .expect("unlink");

    // Verify it's unlinked
    let not_found = store
        .get_account_by_identifier("email", &test_email)
        .expect("lookup after unlink");
    assert!(not_found.is_none());

    // Re-link the same email
    let token2 = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create token 2");
    store.verify_email_token(&token2.token).expect("verify token 2");

    // Should be linked again
    let found_again = store
        .get_account_by_identifier("email", &test_email)
        .expect("lookup after relink");
    assert!(found_again.is_some());
    assert_eq!(found_again.unwrap().id, account_id);

    // Cleanup
    cleanup_test_email(&store, account_id, &test_email);
}

#[test]
fn email_verification_special_characters_in_email() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    // Test emails with special characters (valid per RFC 5321)
    let unique = Uuid::new_v4();
    let test_email = format!("test+tag-{}@example.com", unique);

    // Create and verify
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create token");
    let identifier = store.verify_email_token(&token.token).expect("verify token");

    assert_eq!(identifier.identifier, test_email);

    // Should be findable
    let found = store
        .get_account_by_identifier("email", &test_email)
        .expect("lookup");
    assert!(found.is_some());

    // Cleanup
    cleanup_test_email(&store, account_id, &test_email);
}

#[test]
fn email_verification_long_email_address() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    // Create a long but valid email (64 char local part is the max per RFC)
    let unique = Uuid::new_v4().to_string().replace("-", "");
    let local_part = format!("test-long-{}-padding", unique);
    let test_email = format!("{}@example.com", &local_part[..50.min(local_part.len())]);

    // Create and verify
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create token");
    let identifier = store.verify_email_token(&token.token).expect("verify token");

    assert_eq!(identifier.identifier, test_email);

    // Cleanup
    cleanup_test_email(&store, account_id, &test_email);
}

#[test]
fn email_verification_preserves_email_case() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let unique = Uuid::new_v4();
    let test_email = format!("Test.Case.Mixed-{}@Example.COM", unique);

    // Create and verify - should preserve the case as entered
    let token = store
        .create_email_verification_token(account_id, &test_email)
        .expect("create token");
    let identifier = store.verify_email_token(&token.token).expect("verify token");

    // The stored identifier should match exactly what was provided
    assert_eq!(identifier.identifier, test_email);

    // Cleanup
    cleanup_test_email(&store, account_id, &test_email);
}
