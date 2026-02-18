//! Artifact link extractor for multi-channel collaboration.
//!
//! This module provides a trait-based architecture for extracting artifact links
//! (Google Docs, GitHub PRs, Notion pages, etc.) from message text.
//!
//! # Adding a new artifact type
//!
//! 1. Create a new struct implementing `ArtifactExtractor`
//! 2. Add it to the `EXTRACTORS` list in `extract_all_artifacts()`

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// An artifact extracted from message text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedArtifact {
    /// Type of artifact (e.g., "google_docs", "github_pr", "notion")
    pub artifact_type: String,
    /// External ID of the artifact
    pub artifact_id: String,
    /// Full URL to the artifact
    pub url: String,
    /// Surrounding text context (for debugging/display)
    pub context_snippet: Option<String>,
}

/// Trait for artifact extractors.
pub trait ArtifactExtractor: Send + Sync {
    /// The type identifier for artifacts extracted by this extractor.
    fn artifact_type(&self) -> &'static str;

    /// Extract artifacts from text.
    fn extract(&self, text: &str) -> Vec<ExtractedArtifact>;
}

// =============================================================================
// Google Docs/Sheets/Slides Extractor
// =============================================================================

/// Extracts Google Workspace document links.
pub struct GoogleDocsExtractor;

static GOOGLE_DOCS_PATTERNS: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    vec![
        // Google Docs
        (
            Regex::new(r"https://docs\.google\.com/document/d/([a-zA-Z0-9_-]+)(?:/[^\s]*)?").unwrap(),
            "google_docs",
        ),
        // Google Sheets
        (
            Regex::new(r"https://docs\.google\.com/spreadsheets/d/([a-zA-Z0-9_-]+)(?:/[^\s]*)?").unwrap(),
            "google_sheets",
        ),
        // Google Slides
        (
            Regex::new(r"https://docs\.google\.com/presentation/d/([a-zA-Z0-9_-]+)(?:/[^\s]*)?").unwrap(),
            "google_slides",
        ),
        // Google Drive file (generic)
        (
            Regex::new(r"https://drive\.google\.com/file/d/([a-zA-Z0-9_-]+)(?:/[^\s]*)?").unwrap(),
            "google_drive",
        ),
        // Google Drive open link
        (
            Regex::new(r"https://drive\.google\.com/open\?id=([a-zA-Z0-9_-]+)").unwrap(),
            "google_drive",
        ),
    ]
});

impl ArtifactExtractor for GoogleDocsExtractor {
    fn artifact_type(&self) -> &'static str {
        "google_docs"
    }

    fn extract(&self, text: &str) -> Vec<ExtractedArtifact> {
        let mut artifacts = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for (pattern, doc_type) in GOOGLE_DOCS_PATTERNS.iter() {
            for cap in pattern.captures_iter(text) {
                if let (Some(full_match), Some(id_match)) = (cap.get(0), cap.get(1)) {
                    let artifact_id = id_match.as_str().to_string();

                    // Skip duplicates
                    if seen_ids.contains(&artifact_id) {
                        continue;
                    }
                    seen_ids.insert(artifact_id.clone());

                    let url = full_match.as_str().to_string();
                    let context = extract_context(text, full_match.start(), full_match.end(), 50);

                    artifacts.push(ExtractedArtifact {
                        artifact_type: doc_type.to_string(),
                        artifact_id,
                        url,
                        context_snippet: Some(context),
                    });
                }
            }
        }

        artifacts
    }
}

// =============================================================================
// GitHub Extractor
// =============================================================================

/// Extracts GitHub PR, Issue, and repository links.
pub struct GitHubExtractor;

// GitHub PR pattern
static GITHUB_PR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https://github\.com/([a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+)/pull/(\d+)").unwrap()
});

// GitHub Issue pattern
static GITHUB_ISSUE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https://github\.com/([a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+)/issues/(\d+)").unwrap()
});

// GitHub Repository pattern (only matches tree/blob paths, not PR/issues)
static GITHUB_REPO_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https://github\.com/([a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+)/(?:tree|blob)/[^\s]+").unwrap()
});

