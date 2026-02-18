/// Format a proposal for document edit as a comment reply.
pub fn format_edit_proposal(
    original_text: &str,
    proposed_text: &str,
    explanation: Option<&str>,
) -> String {
    let mut reply = String::new();

    reply.push_str("Here's my suggested edit:\n\n");
    reply.push_str("**Original:**\n");
    reply.push_str(&format!("\"{}\"", original_text));
    reply.push_str("\n\n");
    reply.push_str("**Suggested:**\n");
    reply.push_str(&format!("\"{}\"", proposed_text));

    if let Some(exp) = explanation {
        reply.push_str("\n\n");
        reply.push_str("**Reason:** ");
        reply.push_str(exp);
    }

    reply.push_str(
        "\n\nReply \"apply\" to confirm this edit, or let me know if you'd like any changes.",
    );

    reply
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_edit_proposal() {
        let proposal = format_edit_proposal(
            "The quick brown fox",
            "The swift brown fox",
            Some("'Swift' is more descriptive"),
        );

        assert!(proposal.contains("**Original:**"));
        assert!(proposal.contains("The quick brown fox"));
        assert!(proposal.contains("**Suggested:**"));
        assert!(proposal.contains("The swift brown fox"));
        assert!(proposal.contains("**Reason:**"));
        assert!(proposal.contains("apply"));
    }
}
