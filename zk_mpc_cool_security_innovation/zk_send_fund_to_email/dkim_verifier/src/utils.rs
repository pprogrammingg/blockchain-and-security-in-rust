use mailparse::{MailHeaderMap, parse_mail};
use once_cell::sync::Lazy;
use regex::Regex;

/// Matches one or more whitespace chars (space, tab, newline)
pub static WHITESPACE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+").expect("Failed to compile WHITESPACE_RE"));

/// Collapse all consecutive whitespace into a single space.
/// Used for relaxed canonicalization.
pub fn collapse_whitespace(input: &str) -> String {
    WHITESPACE_RE.replace_all(input, " ").to_string()
}

/// Canonicalize email body according to DKIM canonicalization rules.
///
/// - "relaxed": trim trailing whitespace on lines, convert tabs → spaces,
///   remove trailing empty lines, join with CRLF.
/// - "simple": return body as-is.
pub fn canonicalize_body(body: &str, canon_type: &str) -> String {
    match canon_type {
        "relaxed" => {
            let mut lines: Vec<String> = body
                .lines()
                .map(|l| l.trim_end().replace('\t', " "))
                .collect();

            while matches!(lines.last(), Some(l) if l.is_empty()) {
                lines.pop();
            }

            lines.join("\r\n")
        }
        _ => body.to_string(),
    }
}

/// Canonicalize a single header value according to DKIM "relaxed" rules.
pub fn canonicalize_header_value(value: &str) -> String {
    collapse_whitespace(value.trim())
}

/// Canonicalize multiple headers from a raw email for signing/verifying.
///
/// - `raw_email` = full email string
/// - `header_list` = headers to include, in order
/// - `canon_type` = "relaxed" or "simple"
pub fn canonicalize_headers(
    raw_email: &str,
    header_list: &[&str],
    canon_type: &str,
) -> Result<String, String> {
    let parsed = parse_mail(raw_email.as_bytes()).map_err(|e| e.to_string())?;
    let headers = parsed.get_headers();

    let mut result = String::new();

    for &header_name in header_list {
        if let Some(h) = headers.get_first_header(header_name) {
            let val = match canon_type {
                "relaxed" => canonicalize_header_value(&h.get_value()),
                _ => h.get_value(),
            };
            result.push_str(&format!("{}:{}\r\n", header_name.to_lowercase(), val));
        } else {
            return Err(format!("Missing header: {}", header_name));
        }
    }

    Ok(result)
}

/// Extract body from raw email
pub fn extract_body(raw_email: &str) -> Result<String, String> {
    let parsed = parse_mail(raw_email.as_bytes()).map_err(|e| e.to_string())?;
    parsed.get_body().map_err(|e| e.to_string())
}
