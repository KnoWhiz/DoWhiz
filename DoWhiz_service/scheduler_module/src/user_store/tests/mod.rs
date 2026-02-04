use super::{extract_emails, normalize_email, UserStore};
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
fn extract_emails_finds_all_candidates() {
    let raw = "Alice <alice@example.com>, bob@example.com; Carol <carol+tag@Example.com>";
    let emails = extract_emails(raw);
    assert_eq!(emails.len(), 3);
    assert!(emails.contains(&"alice@example.com".to_string()));
    assert!(emails.contains(&"bob@example.com".to_string()));
    assert!(emails.contains(&"carol@example.com".to_string()));
}

#[test]
fn user_store_get_or_create_is_stable() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("users.db");
    let store = UserStore::new(db_path).unwrap();

    let first = store.get_or_create_user("Alice@Example.com").unwrap();
    let second = store.get_or_create_user("alice@example.com").unwrap();

    assert_eq!(first.user_id, second.user_id);
    assert_eq!(first.email, "alice@example.com");
}

#[test]
fn list_user_ids_returns_all_users() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("users.db");
    let store = UserStore::new(db_path).unwrap();

    let first = store.get_or_create_user("first@example.com").unwrap();
    let second = store.get_or_create_user("second@example.com").unwrap();

    let ids = store.list_user_ids().unwrap();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&first.user_id));
    assert!(ids.contains(&second.user_id));
}
