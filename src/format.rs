extern crate regex;

/// Wrap URLs in text with HTML A tags.
///
/// # Examples
///
/// ```
///   use format::autolink;
///
///   let before = "Share code on https://crates.io";
///   let after = "Share code on <a href=\"https://crates.io\">https://crates.io</a>";
///   assert_eq!(autolink(before), after)
/// ```
pub fn autolink(text: &str) -> String {
    if text.len() == 0 {
        return String::new();
    }

    let url_pattern = regex::Regex::new(
        r"(?ix)
        (?: ((?:http|https):)//)
        [^\s<\x{00A0}\x{0022}]+",
    )
    .unwrap();

    let email_pattern = regex::Regex::new(
        r"(?ix)
        ([\w\.\d\-]+@[\w\.\d\-]+\.\w+)",
    )
    .unwrap();

    let text = url_pattern.replace_all(&text, "<a href=\"$0\">$0</a>");
    let text = email_pattern.replace_all(&text, "<a href=\"mailto:$0\">$0</a>");

    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!("", autolink(""))
    }

    #[test]
    fn test_string_with_http_urls() {
        let src = "Check this out: https://doc.rust-lang.org/\n
               http://fr.wikipedia.org/wiki/Caf%C3%A9ine";
        let linked = "Check this out: <a href=\"https://doc.rust-lang.org/\">https://doc.rust-lang.org/</a>\n
               <a href=\"http://fr.wikipedia.org/wiki/Caf%C3%A9ine\">http://fr.wikipedia.org/wiki/Caf%C3%A9ine</a>";
        assert_eq!(linked, autolink(src))
    }

    #[test]
    fn test_string_with_email_addresses() {
        let src = "Send spam to jack@standard-library.com";
        assert_eq!(
            "Send spam to <a href=\"mailto:jack@standard-library.com\">jack@standard-library.com</a>",
            autolink(src)
        )
    }
}
