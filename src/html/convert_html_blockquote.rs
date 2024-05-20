use regex::Regex;
use std::borrow::Cow;

/// `convert_html_blockquote` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert HTML blockquote tags to markdown syntax in a given string.
/// It specifically targets the following patterns:
/// - HTML blockquote tags: `<blockquote>Text</blockquote>`
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which blockquote tags should be converted.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all blockquote tags converted to markdown syntax.
///
pub fn convert_html_blockquote(value: &str) -> Cow<str> {
    let mut value = value
        .replace("<blockquote>", "> ")
        .replace("</blockquote>", "");

    let re = Regex::new(r#">+"#).unwrap();
    value = re.replace_all(value.as_str(), ">").to_string();

    Cow::Owned(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockquote_tag_converted() {
        let result = convert_html_blockquote("<blockquote>Hello World</blockquote>");
        assert_eq!(result, "> Hello World");
    }

    #[test]
    fn multiple_blockquote_tags_converted() {
        let result = convert_html_blockquote(
            "<blockquote>Hello</blockquote> <blockquote>World</blockquote>",
        );
        assert_eq!(result, "> Hello > World");
    }

    #[test]
    fn no_blockquote_tags_unchanged() {
        let result = convert_html_blockquote("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_html_blockquote("");
        assert_eq!(result, "");
    }
}
