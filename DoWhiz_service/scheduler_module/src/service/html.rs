use kuchiki::traits::*;
use kuchiki::NodeRef;

use super::postmark::PostmarkInbound;

pub(super) fn render_email_html(payload: &PostmarkInbound) -> String {
    if let Some(html) = payload
        .html_body
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        let cleaned = clean_inbound_html(html);
        if !cleaned.trim().is_empty() {
            return cleaned;
        }
    }

    let text_body = payload
        .text_body
        .as_deref()
        .or(payload.stripped_text_reply.as_deref())
        .unwrap_or("");
    if text_body.trim().is_empty() {
        return "<pre>(no content)</pre>".to_string();
    }
    wrap_text_as_html(text_body)
}

fn clean_inbound_html(html: &str) -> String {
    let document = kuchiki::parse_html().one(html);
    remove_html_comments(&document);
    remove_elements_by_selector(
        &document,
        "head, script, style, meta, link, title, noscript",
    );
    remove_hidden_elements(&document);
    remove_tracking_pixels(&document);
    remove_footer_blocks(&document);
    sanitize_allowed_elements(&document);
    extract_body_html(&document)
}

fn remove_html_comments(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        if node.as_comment().is_some() {
            node.detach();
        }
    }
}

fn remove_elements_by_selector(document: &NodeRef, selector: &str) {
    if let Ok(nodes) = document.select(selector) {
        for node in nodes {
            node.as_node().detach();
        }
    }
}

fn remove_hidden_elements(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        let element = match node.as_element() {
            Some(value) => value,
            None => continue,
        };
        if is_hidden_element(element) {
            node.detach();
        }
    }
}

fn remove_tracking_pixels(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        let element = match node.as_element() {
            Some(value) => value,
            None => continue,
        };
        if element.name.local.as_ref() == "img" && is_tracking_pixel(element) {
            node.detach();
        }
    }
}

fn remove_footer_blocks(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        let element = match node.as_element() {
            Some(value) => value,
            None => continue,
        };
        let tag = element.name.local.as_ref();
        if !is_footer_candidate(tag) {
            continue;
        }
        if element_has_footer_marker(element) {
            node.detach();
            continue;
        }
        let text = node.text_contents();
        if text_contains_footer_hint(&text) {
            node.detach();
        }
    }
}

fn sanitize_allowed_elements(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        let element = match node.as_element() {
            Some(value) => value,
            None => continue,
        };
        let tag = element.name.local.as_ref();
        if is_drop_tag(tag) {
            node.detach();
            continue;
        }
        if !is_allowed_tag(tag) {
            unwrap_node(&node);
            continue;
        }
        prune_attributes(tag, element);
    }
}

fn extract_body_html(document: &NodeRef) -> String {
    if let Ok(mut bodies) = document.select("body") {
        if let Some(body) = bodies.next() {
            let mut out = String::new();
            for child in body.as_node().children() {
                out.push_str(&child.to_string());
            }
            return out;
        }
    }
    document.to_string()
}

fn unwrap_node(node: &NodeRef) {
    if node.parent().is_none() {
        return;
    }
    let children: Vec<NodeRef> = node.children().collect();
    for child in children {
        node.insert_before(child);
    }
    node.detach();
}

fn is_allowed_tag(tag: &str) -> bool {
    matches!(
        tag,
        "html"
            | "body"
            | "p"
            | "br"
            | "div"
            | "span"
            | "a"
            | "img"
            | "ul"
            | "ol"
            | "li"
            | "strong"
            | "em"
            | "b"
            | "i"
            | "u"
            | "blockquote"
            | "pre"
            | "code"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "table"
            | "thead"
            | "tbody"
            | "tr"
            | "td"
            | "th"
    )
}

fn is_drop_tag(tag: &str) -> bool {
    matches!(
        tag,
        "script" | "style" | "head" | "meta" | "link" | "title" | "noscript"
    )
}

fn is_footer_candidate(tag: &str) -> bool {
    matches!(
        tag,
        "div" | "p" | "span" | "td" | "li" | "section" | "footer"
    )
}

fn element_has_footer_marker(element: &kuchiki::ElementData) -> bool {
    let attrs = element.attributes.borrow();
    for key in ["class", "id"] {
        if let Some(value) = attrs.get(key) {
            let lower = value.to_ascii_lowercase();
            if lower.contains("footer")
                || lower.contains("unsubscribe")
                || lower.contains("notification")
                || lower.contains("preferences")
            {
                return true;
            }
        }
    }
    false
}

