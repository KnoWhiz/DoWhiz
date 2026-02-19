//! End-to-end test for unified memo.md storage via Azure Blob
//!
//! This test requires:
//! - SUPABASE_PROJECT_URL: Supabase project URL
//! - SUPABASE_ANON_KEY: Supabase anon key
//! - AZURE_STORAGE_CONNECTION_STRING: Azure Blob storage connection string
//! - AZURE_STORAGE_CONTAINER: Container name (e.g., "memos")
//! - The DoWhiz service running at SERVICE_URL (default: http://localhost:9001)
//!
//! Run with: cargo test --test unified_memo_e2e -- --ignored --nocapture

use scheduler_module::blob_store::BlobStore;
use scheduler_module::channel::Channel;
use scheduler_module::memory_diff::{MemoryDiff, SectionChange};
use scheduler_module::memory_queue::{global_memory_queue, MemoryWriteRequest};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct SupabaseAuthResponse {
    access_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DoWhizSignupResponse {
    account_id: Uuid,
    auth_user_id: Uuid,
}

#[derive(Debug, Deserialize)]
struct DoWhizAccountResponse {
    account_id: Uuid,
    identifiers: Vec<IdentifierInfo>,
}

#[derive(Debug, Deserialize)]
struct IdentifierInfo {
    identifier_type: String,
    identifier: String,
    verified: bool,
}

/// Test the full unified memo flow:
/// 1. Login to Supabase to get access token
/// 2. Create DoWhiz account via /auth/signup
/// 3. Link and verify a phone identifier
/// 4. Look up account by channel+identifier
/// 5. Submit memory diff to queue (should route to Azure Blob)
/// 6. Verify memo was written to Azure Blob
/// 7. Clean up
#[tokio::test]
#[ignore] // Requires running service and credentials
async fn test_unified_memo_azure_blob_routing() {
    dotenvy::dotenv().ok();

    let service_url = std::env::var("SERVICE_URL").unwrap_or_else(|_| "http://localhost:9001".to_string());
    let supabase_url = std::env::var("SUPABASE_PROJECT_URL").expect("SUPABASE_PROJECT_URL required");
    let supabase_anon_key = std::env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY required");
    let test_email = std::env::var("TEST_EMAIL").unwrap_or_else(|_| "dylantang12@gmail.com".to_string());
    let test_password = std::env::var("TEST_PASSWORD").unwrap_or_else(|_| "testpassword123".to_string());

    let client = reqwest::Client::new();
    let blob_store = BlobStore::from_env().expect("BlobStore::from_env");

    // Step 1: Login to Supabase to get access token
    println!("Step 1: Logging into Supabase...");
    let login_resp = client
        .post(format!("{}/auth/v1/token?grant_type=password", supabase_url))
        .header("apikey", &supabase_anon_key)
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"email": "{}", "password": "{}"}}"#, test_email, test_password))
        .send()
        .await
        .expect("login request");

    let login_body: serde_json::Value = login_resp.json().await.expect("parse login response");
    let access_token = login_body["access_token"]
        .as_str()
        .expect("access_token in response")
        .to_string();
    println!("✅ Got access token: {}...", &access_token[..20]);

    // Step 2: Create/get DoWhiz account
    println!("\nStep 2: Creating DoWhiz account...");
    let signup_resp = client
        .post(format!("{}/auth/signup", service_url))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("signup request");

    let signup_body: DoWhizSignupResponse = signup_resp.json().await.expect("parse signup response");
    let account_id = signup_body.account_id;
    println!("✅ Account ID: {}", account_id);

    // Step 3: Link a phone identifier
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32;
    let test_phone = format!("+1555{}", timestamp % 10000000);
    println!("\nStep 3: Linking phone identifier: {}", test_phone);

    let link_resp = client
        .post(format!("{}/auth/link", service_url))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"identifier_type": "phone", "identifier": "{}"}}"#, test_phone))
        .send()
        .await
        .expect("link request");

    assert!(link_resp.status().is_success(), "Link failed: {:?}", link_resp.text().await);
    println!("✅ Phone linked");

    // Step 4: Verify the identifier
    println!("\nStep 4: Verifying phone identifier...");
    let verify_resp = client
        .post(format!("{}/auth/verify", service_url))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"identifier_type": "phone", "identifier": "{}", "code": "123456"}}"#, test_phone))
        .send()
        .await
        .expect("verify request");

    assert!(verify_resp.status().is_success(), "Verify failed: {:?}", verify_resp.text().await);
    println!("✅ Phone verified");

    // Step 5: Test account lookup (simulating what executor does)
    println!("\nStep 5: Testing account lookup by channel...");
    let phone_clone = test_phone.clone();
    let looked_up_account_id = tokio::task::spawn_blocking(move || {
        scheduler_module::account_store::lookup_account_by_channel(&Channel::Sms, &phone_clone)
    })
    .await
    .expect("spawn_blocking");

    assert_eq!(
        looked_up_account_id,
        Some(account_id),
        "Account lookup should find the account"
    );
    println!("✅ Account lookup successful: {:?}", looked_up_account_id);

    // Step 6: Submit a memory diff to the queue
    println!("\nStep 6: Submitting memory diff to queue...");
    let diff = MemoryDiff {
        changed_sections: HashMap::from([(
            "Contacts".to_string(),
            SectionChange::Added(vec![
                "- Test Contact: 555-1234".to_string(),
                "- From unified memo test".to_string(),
            ]),
        )]),
    };

    let request = MemoryWriteRequest {
        account_id: Some(account_id),
        user_id: "test-user".to_string(),
        user_memory_dir: PathBuf::from("/tmp/unused"),
        diff,
    };

    tokio::task::spawn_blocking(move || {
        global_memory_queue().submit(request)
    })
    .await
    .expect("spawn_blocking")
    .expect("submit to memory queue");
    println!("✅ Memory diff submitted");

    // Step 7: Verify memo was written to Azure Blob
    println!("\nStep 7: Verifying memo in Azure Blob...");
    let memo_content = blob_store
        .read_memo(account_id)
        .await
        .expect("read_memo from blob");
    println!("Memo content:\n{}", memo_content);

    assert!(
        memo_content.contains("Test Contact"),
        "Memo should contain the test contact"
    );
    assert!(
        memo_content.contains("unified memo test"),
        "Memo should contain the test marker"
    );
    println!("✅ Memo content verified in Azure Blob");

    // Step 8: Clean up - unlink identifier and delete blob
    println!("\nStep 8: Cleaning up...");

    // Unlink the phone
    let unlink_resp = client
        .delete(format!("{}/auth/unlink", service_url))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"identifier_type": "phone", "identifier": "{}"}}"#, test_phone))
        .send()
        .await
        .expect("unlink request");
    assert!(unlink_resp.status().is_success(), "Unlink failed");
    println!("✅ Phone unlinked");

    // Delete the blob
    blob_store
        .delete_memo(account_id)
        .await
        .expect("delete_memo from blob");
    println!("✅ Blob deleted");

    // Note: We don't delete the DoWhiz account so it can be reused for future tests

    println!("\n========================================");
    println!("✅ Unified memo test passed!");
    println!("========================================");
    println!("  - Supabase auth: ✅");
    println!("  - DoWhiz account: ✅");
    println!("  - Identifier linking: ✅");
    println!("  - Account lookup by channel: ✅");
    println!("  - Memory diff → Azure Blob: ✅");
    println!("  - Blob content verified: ✅");
}

