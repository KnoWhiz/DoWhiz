use super::{extract_emails, normalize_email, normalize_phone, normalize_slack_id, UserStore};
use tempfile::TempDir;

#[test]
fn normalize_email_handles_tags_and_case() {
    assert_eq!(
        normalize_email("Alice+test@Example.com"),
        Some("alice@example.com".to_string())
    );
    assert_eq!(
        normalize_email("<bob@Example.com>"),
        Some("bob@example.com".to_string())
    );
    assert_eq!(normalize_email("not-an-email"), None);
}

#[test]
fn normalize_phone_handles_various_formats() {
    assert_eq!(
        normalize_phone("+1 555 123 4567"),
        Some("+15551234567".to_string())
    );
    assert_eq!(
        normalize_phone("555-123-4567"),
        Some("5551234567".to_string())
    );
    assert_eq!(
        normalize_phone("+1(555)123-4567"),
        Some("+15551234567".to_string())
    );
    assert_eq!(normalize_phone(""), None);
    assert_eq!(normalize_phone("   "), None);
}

#[test]
fn normalize_slack_id_uppercases() {
    assert_eq!(normalize_slack_id("u12345"), Some("U12345".to_string()));
    assert_eq!(normalize_slack_id("  U12345  "), Some("U12345".to_string()));
    assert_eq!(normalize_slack_id(""), None);
}

#[test]
fn extract_emails_finds_all_candidates() {
    let raw = "Alice <alice@example.com>, bob@example.com; Carol <carol+tag@Example.com>";
    let emails = extract_emails(raw);
    assert_eq!(emails.len(), 3);
    assert!(emails.contains(&"alice@example.com".to_string()));
    assert!(emails.contains(&"bob@example.com".to_string()));
    assert!(emails.contains(&"carol@example.com".to_string()));
}

#[test]
fn user_store_get_or_create_is_stable_for_email() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("users.db");
    let store = UserStore::new(db_path).unwrap();

    let first = store
        .get_or_create_user("email", "Alice@Example.com")
        .unwrap();
    let second = store
        .get_or_create_user("email", "alice@example.com")
        .unwrap();

    assert_eq!(first.user_id, second.user_id);
    assert_eq!(first.identifier_type, "email");
    assert_eq!(first.identifier, "alice@example.com");
}

#[test]
fn user_store_get_or_create_is_stable_for_phone() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("users.db");
    let store = UserStore::new(db_path).unwrap();

    let first = store
        .get_or_create_user("phone", "+1 555 123 4567")
        .unwrap();
    let second = store.get_or_create_user("phone", "+15551234567").unwrap();

    assert_eq!(first.user_id, second.user_id);
    assert_eq!(first.identifier_type, "phone");
    assert_eq!(first.identifier, "+15551234567");
}

#[test]
fn user_store_get_or_create_is_stable_for_slack() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("users.db");
    let store = UserStore::new(db_path).unwrap();

    let first = store.get_or_create_user("slack", "u12345").unwrap();
    let second = store.get_or_create_user("slack", "U12345").unwrap();

    assert_eq!(first.user_id, second.user_id);
    assert_eq!(first.identifier_type, "slack");
    assert_eq!(first.identifier, "U12345");
}

#[test]
fn user_store_separates_by_identifier_type() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("users.db");
    let store = UserStore::new(db_path).unwrap();

    // Same identifier, different types = different users
    let email_user = store
        .get_or_create_user("email", "test@example.com")
        .unwrap();
    let gdocs_user = store
        .get_or_create_user("google_docs", "test@example.com")
        .unwrap();

    assert_ne!(email_user.user_id, gdocs_user.user_id);
    assert_eq!(email_user.identifier_type, "email");
    assert_eq!(gdocs_user.identifier_type, "google_docs");
}

#[test]
fn list_user_ids_returns_all_users() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("users.db");
    let store = UserStore::new(db_path).unwrap();

    let first = store
        .get_or_create_user("email", "first@example.com")
        .unwrap();
    let second = store.get_or_create_user("phone", "+15551234567").unwrap();

    let ids = store.list_user_ids().unwrap();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&first.user_id));
    assert!(ids.contains(&second.user_id));
}