fn text_contains_footer_hint(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let hints = [
        "unsubscribe",
        "notification settings",
        "manage notifications",
        "email preferences",
        "manage your email",
        "view this email in your browser",
        "view in browser",
        "you are receiving this",
        "to stop receiving",
        "opt out",
        "reply to this email directly",
    ];
    hints.iter().any(|hint| lower.contains(hint))
}

fn is_hidden_element(element: &kuchiki::ElementData) -> bool {
    let attrs = element.attributes.borrow();
    if attrs.contains("hidden") {
        return true;
    }
    if let Some(value) = attrs.get("aria-hidden") {
        if value.trim().eq_ignore_ascii_case("true") {
            return true;
        }
    }
    if let Some(style) = attrs.get("style") {
        if style_contains_hidden(style) {
            return true;
        }
    }
    false
}

fn is_tracking_pixel(element: &kuchiki::ElementData) -> bool {
    let attrs = element.attributes.borrow();
    if let Some(style) = attrs.get("style") {
        if style_contains_hidden(style) {
            return true;
        }
    }
    let src = attrs.get("src").unwrap_or("");
    let src_lower = src.to_ascii_lowercase();
    if src_lower.contains("tracking")
        || src_lower.contains("pixel")
        || src_lower.contains("beacon")
        || src_lower.contains("open.gif")
    {
        return true;
    }
    let width = attrs.get("width").and_then(parse_dimension).or_else(|| {
        attrs
            .get("style")
            .and_then(|style| style_dimension(style, "width"))
    });
    let height = attrs.get("height").and_then(parse_dimension).or_else(|| {
        attrs
            .get("style")
            .and_then(|style| style_dimension(style, "height"))
    });
    matches_1x1(width, height)
}

fn matches_1x1(width: Option<u32>, height: Option<u32>) -> bool {
    match (width, height) {
        (Some(w), Some(h)) => w <= 1 && h <= 1,
        (Some(w), None) => w <= 1,
        (None, Some(h)) => h <= 1,
        (None, None) => false,
    }
}

fn style_contains_hidden(style: &str) -> bool {
    let normalized: String = style
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect();
    normalized.contains("display:none")
        || normalized.contains("visibility:hidden")
        || normalized.contains("opacity:0")
        || normalized.contains("max-height:0")
}

fn style_dimension(style: &str, key: &str) -> Option<u32> {
    for part in style.split(';') {
        let mut iter = part.splitn(2, ':');
        let name = iter.next().unwrap_or("").trim().to_ascii_lowercase();
        if name == key {
            let value = iter.next().unwrap_or("").trim();
            return parse_dimension(value);
        }
    }
    None
}

fn parse_dimension(raw: &str) -> Option<u32> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let digits: String = trimmed
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

fn prune_attributes(tag: &str, element: &kuchiki::ElementData) {
    let mut attrs = element.attributes.borrow_mut();
    let mut to_remove = Vec::new();
    for (name, _) in attrs.map.iter() {
        let local = name.local.as_ref();
        let keep = match tag {
            "a" => matches!(local, "href"),
            "img" => matches!(local, "src" | "alt" | "width" | "height"),
            _ => false,
        };
        if !keep {
            to_remove.push(name.clone());
        }
    }
    for name in to_remove {
        attrs.map.remove(&name);
    }
    if tag == "a" {
        if let Some(href) = attrs.get("href").map(|value| value.to_string()) {
            if !is_safe_link(&href) {
                attrs.remove("href");
            }
        }
    }
    if tag == "img" {
        if let Some(src) = attrs.get("src").map(|value| value.to_string()) {
            if !is_safe_image_src(&src) {
                attrs.remove("src");
            }
        }
    }
}

fn is_safe_link(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !(lower.starts_with("javascript:") || lower.starts_with("vbscript:"))
}

fn is_safe_image_src(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !(lower.starts_with("javascript:") || lower.starts_with("vbscript:"))
}

fn wrap_text_as_html(input: &str) -> String {
    format!("<pre>{}</pre>", escape_html(input))
}

fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

pub(super) fn strip_html_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

pub(super) fn truncate_preview(input: &str, max_len: usize) -> String {
    if input.len() <= max_len {
        return input.to_string();
    }
    let mut end = max_len;
    while end > 0 && !input.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &input[..end])
}
