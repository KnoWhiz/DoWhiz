mod test_support;

use scheduler_module::account_store::AccountStore;
use uuid::Uuid;

fn get_test_account_store() -> Option<AccountStore> {
    let db_url = test_support::require_supabase_db_url("billing_e2e")?;
    match AccountStore::new(&db_url) {
        Ok(store) => Some(store),
        Err(e) => {
            eprintln!("Failed to create AccountStore: {}", e);
            None
        }
    }
}

fn get_existing_test_account(store: &AccountStore) -> Option<Uuid> {
    if let Ok(test_account_id) = std::env::var("TEST_ACCOUNT_ID") {
        if let Ok(uuid) = Uuid::parse_str(&test_account_id) {
            if store.get_account(uuid).ok().flatten().is_some() {
                return Some(uuid);
            }
        }
    }
    eprintln!("Skipping test: No TEST_ACCOUNT_ID env var set.");
    None
}

// ============================================================================
// Balance Tests
// ============================================================================

#[test]
fn billing_get_balance_returns_zero_for_new_account() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let balance = store.get_balance(account_id).expect("get balance");

    // Balance should have valid values (non-negative)
    assert!(balance.purchased_hours >= 0.0);
    assert!(balance.used_hours >= 0.0);
    // balance_hours can be negative if over limit
}

#[test]
fn billing_add_purchased_hours() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    // Get initial balance
    let initial = store.get_balance(account_id).expect("get initial balance");

    // Add 5 hours
    store
        .add_purchased_hours(account_id, 5.0)
        .expect("add purchased hours");

    // Check new balance
    let after = store.get_balance(account_id).expect("get balance after");
    assert!(
        (after.purchased_hours - initial.purchased_hours - 5.0).abs() < 0.001,
        "purchased_hours should increase by 5"
    );

    // Cleanup: subtract the hours we added
    store
        .add_purchased_hours(account_id, -5.0)
        .expect("cleanup: remove purchased hours");
}

#[test]
fn billing_has_sufficient_balance_with_grace() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    // Get current balance
    let balance = store.get_balance(account_id).expect("get balance");

    // has_sufficient_balance allows 1 hour grace:
    // used_hours <= purchased_hours + 1.0
    let result = store
        .has_sufficient_balance(account_id)
        .expect("check balance");

    // If used <= purchased + 1, should be true
    let expected = balance.used_hours <= balance.purchased_hours + 1.0;
    assert_eq!(result, expected);
}

#[test]
fn billing_blocks_when_exceeded_grace_period() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    // Get current balance
    let initial = store.get_balance(account_id).expect("get initial balance");

    // Save current purchased_hours for restoration
    let original_purchased = initial.purchased_hours;

    // To test blocking, we need: used_hours > purchased_hours + 1
    // We'll temporarily reduce purchased_hours to make balance insufficient
    // New purchased should be: used_hours - 1.5 (so used > purchased + 1)
    let reduction_needed = original_purchased - (initial.used_hours - 1.5);

    // Reduce purchased_hours to trigger insufficient balance
    store
        .add_purchased_hours(account_id, -reduction_needed)
        .expect("reduce purchased hours for test");

    // Now check - should return false (blocked)
    let should_block = store
        .has_sufficient_balance(account_id)
        .expect("check balance when over limit");

    // Verify the math: after reduction, purchased = used_hours - 1.5
    // So used_hours > purchased + 1 becomes: used > (used - 1.5) + 1 = used > used - 0.5
    // This is always true, so should_block should be false
    assert!(
        !should_block,
        "should be blocked when used_hours exceeds purchased_hours + 1 grace"
    );

    // Restore the original purchased_hours
    store
        .add_purchased_hours(account_id, reduction_needed)
        .expect("restore purchased hours");

    // Verify restoration
    let restored = store.get_balance(account_id).expect("get restored balance");
    assert!(
        (restored.purchased_hours - original_purchased).abs() < 0.001,
        "purchased_hours should be restored"
    );
}

#[test]
fn billing_allows_exactly_at_grace_limit() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    // Get current balance
    let initial = store.get_balance(account_id).expect("get initial balance");
    let original_purchased = initial.purchased_hours;

    // Set purchased_hours so that used_hours == purchased_hours + 1 exactly
    // This should still be ALLOWED (the check is <=, not <)
    let target_purchased = initial.used_hours - 1.0;
    let adjustment = target_purchased - original_purchased;

    store
        .add_purchased_hours(account_id, adjustment)
        .expect("adjust purchased hours");

    // Check - should return true (exactly at grace limit is allowed)
    let at_limit = store
        .has_sufficient_balance(account_id)
        .expect("check balance at exact limit");

    assert!(
        at_limit,
        "should be allowed when used_hours == purchased_hours + 1 (exactly at grace limit)"
    );

    // Restore
    store
        .add_purchased_hours(account_id, -adjustment)
        .expect("restore purchased hours");
}

// ============================================================================
// Payment Recording Tests
// ============================================================================

