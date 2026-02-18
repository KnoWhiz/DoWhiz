//! End-to-end integration tests for the google-docs CLI.
//!
//! These tests require:
//! - GOOGLE_DOCS_CLI_E2E=1 to enable
//! - GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, GOOGLE_REFRESH_TOKEN set
//! - GOOGLE_DOCS_TEST_DOC_ID set to a test document ID
//!
//! The test document should be a Google Doc that the service account has edit access to.
//! The tests will MODIFY the document content, so use a dedicated test document.

use std::env;
use std::path::Path;
use std::process::Command;

fn env_enabled(key: &str) -> bool {
    matches!(env::var(key).as_deref(), Ok("1"))
}

fn get_cli_path() -> std::path::PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .parent()
        .unwrap()
        .join("bin")
        .join("google-docs")
}

fn run_cli(args: &[&str]) -> (bool, String, String) {
    let cli_path = get_cli_path();
    let output = Command::new(&cli_path)
        .args(args)
        .output()
        .expect("Failed to run google-docs CLI");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.success(), stdout, stderr)
}

/// Test that list-documents works with real credentials.
#[test]
fn google_docs_cli_e2e_list_documents() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let (success, stdout, stderr) = run_cli(&["list-documents"]);

    assert!(
        success,
        "list-documents should succeed with valid credentials.\nstdout: {}\nstderr: {}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Found") && stdout.contains("documents"),
        "Output should list documents.\nstdout: {}",
        stdout
    );
}

/// Test that read-document works with real credentials.
#[test]
fn google_docs_cli_e2e_read_document() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let doc_id = env::var("GOOGLE_DOCS_TEST_DOC_ID").unwrap_or_default();
    if doc_id.is_empty() {
        eprintln!("GOOGLE_DOCS_TEST_DOC_ID not set; skipping read-document test.");
        return;
    }

    let (success, stdout, stderr) = run_cli(&["read-document", &doc_id]);

    assert!(
        success,
        "read-document should succeed.\nstdout: {}\nstderr: {}",
        stdout, stderr
    );
    // Document should have some content
    assert!(
        !stdout.trim().is_empty(),
        "Document content should not be empty"
    );
}

/// Test that list-comments works with real credentials.
#[test]
fn google_docs_cli_e2e_list_comments() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let doc_id = env::var("GOOGLE_DOCS_TEST_DOC_ID").unwrap_or_default();
    if doc_id.is_empty() {
        eprintln!("GOOGLE_DOCS_TEST_DOC_ID not set; skipping list-comments test.");
        return;
    }

    let (success, stdout, stderr) = run_cli(&["list-comments", &doc_id]);

    assert!(
        success,
        "list-comments should succeed.\nstdout: {}\nstderr: {}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Found") && stdout.contains("comments"),
        "Output should list comments count.\nstdout: {}",
        stdout
    );
}

/// Test mark-deletion: Mark text with red strikethrough.
/// This test requires a document with specific test text.
#[test]
fn google_docs_cli_e2e_mark_deletion() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let doc_id = env::var("GOOGLE_DOCS_TEST_DOC_ID").unwrap_or_default();
    if doc_id.is_empty() {
        eprintln!("GOOGLE_DOCS_TEST_DOC_ID not set; skipping mark-deletion test.");
        return;
    }

    // First, read the document to find a text segment
    let (success, content, _) = run_cli(&["read-document", &doc_id]);
    if !success || content.trim().is_empty() {
        eprintln!("Could not read document content; skipping mark-deletion test.");
        return;
    }

    // Find first non-empty line to mark for deletion
    let test_text = content
        .lines()
        .find(|line| !line.trim().is_empty() && line.len() > 3)
        .map(|s| s.trim())
        .unwrap_or("");

    if test_text.is_empty() || test_text.len() < 3 {
        eprintln!("No suitable text found in document for mark-deletion test.");
        return;
    }

    // Take first 10 chars to mark
    let mark_text = &test_text[..test_text.len().min(20)];

    let (success, stdout, stderr) =
        run_cli(&["mark-deletion", &doc_id, &format!("--find={}", mark_text)]);

    assert!(
        success,
        "mark-deletion should succeed.\nstdout: {}\nstderr: {}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Successfully marked") || stdout.contains("red strikethrough"),
        "Output should confirm deletion marking.\nstdout: {}",
        stdout
    );

    println!(
        "✓ mark-deletion test passed: marked '{}' for deletion",
        mark_text
    );
}

