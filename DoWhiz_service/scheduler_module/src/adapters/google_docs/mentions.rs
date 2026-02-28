use regex::Regex;
use std::sync::{LazyLock, OnceLock};
use tracing::warn;

/// Oliver (little_bear) patterns - require @ prefix or email format
static OLIVER_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i)@oliver\b").unwrap(),
        Regex::new(r"(?i)@little[_\s-]?bear\b").unwrap(),
        Regex::new(r"(?i)oliver@dowhiz\.com").unwrap(),
    ]
});

/// Maggie (mini_mouse) patterns - require @ prefix or email format
static MAGGIE_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i)@maggie\b").unwrap(),
        Regex::new(r"(?i)@mini[_\s-]?mouse\b").unwrap(),
        Regex::new(r"(?i)maggie@dowhiz\.com").unwrap(),
    ]
});

/// Proto / Boiled-Egg (boiled_egg) patterns - require @ prefix or email format
static PROTO_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i)@proto\b").unwrap(),
        Regex::new(r"(?i)@boiled[_\s-]?egg\b").unwrap(),
        Regex::new(r"(?i)proto@dowhiz\.com").unwrap(),
    ]
});

/// Devin / Sticky-Octopus (sticky_octopus) patterns - require @ prefix or email format
static DEVIN_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i)@devin\b").unwrap(),
        Regex::new(r"(?i)@sticky[_\s-]?octopus\b").unwrap(),
        Regex::new(r"(?i)devin@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)coder@dowhiz\.com").unwrap(),
    ]
});

/// All employee patterns (used when no filter is set)
static ALL_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    let mut patterns = Vec::new();
    patterns.extend(OLIVER_PATTERNS.iter().cloned());
    patterns.extend(MAGGIE_PATTERNS.iter().cloned());
    patterns.extend(PROTO_PATTERNS.iter().cloned());
    patterns.extend(DEVIN_PATTERNS.iter().cloned());
    patterns
});

/// Cache the employee mention filter from environment.
/// Set EMPLOYEE_MENTION_FILTER to the employee_id (e.g., "proto" for local, "little_bear" for production)
/// to only respond to mentions of that specific employee.
static EMPLOYEE_FILTER: OnceLock<Option<String>> = OnceLock::new();

fn get_employee_filter() -> Option<&'static String> {
    EMPLOYEE_FILTER
        .get_or_init(|| std::env::var("EMPLOYEE_MENTION_FILTER").ok())
        .as_ref()
}

/// Check if text contains an employee mention.
/// When EMPLOYEE_MENTION_FILTER is set, only checks patterns for that employee.
/// This allows local testing (proto) and production (oliver) to not interfere with each other.
pub fn contains_employee_mention(text: &str) -> bool {
    let filter = get_employee_filter();

    match filter.map(|s| s.as_str()) {
        Some("oliver") | Some("little_bear") => OLIVER_PATTERNS.iter().any(|p| p.is_match(text)),
        Some("proto") | Some("boiled_egg") => PROTO_PATTERNS.iter().any(|p| p.is_match(text)),
        Some("maggie") | Some("mini_mouse") => MAGGIE_PATTERNS.iter().any(|p| p.is_match(text)),
        Some("devin") | Some("sticky_octopus") => DEVIN_PATTERNS.iter().any(|p| p.is_match(text)),
        None => {
            // No filter - check all patterns (backwards compatible)
            ALL_PATTERNS.iter().any(|p| p.is_match(text))
        }
        Some(other) => {
            // Unknown filter - warn and check all
            warn!(
                "Unknown EMPLOYEE_MENTION_FILTER: {}, checking all patterns",
                other
            );
            ALL_PATTERNS.iter().any(|p| p.is_match(text))
        }
    }
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

/// Check if text matches Oliver patterns (for testing/direct use).
#[allow(dead_code)]
pub fn matches_oliver_patterns(text: &str) -> bool {
    OLIVER_PATTERNS.iter().any(|p| p.is_match(text))
}

/// Check if text matches Proto patterns (for testing/direct use).
#[allow(dead_code)]
pub fn matches_proto_patterns(text: &str) -> bool {
    PROTO_PATTERNS.iter().any(|p| p.is_match(text))
}

/// Check if text matches Maggie patterns (for testing/direct use).
#[allow(dead_code)]
pub fn matches_maggie_patterns(text: &str) -> bool {
    MAGGIE_PATTERNS.iter().any(|p| p.is_match(text))
}

/// Check if text matches Devin patterns (for testing/direct use).
#[allow(dead_code)]
pub fn matches_devin_patterns(text: &str) -> bool {
    DEVIN_PATTERNS.iter().any(|p| p.is_match(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: contains_employee_mention behavior depends on EMPLOYEE_MENTION_FILTER env var.
    // When not set (default in tests), it checks ALL patterns.
    // When set to "proto", it only checks Proto patterns, etc.

    #[test]
    fn test_employee_mention_detection() {
        // Explicit @ mentions should trigger (when no filter or filter matches)
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
    fn test_oliver_patterns() {
        assert!(matches_oliver_patterns("@Oliver please"));
        assert!(matches_oliver_patterns("@oliver can you"));
        assert!(matches_oliver_patterns("@little_bear help"));
        assert!(matches_oliver_patterns("@little-bear help"));
        assert!(matches_oliver_patterns("oliver@dowhiz.com"));

        // Should NOT match other employees
        assert!(!matches_oliver_patterns("@proto check"));
        assert!(!matches_oliver_patterns("@maggie please"));
        assert!(!matches_oliver_patterns("proto@dowhiz.com"));
    }

    #[test]
    fn test_proto_patterns() {
        assert!(matches_proto_patterns("@proto check"));
        assert!(matches_proto_patterns("@PROTO look"));
        assert!(matches_proto_patterns("@boiled-egg fix"));
        assert!(matches_proto_patterns("@boiled_egg fix"));
        assert!(matches_proto_patterns("proto@dowhiz.com"));

        // Should NOT match other employees
        assert!(!matches_proto_patterns("@oliver please"));
        assert!(!matches_proto_patterns("@maggie check"));
        assert!(!matches_proto_patterns("oliver@dowhiz.com"));
    }

    #[test]
    fn test_maggie_patterns() {
        assert!(matches_maggie_patterns("@maggie please"));
        assert!(matches_maggie_patterns("@MAGGIE help"));
        assert!(matches_maggie_patterns("@mini-mouse check"));
        assert!(matches_maggie_patterns("@mini_mouse check"));
        assert!(matches_maggie_patterns("maggie@dowhiz.com"));

        // Should NOT match other employees
        assert!(!matches_maggie_patterns("@oliver please"));
        assert!(!matches_maggie_patterns("@proto check"));
    }

    #[test]
    fn test_devin_patterns() {
        assert!(matches_devin_patterns("@devin help"));
        assert!(matches_devin_patterns("@DEVIN review"));
        assert!(matches_devin_patterns("@sticky-octopus fix"));
        assert!(matches_devin_patterns("@sticky_octopus fix"));
        assert!(matches_devin_patterns("devin@dowhiz.com"));
        assert!(matches_devin_patterns("coder@dowhiz.com"));

        // Should NOT match other employees
        assert!(!matches_devin_patterns("@oliver please"));
        assert!(!matches_devin_patterns("@proto check"));
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