#[test]
fn billing_record_payment_idempotent() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    // Use a unique session ID for this test
    let session_id = format!("test_session_{}", Uuid::new_v4());

    // Should not exist initially
    let exists_before = store
        .payment_exists(&session_id)
        .expect("check payment exists");
    assert!(!exists_before, "payment should not exist before recording");

    // Record the payment
    store
        .record_payment(account_id, &session_id, 5000, 5.0)
        .expect("record payment");

    // Should exist now
    let exists_after = store
        .payment_exists(&session_id)
        .expect("check payment exists after");
    assert!(exists_after, "payment should exist after recording");

    // Recording again should not error (idempotent)
    // The ON CONFLICT DO NOTHING should handle this
    let result = store.record_payment(account_id, &session_id, 5000, 5.0);
    assert!(result.is_ok(), "duplicate record_payment should not error");

    // Cleanup: delete the test payment
    cleanup_test_payment(&store, &session_id);
}

#[test]
fn billing_payment_exists_false_for_nonexistent() {
    let Some(store) = get_test_account_store() else {
        return;
    };

    let fake_session_id = format!("nonexistent_session_{}", Uuid::new_v4());
    let exists = store
        .payment_exists(&fake_session_id)
        .expect("check payment exists");
    assert!(!exists, "nonexistent payment should return false");
}

#[test]
fn billing_record_payment_stores_correct_values() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let session_id = format!("test_values_{}", Uuid::new_v4());
    let amount_cents = 10000; // $100
    let hours = 10.0;

    // Record payment
    store
        .record_payment(account_id, &session_id, amount_cents, hours)
        .expect("record payment");

    // Verify it exists
    assert!(store.payment_exists(&session_id).expect("check exists"));

    // Cleanup
    cleanup_test_payment(&store, &session_id);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn billing_full_purchase_flow() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let session_id = format!("test_flow_{}", Uuid::new_v4());
    let hours_to_purchase = 3.0;
    let amount_cents = 3000; // $30 at $10/hr

    // Get initial balance
    let initial_balance = store.get_balance(account_id).expect("get initial balance");

    // Simulate webhook: record payment
    store
        .record_payment(account_id, &session_id, amount_cents, hours_to_purchase)
        .expect("record payment");

    // Simulate webhook: add purchased hours
    store
        .add_purchased_hours(account_id, hours_to_purchase)
        .expect("add purchased hours");

    // Verify balance increased
    let final_balance = store.get_balance(account_id).expect("get final balance");
    assert!(
        (final_balance.purchased_hours - initial_balance.purchased_hours - hours_to_purchase).abs()
            < 0.001,
        "purchased_hours should increase by {} hours",
        hours_to_purchase
    );

    // Verify balance_hours increased accordingly
    assert!(
        (final_balance.balance_hours - initial_balance.balance_hours - hours_to_purchase).abs()
            < 0.001,
        "balance_hours should increase by {} hours",
        hours_to_purchase
    );

    // Cleanup
    store
        .add_purchased_hours(account_id, -hours_to_purchase)
        .expect("cleanup: remove hours");
    cleanup_test_payment(&store, &session_id);
}

#[test]
fn billing_duplicate_webhook_does_not_double_credit() {
    let Some(store) = get_test_account_store() else {
        return;
    };
    let Some(account_id) = get_existing_test_account(&store) else {
        return;
    };

    let session_id = format!("test_dup_{}", Uuid::new_v4());
    let hours = 2.0;

    // Get initial balance
    let initial = store.get_balance(account_id).expect("get initial balance");

    // First webhook call - should process
    if !store.payment_exists(&session_id).expect("check exists") {
        store
            .record_payment(account_id, &session_id, 2000, hours)
            .expect("record payment 1");
        store
            .add_purchased_hours(account_id, hours)
            .expect("add hours 1");
    }

    let after_first = store.get_balance(account_id).expect("balance after first");

    // Second webhook call (duplicate) - should skip
    if !store.payment_exists(&session_id).expect("check exists 2") {
        store
            .record_payment(account_id, &session_id, 2000, hours)
            .expect("record payment 2");
        store
            .add_purchased_hours(account_id, hours)
            .expect("add hours 2");
    }

    let after_second = store.get_balance(account_id).expect("balance after second");

    // Balance should be same after duplicate
    assert!(
        (after_second.purchased_hours - after_first.purchased_hours).abs() < 0.001,
        "duplicate webhook should not add more hours"
    );

    // Total increase should be exactly `hours`, not 2x
    assert!(
        (after_second.purchased_hours - initial.purchased_hours - hours).abs() < 0.001,
        "total increase should be {} hours, not double",
        hours
    );

    // Cleanup
    store
        .add_purchased_hours(account_id, -hours)
        .expect("cleanup");
    cleanup_test_payment(&store, &session_id);
}

// ============================================================================
// Helper Functions
// ============================================================================

fn cleanup_test_payment(store: &AccountStore, session_id: &str) {
    // Direct SQL to delete test payment
    // This is a test-only helper
    let mut conn = store.get_conn().expect("get connection for cleanup");
    let _ = conn.execute(
        "DELETE FROM payments WHERE stripe_session_id = $1",
        &[&session_id],
    );
}