/// Test insert-suggestion: Insert blue text after anchor.
#[test]
fn google_docs_cli_e2e_insert_suggestion() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let doc_id = env::var("GOOGLE_DOCS_TEST_DOC_ID").unwrap_or_default();
    if doc_id.is_empty() {
        eprintln!("GOOGLE_DOCS_TEST_DOC_ID not set; skipping insert-suggestion test.");
        return;
    }

    // Read document to find anchor text
    let (success, content, _) = run_cli(&["read-document", &doc_id]);
    if !success || content.trim().is_empty() {
        eprintln!("Could not read document content; skipping insert-suggestion test.");
        return;
    }

    // Find first non-empty line as anchor
    let anchor_text = content
        .lines()
        .find(|line| !line.trim().is_empty() && line.len() > 3)
        .map(|s| s.trim())
        .unwrap_or("");

    if anchor_text.is_empty() {
        eprintln!("No suitable anchor text found in document.");
        return;
    }

    let anchor = &anchor_text[..anchor_text.len().min(15)];
    let suggestion_text = " [TEST SUGGESTION] ";

    let (success, stdout, stderr) = run_cli(&[
        "insert-suggestion",
        &doc_id,
        &format!("--after={}", anchor),
        &format!("--text={}", suggestion_text),
    ]);

    assert!(
        success,
        "insert-suggestion should succeed.\nstdout: {}\nstderr: {}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Successfully inserted") || stdout.contains("blue"),
        "Output should confirm suggestion insertion.\nstdout: {}",
        stdout
    );

    println!(
        "✓ insert-suggestion test passed: inserted '{}' after '{}'",
        suggestion_text, anchor
    );
}

/// Test suggest-replace: Replace text with revision marks.
#[test]
fn google_docs_cli_e2e_suggest_replace() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let doc_id = env::var("GOOGLE_DOCS_TEST_DOC_ID").unwrap_or_default();
    if doc_id.is_empty() {
        eprintln!("GOOGLE_DOCS_TEST_DOC_ID not set; skipping suggest-replace test.");
        return;
    }

    // Read document to find text to replace
    let (success, content, _) = run_cli(&["read-document", &doc_id]);
    if !success || content.trim().is_empty() {
        eprintln!("Could not read document content; skipping suggest-replace test.");
        return;
    }

    // Find a word to replace
    let old_text = content
        .split_whitespace()
        .find(|word| word.len() >= 3 && word.chars().all(|c| c.is_alphabetic()))
        .unwrap_or("");

    if old_text.is_empty() {
        eprintln!("No suitable word found in document for replacement.");
        return;
    }

    let new_text = format!("[REPLACED:{}]", old_text);

    let (success, stdout, stderr) = run_cli(&[
        "suggest-replace",
        &doc_id,
        &format!("--find={}", old_text),
        &format!("--replace={}", new_text),
    ]);

    assert!(
        success,
        "suggest-replace should succeed.\nstdout: {}\nstderr: {}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Successfully suggested") || stdout.contains("strikethrough"),
        "Output should confirm replacement suggestion.\nstdout: {}",
        stdout
    );

    println!(
        "✓ suggest-replace test passed: suggested replacing '{}' with '{}'",
        old_text, new_text
    );
}

/// Test apply-suggestions: Apply all pending suggestions.
#[test]
fn google_docs_cli_e2e_apply_suggestions() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let doc_id = env::var("GOOGLE_DOCS_TEST_DOC_ID").unwrap_or_default();
    if doc_id.is_empty() {
        eprintln!("GOOGLE_DOCS_TEST_DOC_ID not set; skipping apply-suggestions test.");
        return;
    }

    let (success, stdout, stderr) = run_cli(&["apply-suggestions", &doc_id]);

    assert!(
        success,
        "apply-suggestions should succeed.\nstdout: {}\nstderr: {}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Successfully applied") || stdout.contains("applied"),
        "Output should confirm suggestions were applied.\nstdout: {}",
        stdout
    );

    println!("✓ apply-suggestions test passed");
}

/// Test discard-suggestions: Discard all pending suggestions.
#[test]
fn google_docs_cli_e2e_discard_suggestions() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let doc_id = env::var("GOOGLE_DOCS_TEST_DOC_ID").unwrap_or_default();
    if doc_id.is_empty() {
        eprintln!("GOOGLE_DOCS_TEST_DOC_ID not set; skipping discard-suggestions test.");
        return;
    }

    let (success, stdout, stderr) = run_cli(&["discard-suggestions", &doc_id]);

    assert!(
        success,
        "discard-suggestions should succeed.\nstdout: {}\nstderr: {}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Successfully discarded") || stdout.contains("discarded"),
        "Output should confirm suggestions were discarded.\nstdout: {}",
        stdout
    );

    println!("✓ discard-suggestions test passed");
}