impl ArtifactExtractor for GitHubExtractor {
    fn artifact_type(&self) -> &'static str {
        "github"
    }

    fn extract(&self, text: &str) -> Vec<ExtractedArtifact> {
        let mut artifacts = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        // Extract PRs
        for cap in GITHUB_PR_PATTERN.captures_iter(text) {
            if let (Some(full_match), Some(repo), Some(num)) = (cap.get(0), cap.get(1), cap.get(2)) {
                let artifact_id = format!("{}#{}", repo.as_str(), num.as_str());
                if seen_ids.contains(&artifact_id) {
                    continue;
                }
                seen_ids.insert(artifact_id.clone());

                let url = full_match.as_str().to_string();
                let context = extract_context(text, full_match.start(), full_match.end(), 50);

                artifacts.push(ExtractedArtifact {
                    artifact_type: "github_pr".to_string(),
                    artifact_id,
                    url,
                    context_snippet: Some(context),
                });
            }
        }

        // Extract Issues
        for cap in GITHUB_ISSUE_PATTERN.captures_iter(text) {
            if let (Some(full_match), Some(repo), Some(num)) = (cap.get(0), cap.get(1), cap.get(2)) {
                let artifact_id = format!("{}#{}", repo.as_str(), num.as_str());
                if seen_ids.contains(&artifact_id) {
                    continue;
                }
                seen_ids.insert(artifact_id.clone());

                let url = full_match.as_str().to_string();
                let context = extract_context(text, full_match.start(), full_match.end(), 50);

                artifacts.push(ExtractedArtifact {
                    artifact_type: "github_issue".to_string(),
                    artifact_id,
                    url,
                    context_snippet: Some(context),
                });
            }
        }

        // Extract Repos (only tree/blob paths)
        for cap in GITHUB_REPO_PATTERN.captures_iter(text) {
            if let (Some(full_match), Some(repo)) = (cap.get(0), cap.get(1)) {
                let artifact_id = repo.as_str().to_string();
                if seen_ids.contains(&artifact_id) {
                    continue;
                }
                seen_ids.insert(artifact_id.clone());

                let url = full_match.as_str().to_string();
                let context = extract_context(text, full_match.start(), full_match.end(), 50);

                artifacts.push(ExtractedArtifact {
                    artifact_type: "github_repo".to_string(),
                    artifact_id,
                    url,
                    context_snippet: Some(context),
                });
            }
        }

        artifacts
    }
}

// =============================================================================
// Notion Extractor
// =============================================================================

/// Extracts Notion page links.
pub struct NotionExtractor;

static NOTION_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        // Notion page with ID in URL
        Regex::new(r"https://(?:www\.)?notion\.so/(?:[a-zA-Z0-9_-]+/)?([a-zA-Z0-9]+(?:-[a-f0-9]{32})?)").unwrap(),
        // Notion.site pages
        Regex::new(r"https://[a-zA-Z0-9_-]+\.notion\.site/([a-zA-Z0-9_-]+(?:-[a-f0-9]{32})?)").unwrap(),
    ]
});

impl ArtifactExtractor for NotionExtractor {
    fn artifact_type(&self) -> &'static str {
        "notion"
    }

    fn extract(&self, text: &str) -> Vec<ExtractedArtifact> {
        let mut artifacts = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for pattern in NOTION_PATTERNS.iter() {
            for cap in pattern.captures_iter(text) {
                if let (Some(full_match), Some(id_match)) = (cap.get(0), cap.get(1)) {
                    let artifact_id = id_match.as_str().to_string();

                    // Skip duplicates
                    if seen_ids.contains(&artifact_id) {
                        continue;
                    }
                    seen_ids.insert(artifact_id.clone());

                    let url = full_match.as_str().to_string();
                    let context = extract_context(text, full_match.start(), full_match.end(), 50);

                    artifacts.push(ExtractedArtifact {
                        artifact_type: "notion".to_string(),
                        artifact_id,
                        url,
                        context_snippet: Some(context),
                    });
                }
            }
        }

        artifacts
    }
}

// =============================================================================
// Linear Extractor
// =============================================================================

/// Extracts Linear issue links.
pub struct LinearExtractor;

