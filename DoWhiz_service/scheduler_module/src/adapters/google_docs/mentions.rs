use regex::Regex;
use std::sync::LazyLock;

/// Patterns to match employee mentions in comments.
/// Supports all DoWhiz employees: oliver, maggie, proto, devin, etc.
static EMPLOYEE_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        // Oliver (little_bear)
        Regex::new(r"(?i)\b@?oliver\b").unwrap(),
        Regex::new(r"(?i)oliver@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)\blittle[_\s-]?bear\b").unwrap(),
        // Maggie (mini_mouse)
        Regex::new(r"(?i)\b@?maggie\b").unwrap(),
        Regex::new(r"(?i)maggie@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)\bmini[_\s-]?mouse\b").unwrap(),
        // Proto / Boiled-Egg (boiled_egg) - for local testing
        Regex::new(r"(?i)\b@?proto\b").unwrap(),
        Regex::new(r"(?i)proto@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)\bboiled[_\s-]?egg\b").unwrap(),
        // Devin / Sticky-Octopus (sticky_octopus)
        Regex::new(r"(?i)\b@?devin\b").unwrap(),
        Regex::new(r"(?i)devin@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)\bsticky[_\s-]?octopus\b").unwrap(),
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
pub fn extract_employee_name(text: &str) -> Option<&'static str> {
    let text_lower = text.to_lowercase();
    // Oliver (little_bear)
    if text_lower.contains("oliver")
        || text_lower.contains("little_bear")
        || text_lower.contains("little bear")
        || text_lower.contains("little-bear")
    {
        Some("Oliver")
    // Maggie (mini_mouse)
    } else if text_lower.contains("maggie")
        || text_lower.contains("mini_mouse")
        || text_lower.contains("mini mouse")
        || text_lower.contains("mini-mouse")
    {
        Some("Maggie")
    // Proto / Boiled-Egg (boiled_egg) - for local testing
    } else if text_lower.contains("proto")
        || text_lower.contains("boiled_egg")
        || text_lower.contains("boiled egg")
        || text_lower.contains("boiled-egg")
    {
        Some("Proto")
    // Devin / Sticky-Octopus (sticky_octopus)
    } else if text_lower.contains("devin")
        || text_lower.contains("sticky_octopus")
        || text_lower.contains("sticky octopus")
        || text_lower.contains("sticky-octopus")
        || text_lower.contains("coder")
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
        assert!(contains_employee_mention("Hey oliver can you help?"));
        assert!(contains_employee_mention("@Oliver please review"));
        assert!(contains_employee_mention("Oliver, check this"));
        assert!(contains_employee_mention("Contact oliver@dowhiz.com"));
        assert!(contains_employee_mention("little_bear please fix"));
        assert!(contains_employee_mention("little bear help me"));
        assert!(contains_employee_mention("OLIVER look at this"));

        assert!(contains_employee_mention("Hey maggie can you help?"));
        assert!(contains_employee_mention("mini_mouse please check"));

        assert!(!contains_employee_mention("Hey John can you help?"));
        assert!(!contains_employee_mention("This is a regular comment"));
    }

    #[test]
    fn test_extract_employee_name() {
        assert_eq!(extract_employee_name("Hey oliver"), Some("Oliver"));
        assert_eq!(extract_employee_name("@Oliver please"), Some("Oliver"));
        assert_eq!(extract_employee_name("little_bear help"), Some("Oliver"));
        assert_eq!(extract_employee_name("maggie check"), Some("Maggie"));
        assert_eq!(extract_employee_name("mini mouse help"), Some("Maggie"));
        assert_eq!(extract_employee_name("John help"), None);
    }
}