/// Full workflow test: insert suggestion, then apply it.
#[test]
fn google_docs_cli_e2e_full_suggestion_workflow() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let doc_id = env::var("GOOGLE_DOCS_TEST_DOC_ID").unwrap_or_default();
    if doc_id.is_empty() {
        eprintln!("GOOGLE_DOCS_TEST_DOC_ID not set; skipping full workflow test.");
        return;
    }

    println!("=== Full Suggestion Workflow Test ===");

    // Step 1: Read initial content
    let (success, initial_content, _) = run_cli(&["read-document", &doc_id]);
    assert!(success, "Should read initial document");
    println!(
        "1. Read initial document content ({} chars)",
        initial_content.len()
    );

    // Step 2: Find anchor and insert suggestion
    let anchor = initial_content
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|s| s.trim())
        .unwrap_or("Dear");
    let anchor_short = &anchor[..anchor.len().min(10)];
    let test_text = " [E2E_TEST_INSERTION] ";

    let (success, _, stderr) = run_cli(&[
        "insert-suggestion",
        &doc_id,
        &format!("--after={}", anchor_short),
        &format!("--text={}", test_text),
    ]);
    assert!(success, "insert-suggestion should succeed: {}", stderr);
    println!(
        "2. Inserted suggestion '{}' after '{}'",
        test_text, anchor_short
    );

    // Step 3: Read document - should now contain blue text
    let (success, content_with_suggestion, _) = run_cli(&["read-document", &doc_id]);
    assert!(success, "Should read document with suggestion");
    assert!(
        content_with_suggestion.contains("[E2E_TEST_INSERTION]"),
        "Document should contain the inserted text"
    );
    println!("3. Verified suggestion is in document");

    // Step 4: Apply suggestions
    let (success, _, stderr) = run_cli(&["apply-suggestions", &doc_id]);
    assert!(success, "apply-suggestions should succeed: {}", stderr);
    println!("4. Applied suggestions (blue text -> black)");

    // Step 5: Verify final content
    let (success, final_content, _) = run_cli(&["read-document", &doc_id]);
    assert!(success, "Should read final document");
    assert!(
        final_content.contains("[E2E_TEST_INSERTION]"),
        "Document should still contain the text after apply"
    );
    println!("5. Verified text persisted after apply");

    println!("=== Full Suggestion Workflow Test PASSED ===");
}

/// Direct edit test: apply-edit (suggest + apply in one step)
#[test]
fn google_docs_cli_e2e_apply_edit() {
    if !env_enabled("GOOGLE_DOCS_CLI_E2E") {
        eprintln!("GOOGLE_DOCS_CLI_E2E not set; skipping E2E test.");
        return;
    }

    let doc_id = env::var("GOOGLE_DOCS_TEST_DOC_ID").unwrap_or_default();
    if doc_id.is_empty() {
        eprintln!("GOOGLE_DOCS_TEST_DOC_ID not set; skipping apply-edit test.");
        return;
    }

    // Read document to find text to replace
    let (success, content, _) = run_cli(&["read-document", &doc_id]);
    if !success || content.trim().is_empty() {
        eprintln!("Could not read document content.");
        return;
    }

    // Find a word to replace
    let old_text = content
        .split_whitespace()
        .find(|word| word.len() >= 4 && word.chars().all(|c| c.is_alphabetic()))
        .unwrap_or("");

    if old_text.is_empty() {
        eprintln!("No suitable word found in document.");
        return;
    }

    let new_text = format!("[EDIT:{}]", old_text);

    let (success, stdout, stderr) = run_cli(&[
        "apply-edit",
        &doc_id,
        &format!("--find={}", old_text),
        &format!("--replace={}", new_text),
    ]);

    assert!(
        success,
        "apply-edit should succeed.\nstdout: {}\nstderr: {}",
        stdout, stderr
    );
    assert!(
        stdout.contains("Successfully replaced"),
        "Output should confirm replacement.\nstdout: {}",
        stdout
    );

    // Verify the change
    let (success, final_content, _) = run_cli(&["read-document", &doc_id]);
    assert!(success, "Should read final document");
    assert!(
        final_content.contains(&new_text),
        "Document should contain the replaced text '{}'\nContent: {}",
        new_text,
        final_content
    );

    println!(
        "✓ apply-edit test passed: replaced '{}' with '{}'",
        old_text, new_text
    );
}