static LINEAR_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        // Linear issue URL
        Regex::new(r"https://linear\.app/([a-zA-Z0-9_-]+)/issue/([A-Z]+-\d+)").unwrap(),
    ]
});

impl ArtifactExtractor for LinearExtractor {
    fn artifact_type(&self) -> &'static str {
        "linear"
    }

    fn extract(&self, text: &str) -> Vec<ExtractedArtifact> {
        let mut artifacts = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for pattern in LINEAR_PATTERNS.iter() {
            for cap in pattern.captures_iter(text) {
                if let (Some(full_match), Some(id_match)) = (cap.get(0), cap.get(2)) {
                    let artifact_id = id_match.as_str().to_string();

                    // Skip duplicates
                    if seen_ids.contains(&artifact_id) {
                        continue;
                    }
                    seen_ids.insert(artifact_id.clone());

                    let url = full_match.as_str().to_string();
                    let context = extract_context(text, full_match.start(), full_match.end(), 50);

                    artifacts.push(ExtractedArtifact {
                        artifact_type: "linear_issue".to_string(),
                        artifact_id,
                        url,
                        context_snippet: Some(context),
                    });
                }
            }
        }

        artifacts
    }
}

// =============================================================================
// Figma Extractor
// =============================================================================

/// Extracts Figma design links.
pub struct FigmaExtractor;

static FIGMA_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        // Figma file
        Regex::new(r"https://(?:www\.)?figma\.com/(?:file|design)/([a-zA-Z0-9]+)(?:/[^\s]*)?").unwrap(),
        // Figma prototype
        Regex::new(r"https://(?:www\.)?figma\.com/proto/([a-zA-Z0-9]+)(?:/[^\s]*)?").unwrap(),
    ]
});

impl ArtifactExtractor for FigmaExtractor {
    fn artifact_type(&self) -> &'static str {
        "figma"
    }

    fn extract(&self, text: &str) -> Vec<ExtractedArtifact> {
        let mut artifacts = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for pattern in FIGMA_PATTERNS.iter() {
            for cap in pattern.captures_iter(text) {
                if let (Some(full_match), Some(id_match)) = (cap.get(0), cap.get(1)) {
                    let artifact_id = id_match.as_str().to_string();

                    // Skip duplicates
                    if seen_ids.contains(&artifact_id) {
                        continue;
                    }
                    seen_ids.insert(artifact_id.clone());

                    let url = full_match.as_str().to_string();
                    let context = extract_context(text, full_match.start(), full_match.end(), 50);

                    artifacts.push(ExtractedArtifact {
                        artifact_type: "figma".to_string(),
                        artifact_id,
                        url,
                        context_snippet: Some(context),
                    });
                }
            }
        }

        artifacts
    }
}

// =============================================================================
// Jira Extractor
// =============================================================================

/// Extracts Jira issue links.
pub struct JiraExtractor;

static JIRA_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        // Jira Cloud
        Regex::new(r"https://([a-zA-Z0-9_-]+)\.atlassian\.net/browse/([A-Z]+-\d+)").unwrap(),
        // Jira Server/DC
        Regex::new(r"https://[^\s]+/browse/([A-Z]+-\d+)").unwrap(),
    ]
});

impl ArtifactExtractor for JiraExtractor {
    fn artifact_type(&self) -> &'static str {
        "jira"
    }

    fn extract(&self, text: &str) -> Vec<ExtractedArtifact> {
        let mut artifacts = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for pattern in JIRA_PATTERNS.iter() {
            for cap in pattern.captures_iter(text) {
                if let Some(full_match) = cap.get(0) {
                    // Get the issue key (last capture group)
                    let artifact_id = cap
                        .get(cap.len() - 1)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();

                    if artifact_id.is_empty() {
                        continue;
                    }

                    // Skip duplicates
                    if seen_ids.contains(&artifact_id) {
                        continue;
                    }
                    seen_ids.insert(artifact_id.clone());

                    let url = full_match.as_str().to_string();
                    let context = extract_context(text, full_match.start(), full_match.end(), 50);

                    artifacts.push(ExtractedArtifact {
                        artifact_type: "jira_issue".to_string(),
                        artifact_id,
                        url,
                        context_snippet: Some(context),
                    });
                }
            }
        }

        artifacts
    }
}

