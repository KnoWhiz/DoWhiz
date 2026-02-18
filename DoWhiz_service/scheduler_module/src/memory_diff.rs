use std::collections::{HashMap, HashSet};

/// Represents changes made to a memo.md file
#[derive(Debug, Clone, Default)]
pub struct MemoryDiff {
    /// Sections that were added or modified, keyed by section name
    pub changed_sections: HashMap<String, SectionChange>,
}

/// Represents a change to a specific section
#[derive(Debug, Clone)]
pub enum SectionChange {
    /// New lines added to an existing section
    Added(Vec<String>),
    /// Section content was replaced entirely (for major rewrites)
    Replaced(String),
    /// A new section that didn't exist before
    NewSection(String),
}

impl MemoryDiff {
    pub fn is_empty(&self) -> bool {
        self.changed_sections.is_empty()
    }
}

/// Compute the diff between original memo.md content and modified content
pub fn compute_memory_diff(original: &str, modified: &str) -> MemoryDiff {
    let original_sections = parse_sections(original);
    let modified_sections = parse_sections(modified);

    let mut changed_sections = HashMap::new();

    // Handle case where neither has sections (raw content without ## headers)
    if original_sections.is_empty() && modified_sections.is_empty() {
        if original.trim() != modified.trim() {
            let added = find_added_lines(original, modified);
            if !added.is_empty() {
                // Use a special "__raw__" key for non-sectioned content
                changed_sections.insert("__raw__".to_string(), SectionChange::Added(added));
            } else {
                // Content differs but no line additions - treat as replacement
                changed_sections.insert(
                    "__raw__".to_string(),
                    SectionChange::Replaced(modified.to_string()),
                );
            }
        }
        return MemoryDiff { changed_sections };
    }

    for (section_name, modified_content) in &modified_sections {
        match original_sections.get(section_name) {
            None => {
                // New section that didn't exist before
                if !modified_content.trim().is_empty() {
                    changed_sections.insert(
                        section_name.clone(),
                        SectionChange::NewSection(modified_content.clone()),
                    );
                }
            }
            Some(original_content) => {
                if original_content.trim() != modified_content.trim() {
                    // Section was modified - find what was added
                    let added = find_added_lines(original_content, modified_content);
                    if !added.is_empty() {
                        changed_sections.insert(section_name.clone(), SectionChange::Added(added));
                    } else if original_content.trim() != modified_content.trim() {
                        // Content changed but no clear line additions - treat as replace
                        changed_sections.insert(
                            section_name.clone(),
                            SectionChange::Replaced(modified_content.clone()),
                        );
                    }
                }
            }
        }
    }

    MemoryDiff { changed_sections }
}

/// Parse memo.md content into sections: "## SectionName" -> content
fn parse_sections(content: &str) -> HashMap<String, String> {
    let mut sections = HashMap::new();
    let mut current_section = String::new();
    let mut current_content = Vec::new();

    for line in content.lines() {
        if line.starts_with("## ") {
            // Save previous section
            if !current_section.is_empty() {
                sections.insert(current_section.clone(), current_content.join("\n"));
            }
            current_section = line[3..].trim().to_string();
            current_content = Vec::new();
        } else if line.starts_with("# ") {
            // Top-level header (# Memo), skip but reset section
            if !current_section.is_empty() {
                sections.insert(current_section.clone(), current_content.join("\n"));
            }
            current_section = String::new();
            current_content = Vec::new();
        } else if !current_section.is_empty() {
            current_content.push(line.to_string());
        }
    }

    // Save last section
    if !current_section.is_empty() {
        sections.insert(current_section, current_content.join("\n"));
    }

    sections
}

/// Find lines in modified that aren't in original
fn find_added_lines(original: &str, modified: &str) -> Vec<String> {
    let original_lines: HashSet<&str> = original
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    modified
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !original_lines.contains(trimmed)
        })
        .map(|s| s.to_string())
        .collect()
}