/// Test multiple memory diffs accumulate correctly
#[tokio::test]
#[ignore] // Requires running service and credentials
async fn test_multiple_memory_diffs() {
    dotenvy::dotenv().ok();

    let service_url = std::env::var("SERVICE_URL").unwrap_or_else(|_| "http://localhost:9001".to_string());
    let supabase_url = std::env::var("SUPABASE_PROJECT_URL").expect("SUPABASE_PROJECT_URL required");
    let supabase_anon_key = std::env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY required");
    let test_email = std::env::var("TEST_EMAIL").unwrap_or_else(|_| "dylantang12@gmail.com".to_string());
    let test_password = std::env::var("TEST_PASSWORD").unwrap_or_else(|_| "testpassword123".to_string());

    let client = reqwest::Client::new();
    let blob_store = BlobStore::from_env().expect("BlobStore::from_env");

    // Login to Supabase
    println!("Logging into Supabase...");
    let login_resp = client
        .post(format!("{}/auth/v1/token?grant_type=password", supabase_url))
        .header("apikey", &supabase_anon_key)
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"email": "{}", "password": "{}"}}"#, test_email, test_password))
        .send()
        .await
        .expect("login request");

    let login_body: serde_json::Value = login_resp.json().await.expect("parse login response");
    let access_token = login_body["access_token"]
        .as_str()
        .expect("access_token in response")
        .to_string();

    // Get/create DoWhiz account
    let signup_resp = client
        .post(format!("{}/auth/signup", service_url))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("signup request");

    let signup_body: DoWhizSignupResponse = signup_resp.json().await.expect("parse signup response");
    let account_id = signup_body.account_id;
    println!("Account ID: {}", account_id);

    // Clear any existing memo first
    let _ = blob_store.delete_memo(account_id).await;

    // Send first diff - add to Contacts
    println!("\nSending diff 1: Adding contact Alice...");
    let diff1 = MemoryDiff {
        changed_sections: HashMap::from([(
            "Contacts".to_string(),
            SectionChange::Added(vec!["- Alice: alice@example.com".to_string()]),
        )]),
    };

    let request1 = MemoryWriteRequest {
        account_id: Some(account_id),
        user_id: "test-user".to_string(),
        user_memory_dir: PathBuf::from("/tmp/unused"),
        diff: diff1,
    };

    tokio::task::spawn_blocking(move || global_memory_queue().submit(request1))
        .await
        .expect("spawn_blocking")
        .expect("submit diff 1");
    println!("✅ Diff 1 submitted");

    // Send second diff - add another contact
    println!("\nSending diff 2: Adding contact Bob...");
    let diff2 = MemoryDiff {
        changed_sections: HashMap::from([(
            "Contacts".to_string(),
            SectionChange::Added(vec!["- Bob: bob@example.com".to_string()]),
        )]),
    };

    let request2 = MemoryWriteRequest {
        account_id: Some(account_id),
        user_id: "test-user".to_string(),
        user_memory_dir: PathBuf::from("/tmp/unused"),
        diff: diff2,
    };

    tokio::task::spawn_blocking(move || global_memory_queue().submit(request2))
        .await
        .expect("spawn_blocking")
        .expect("submit diff 2");
    println!("✅ Diff 2 submitted");

    // Send third diff - add to a different section
    println!("\nSending diff 3: Adding preference...");
    let diff3 = MemoryDiff {
        changed_sections: HashMap::from([(
            "Preferences".to_string(),
            SectionChange::Added(vec!["- Prefers dark mode".to_string()]),
        )]),
    };

    let request3 = MemoryWriteRequest {
        account_id: Some(account_id),
        user_id: "test-user".to_string(),
        user_memory_dir: PathBuf::from("/tmp/unused"),
        diff: diff3,
    };

    tokio::task::spawn_blocking(move || global_memory_queue().submit(request3))
        .await
        .expect("spawn_blocking")
        .expect("submit diff 3");
    println!("✅ Diff 3 submitted");

    // Verify all diffs accumulated
    println!("\nVerifying accumulated memo...");
    let memo_content = blob_store
        .read_memo(account_id)
        .await
        .expect("read_memo from blob");
    println!("Final memo content:\n{}", memo_content);

    // Check all additions are present
    assert!(memo_content.contains("Alice"), "Should contain Alice");
    assert!(memo_content.contains("Bob"), "Should contain Bob");
    assert!(memo_content.contains("dark mode"), "Should contain preference");

    println!("\n========================================");
    println!("✅ Multiple diffs test passed!");
    println!("========================================");
    println!("  - Diff 1 (Alice): ✅");
    println!("  - Diff 2 (Bob): ✅");
    println!("  - Diff 3 (Preferences): ✅");
    println!("  - All accumulated correctly: ✅");

    // Cleanup
    blob_store.delete_memo(account_id).await.expect("delete blob");
    println!("\n✅ Cleaned up blob");
}