// =============================================================================
// Main extraction function
// =============================================================================

/// All registered artifact extractors.
static EXTRACTORS: LazyLock<Vec<Box<dyn ArtifactExtractor>>> = LazyLock::new(|| {
    vec![
        Box::new(GoogleDocsExtractor),
        Box::new(GitHubExtractor),
        Box::new(NotionExtractor),
        Box::new(LinearExtractor),
        Box::new(FigmaExtractor),
        Box::new(JiraExtractor),
    ]
});

/// Extract all artifacts from text using all registered extractors.
pub fn extract_all_artifacts(text: &str) -> Vec<ExtractedArtifact> {
    let mut all_artifacts = Vec::new();

    for extractor in EXTRACTORS.iter() {
        all_artifacts.extend(extractor.extract(text));
    }

    all_artifacts
}

/// Extract artifacts from both HTML body and text body.
pub fn extract_artifacts_from_email(
    html_body: Option<&str>,
    text_body: Option<&str>,
    subject: Option<&str>,
) -> Vec<ExtractedArtifact> {
    let mut all_artifacts = Vec::new();
    let mut seen_urls = std::collections::HashSet::new();

    // Extract from HTML body first (usually has more context)
    if let Some(html) = html_body {
        // Simple HTML link extraction - look for href attributes
        let href_pattern = Regex::new(r#"href="([^"]+)""#).unwrap();
        for cap in href_pattern.captures_iter(html) {
            if let Some(url) = cap.get(1) {
                let artifacts = extract_all_artifacts(url.as_str());
                for artifact in artifacts {
                    if !seen_urls.contains(&artifact.url) {
                        seen_urls.insert(artifact.url.clone());
                        all_artifacts.push(artifact);
                    }
                }
            }
        }

        // Also extract from plain text in HTML
        for artifact in extract_all_artifacts(html) {
            if !seen_urls.contains(&artifact.url) {
                seen_urls.insert(artifact.url.clone());
                all_artifacts.push(artifact);
            }
        }
    }

    // Extract from text body
    if let Some(text) = text_body {
        for artifact in extract_all_artifacts(text) {
            if !seen_urls.contains(&artifact.url) {
                seen_urls.insert(artifact.url.clone());
                all_artifacts.push(artifact);
            }
        }
    }

    // Extract from subject
    if let Some(subj) = subject {
        for artifact in extract_all_artifacts(subj) {
            if !seen_urls.contains(&artifact.url) {
                seen_urls.insert(artifact.url.clone());
                all_artifacts.push(artifact);
            }
        }
    }

    all_artifacts
}

/// Extract context around a match.
fn extract_context(text: &str, start: usize, end: usize, context_len: usize) -> String {
    // Calculate desired bounds
    let desired_start = start.saturating_sub(context_len);
    let desired_end = (end + context_len).min(text.len());

    // Find safe character boundaries
    let safe_start = if desired_start == 0 {
        0
    } else {
        // Find the nearest char boundary at or after desired_start
        text.char_indices()
            .find(|(i, _)| *i >= desired_start)
            .map(|(i, _)| i)
            .unwrap_or(start)
    };

    let safe_end = if desired_end >= text.len() {
        text.len()
    } else {
        // Find the nearest char boundary at or after desired_end
        text.char_indices()
            .find(|(i, _)| *i >= desired_end)
            .map(|(i, _)| i)
            .unwrap_or(text.len())
    };

    let mut context = String::new();
    if safe_start > 0 {
        context.push_str("...");
    }
    context.push_str(&text[safe_start..safe_end]);
    if safe_end < text.len() {
        context.push_str("...");
    }
    context
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_docs_extraction() {
        let text = "Please review this document: https://docs.google.com/document/d/1abc123xyz/edit";
        let artifacts = GoogleDocsExtractor.extract(text);

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].artifact_type, "google_docs");
        assert_eq!(artifacts[0].artifact_id, "1abc123xyz");
        assert!(artifacts[0].url.contains("docs.google.com"));
    }

    #[test]
    fn test_google_sheets_extraction() {
        let text = "Check out the spreadsheet: https://docs.google.com/spreadsheets/d/sheet123/edit#gid=0";
        let artifacts = GoogleDocsExtractor.extract(text);

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].artifact_type, "google_sheets");
        assert_eq!(artifacts[0].artifact_id, "sheet123");
    }

    #[test]
    fn test_github_pr_extraction() {
        let text = "Please review PR: https://github.com/owner/repo/pull/123";
        let artifacts = GitHubExtractor.extract(text);

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].artifact_type, "github_pr");
        assert_eq!(artifacts[0].artifact_id, "owner/repo#123");
    }

    #[test]
    fn test_github_issue_extraction() {
        let text = "Related issue: https://github.com/anthropics/claude-code/issues/456";
        let artifacts = GitHubExtractor.extract(text);

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].artifact_type, "github_issue");
        assert_eq!(artifacts[0].artifact_id, "anthropics/claude-code#456");
    }

    #[test]
    fn test_notion_extraction() {
        let text = "See the spec: https://notion.so/workspace/My-Page-abc123def456789";
        let artifacts = NotionExtractor.extract(text);

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].artifact_type, "notion");
    }

    #[test]
    fn test_figma_extraction() {
        let text = "Design: https://www.figma.com/file/abc123xyz/My-Design";
        let artifacts = FigmaExtractor.extract(text);

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].artifact_type, "figma");
        assert_eq!(artifacts[0].artifact_id, "abc123xyz");
    }

    #[test]
    fn test_linear_extraction() {
        let text = "Working on https://linear.app/myteam/issue/ENG-123";
        let artifacts = LinearExtractor.extract(text);

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].artifact_type, "linear_issue");
        assert_eq!(artifacts[0].artifact_id, "ENG-123");
    }

    #[test]
    fn test_jira_extraction() {
        let text = "Linked to https://mycompany.atlassian.net/browse/PROJ-456";
        let artifacts = JiraExtractor.extract(text);

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].artifact_type, "jira_issue");
        assert_eq!(artifacts[0].artifact_id, "PROJ-456");
    }

    #[test]
    fn test_multiple_artifacts() {
        let text = r#"
            Please review the doc: https://docs.google.com/document/d/doc123/edit
            and the related PR: https://github.com/owner/repo/pull/789
            Design mockup: https://figma.com/file/fig456/mockup
        "#;

        let artifacts = extract_all_artifacts(text);

        assert_eq!(artifacts.len(), 3);

        let types: Vec<&str> = artifacts.iter().map(|a| a.artifact_type.as_str()).collect();
        assert!(types.contains(&"google_docs"));
        assert!(types.contains(&"github_pr"));
        assert!(types.contains(&"figma"));
    }

    #[test]
    fn test_deduplication() {
        let text = r#"
            Check https://docs.google.com/document/d/same123/edit
            and also https://docs.google.com/document/d/same123/edit?tab=t.0
        "#;

        let artifacts = GoogleDocsExtractor.extract(text);

        // Should deduplicate by artifact_id
        assert_eq!(artifacts.len(), 1);
    }

    #[test]
    fn test_extract_from_email() {
        let html = r#"<a href="https://docs.google.com/document/d/doc123/edit">Link</a>"#;
        let text = "Plain text with https://github.com/owner/repo/pull/1";
        let subject = "Re: https://linear.app/team/issue/ENG-42";

        let artifacts = extract_artifacts_from_email(Some(html), Some(text), Some(subject));

        // Should extract: google_docs from HTML, github_pr from text, linear_issue from subject
        // Note: HTML extraction also finds the link in href and in plain text, but deduplication should handle it
        assert!(artifacts.len() >= 3);

        let types: Vec<&str> = artifacts.iter().map(|a| a.artifact_type.as_str()).collect();
        assert!(types.contains(&"google_docs"));
        assert!(types.contains(&"github_pr"));
        assert!(types.contains(&"linear_issue"));
    }

    #[test]
    fn test_context_extraction() {
        let text = "Before text https://example.com after text";
        // URL starts at position 12, ends at 31
        let context = extract_context(text, 12, 31, 10);

        // With context_len=10, we should get characters from position 2 to 41
        assert!(context.contains("text"));
        assert!(context.contains("https://example.com"));
        assert!(context.contains("after"));
    }
}
