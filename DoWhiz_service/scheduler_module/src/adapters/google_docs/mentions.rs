use regex::Regex;
use std::sync::LazyLock;

/// Patterns to match explicit employee mentions in comments.
/// Only triggers on explicit @ mentions like @proto, @oliver, or email addresses.
/// This prevents false positives from signatures like "Boiled-Egg" without @.
static EMPLOYEE_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        // Oliver (little_bear) - require @ prefix or email format
        Regex::new(r"(?i)@oliver\b").unwrap(),
        Regex::new(r"(?i)@little[_\s-]?bear\b").unwrap(),
        Regex::new(r"(?i)oliver@dowhiz\.com").unwrap(),
        // Maggie (mini_mouse) - require @ prefix or email format
        Regex::new(r"(?i)@maggie\b").unwrap(),
        Regex::new(r"(?i)@mini[_\s-]?mouse\b").unwrap(),
        Regex::new(r"(?i)maggie@dowhiz\.com").unwrap(),
        // Proto / Boiled-Egg (boiled_egg) - require @ prefix or email format
        Regex::new(r"(?i)@proto\b").unwrap(),
        Regex::new(r"(?i)@boiled[_\s-]?egg\b").unwrap(),
        Regex::new(r"(?i)proto@dowhiz\.com").unwrap(),
        // Devin / Sticky-Octopus (sticky_octopus) - require @ prefix or email format
        Regex::new(r"(?i)@devin\b").unwrap(),
        Regex::new(r"(?i)@sticky[_\s-]?octopus\b").unwrap(),
        Regex::new(r"(?i)devin@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)coder@dowhiz\.com").unwrap(),
    ]
});

/// Check if text contains an employee mention.
pub fn contains_employee_mention(text: &str) -> bool {
    EMPLOYEE_PATTERNS
        .iter()
        .any(|pattern| pattern.is_match(text))
}

/// Extract the employee name from a mention.
/// Returns the canonical display name for the employee.
/// This function is called after `contains_employee_mention` returns true,
/// so we can use looser matching here to extract the name.
pub fn extract_employee_name(text: &str) -> Option<&'static str> {
    let text_lower = text.to_lowercase();
    // Oliver (little_bear)
    if text_lower.contains("@oliver")
        || text_lower.contains("oliver@dowhiz")
        || text_lower.contains("@little_bear")
        || text_lower.contains("@little-bear")
    {
        Some("Oliver")
    // Maggie (mini_mouse)
    } else if text_lower.contains("@maggie")
        || text_lower.contains("maggie@dowhiz")
        || text_lower.contains("@mini_mouse")
        || text_lower.contains("@mini-mouse")
    {
        Some("Maggie")
    // Proto / Boiled-Egg (boiled_egg) - for local testing
    } else if text_lower.contains("@proto")
        || text_lower.contains("proto@dowhiz")
        || text_lower.contains("@boiled_egg")
        || text_lower.contains("@boiled-egg")
    {
        Some("Proto")
    // Devin / Sticky-Octopus (sticky_octopus)
    } else if text_lower.contains("@devin")
        || text_lower.contains("devin@dowhiz")
        || text_lower.contains("coder@dowhiz")
        || text_lower.contains("@sticky_octopus")
        || text_lower.contains("@sticky-octopus")
    {
        Some("Devin")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employee_mention_detection() {
        // Explicit @ mentions should trigger
        assert!(contains_employee_mention("@Oliver please review"));
        assert!(contains_employee_mention("@oliver can you help?"));
        assert!(contains_employee_mention("@proto check this"));
        assert!(contains_employee_mention("@PROTO look at this"));
        assert!(contains_employee_mention("@maggie please check"));
        assert!(contains_employee_mention("@devin help me"));
        assert!(contains_employee_mention("@boiled-egg fix this"));
        assert!(contains_employee_mention("@little_bear help"));
        assert!(contains_employee_mention("@mini-mouse check"));
        assert!(contains_employee_mention("@sticky_octopus review"));

        // Email format should trigger
        assert!(contains_employee_mention("Contact oliver@dowhiz.com"));
        assert!(contains_employee_mention("proto@dowhiz.com please help"));
        assert!(contains_employee_mention("maggie@dowhiz.com"));
        assert!(contains_employee_mention("devin@dowhiz.com"));
        assert!(contains_employee_mention("coder@dowhiz.com"));

        // Names without @ should NOT trigger (prevents signature false positives)
        assert!(!contains_employee_mention("Hey oliver can you help?"));
        assert!(!contains_employee_mention("Oliver, check this"));
        assert!(!contains_employee_mention("little_bear please fix"));
        assert!(!contains_employee_mention("Boiled-Egg"));
        assert!(!contains_employee_mention("Go eggs! Boiled-Egg"));
        assert!(!contains_employee_mention("proto"));
        assert!(!contains_employee_mention("maggie said hello"));

        // Unrelated text should NOT trigger
        assert!(!contains_employee_mention("Hey John can you help?"));
        assert!(!contains_employee_mention("This is a regular comment"));
    }

    #[test]
    fn test_extract_employee_name() {
        // extract_employee_name is used after detection, so it can be looser
        assert_eq!(extract_employee_name("@Oliver please"), Some("Oliver"));
        assert_eq!(extract_employee_name("oliver@dowhiz.com"), Some("Oliver"));
        assert_eq!(extract_employee_name("@proto help"), Some("Proto"));
        assert_eq!(extract_employee_name("@maggie check"), Some("Maggie"));
        assert_eq!(extract_employee_name("@devin review"), Some("Devin"));
        assert_eq!(extract_employee_name("John help"), None);
    }
}