/// Test that without an account, memory writes go to local storage
#[test]
#[ignore] // Requires file system access
fn test_fallback_to_local_storage() {
    use std::fs;
    use tempfile::TempDir;

    let temp = TempDir::new().expect("tempdir");
    let memory_dir = temp.path().join("memory");
    fs::create_dir_all(&memory_dir).expect("create memory dir");

    // Write initial memo
    let initial = "# Memo\n\n## Contacts\n\n## Preferences\n";
    fs::write(memory_dir.join("memo.md"), initial).expect("write initial");

    // Submit diff WITHOUT account_id (should use local storage)
    let diff = MemoryDiff {
        changed_sections: HashMap::from([(
            "Contacts".to_string(),
            SectionChange::Added(vec!["- Local Storage Test".to_string()]),
        )]),
    };

    let request = MemoryWriteRequest {
        account_id: None, // No account = local storage
        user_id: "test-user".to_string(),
        user_memory_dir: memory_dir.clone(),
        diff,
    };

    global_memory_queue()
        .submit(request)
        .expect("submit to memory queue");

    // Verify local file was updated
    let content = fs::read_to_string(memory_dir.join("memo.md")).expect("read memo");
    assert!(
        content.contains("Local Storage Test"),
        "Local memo should contain the test content"
    );

    println!("✅ Local storage fallback test passed!");
}