/// Apply a diff to existing memo.md content, returning the merged result
pub fn apply_memory_diff(current_content: &str, diff: &MemoryDiff) -> String {
    if diff.is_empty() {
        return current_content.to_string();
    }

    // Handle raw (non-sectioned) content
    if let Some(raw_change) = diff.changed_sections.get("__raw__") {
        return match raw_change {
            SectionChange::Added(lines) => {
                let mut result = current_content.to_string();
                for line in lines {
                    // Avoid duplicates
                    if !result.lines().any(|l| l.trim() == line.trim()) {
                        if !result.is_empty() && !result.ends_with('\n') {
                            result.push('\n');
                        }
                        result.push_str(line);
                    }
                }
                result
            }
            SectionChange::Replaced(new_content) => new_content.clone(),
            SectionChange::NewSection(content) => {
                let mut result = current_content.to_string();
                if !result.is_empty() && !result.ends_with('\n') {
                    result.push('\n');
                }
                result.push_str(content);
                result
            }
        };
    }

    let mut sections = parse_sections_ordered(current_content);

    for (section_name, change) in &diff.changed_sections {
        match change {
            SectionChange::Added(lines) => {
                // Append new lines to existing section
                if let Some(section) = sections.iter_mut().find(|(name, _)| name == section_name) {
                    let mut content = section.1.clone();
                    for line in lines {
                        // Avoid duplicates
                        if !content.lines().any(|l| l.trim() == line.trim()) {
                            if !content.is_empty() && !content.ends_with('\n') {
                                content.push('\n');
                            }
                            content.push_str(line);
                        }
                    }
                    section.1 = content;
                } else {
                    // Section doesn't exist yet, create it
                    sections.push((section_name.clone(), lines.join("\n")));
                }
            }
            SectionChange::Replaced(new_content) => {
                // Replace entire section content
                if let Some(section) = sections.iter_mut().find(|(name, _)| name == section_name) {
                    section.1 = new_content.clone();
                } else {
                    sections.push((section_name.clone(), new_content.clone()));
                }
            }
            SectionChange::NewSection(content) => {
                // Add new section if it doesn't exist
                if !sections.iter().any(|(name, _)| name == section_name) {
                    sections.push((section_name.clone(), content.clone()));
                }
            }
        }
    }

    // Rebuild memo.md from sections
    rebuild_memo(sections)
}

/// Parse sections while preserving order
fn parse_sections_ordered(content: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_section = String::new();
    let mut current_content = Vec::new();

    for line in content.lines() {
        if line.starts_with("## ") {
            if !current_section.is_empty() {
                sections.push((current_section.clone(), current_content.join("\n")));
            }
            current_section = line[3..].trim().to_string();
            current_content = Vec::new();
        } else if line.starts_with("# ") {
            if !current_section.is_empty() {
                sections.push((current_section.clone(), current_content.join("\n")));
            }
            current_section = String::new();
            current_content = Vec::new();
        } else if !current_section.is_empty() {
            current_content.push(line.to_string());
        }
    }

    if !current_section.is_empty() {
        sections.push((current_section, current_content.join("\n")));
    }

    sections
}

/// Rebuild memo.md from ordered sections
fn rebuild_memo(sections: Vec<(String, String)>) -> String {
    let mut result = String::from("# Memo\n\n");

    for (name, content) in sections {
        result.push_str(&format!("## {}\n", name));
        if !content.trim().is_empty() {
            result.push_str(&content);
            if !content.ends_with('\n') {
                result.push('\n');
            }
        }
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_diff_added_lines() {
        let original = r#"# Memo

## Contacts
- Alice: 555-0000

## Preferences
"#;
        let modified = r#"# Memo

## Contacts
- Alice: 555-0000
- Bob: 555-1234

## Preferences
"#;

        let diff = compute_memory_diff(original, modified);
        assert!(!diff.is_empty());
        assert!(diff.changed_sections.contains_key("Contacts"));

        if let Some(SectionChange::Added(lines)) = diff.changed_sections.get("Contacts") {
            assert!(lines.iter().any(|l| l.contains("Bob")));
        } else {
            panic!("Expected Added change");
        }
    }

    #[test]
    fn test_compute_diff_new_section() {
        let original = r#"# Memo

## Profile

## Contacts
"#;
        let modified = r#"# Memo

## Profile

## Contacts

## Projects
- Project Alpha
"#;

        let diff = compute_memory_diff(original, modified);
        assert!(diff.changed_sections.contains_key("Projects"));

        if let Some(SectionChange::NewSection(content)) = diff.changed_sections.get("Projects") {
            assert!(content.contains("Project Alpha"));
        } else {
            panic!("Expected NewSection change");
        }
    }

    #[test]
    fn test_apply_diff_adds_lines() {
        let current = r#"# Memo

## Contacts
- Alice: 555-0000

## Preferences
"#;

        let diff = MemoryDiff {
            changed_sections: HashMap::from([(
                "Contacts".to_string(),
                SectionChange::Added(vec!["- Bob: 555-1234".to_string()]),
            )]),
        };

        let result = apply_memory_diff(current, &diff);
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_apply_diff_no_duplicates() {
        let current = r#"# Memo

## Contacts
- Alice: 555-0000

"#;

        let diff = MemoryDiff {
            changed_sections: HashMap::from([(
                "Contacts".to_string(),
                SectionChange::Added(vec!["- Alice: 555-0000".to_string()]),
            )]),
        };

        let result = apply_memory_diff(current, &diff);
        // Should only have one Alice entry
        assert_eq!(result.matches("Alice").count(), 1);
    }

    #[test]
    fn test_empty_diff() {
        let content = r#"# Memo

## Contacts
- Alice: 555-0000
"#;

        let diff = compute_memory_diff(content, content);
        assert!(diff.is_empty());
    }
}
