use regex::Regex;
use std::borrow::Cow;

/// `convert_html_link` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert HTML link tags to markdown syntax in a given string.
/// It specifically targets the following pattern:
/// - HTML link tags: `<a href="URL">Text</a>`
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which link tags should be converted.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all link tags converted to markdown syntax.
///
pub fn convert_html_link(value: &str) -> Cow<str> {
    let re = Regex::new(r#"<a\s+href="([^"]+)">([^<]+)</a>"#).unwrap();
    let value = re.replace_all(value, "[$2]($1)").to_string();
    Cow::Owned(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_tag_converted() {
        let result = convert_html_link("<a href=\"https://example.com\">Example</a>");
        assert_eq!(result, "[Example](https://example.com)");
    }

    #[test]
    fn multiple_link_tags_converted() {
        let result = convert_html_link("<a href=\"https://example1.com\">Example1</a><a href=\"https://example2.com\">Example2</a>");
        assert_eq!(result, "[Example1](https://example1.com)[Example2](https://example2.com)");
    }

    #[test]
    fn no_tags_unchanged() {
        let result = convert_html_link("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_html_link("");
        assert_eq!(result, "");
    }
}