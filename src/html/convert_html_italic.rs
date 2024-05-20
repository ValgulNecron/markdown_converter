use std::borrow::Cow;

/// `convert_html_italic` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert HTML italic tags to markdown syntax in a given string.
/// It specifically targets the following patterns:
/// - HTML italic tags: `<i>Text</i>` and `<em>Text</em>`
///
/// # Arguments
///
/// * `italic` - A string slice that holds the content from which italic tags should be converted.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all italic tags converted to markdown syntax.
///
pub fn convert_html_italic(italic: &str) -> Cow<str> {
    Cow::Owned(
        italic
            .replace("<i>", "*")
            .replace("</i>", "*")
            .replace("<em>", "*")
            .replace("</em>", "*"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn italic_tag_converted() {
        let result = convert_html_italic("<i>Hello</i>");
        assert_eq!(result, "*Hello*");
    }

    #[test]
    fn em_tag_converted() {
        let result = convert_html_italic("<em>World</em>");
        assert_eq!(result, "*World*");
    }

    #[test]
    fn multiple_italic_tags_converted() {
        let result = convert_html_italic("<i>Hello</i> <em>World</em>");
        assert_eq!(result, "*Hello* *World*");
    }

    #[test]
    fn no_tags_unchanged() {
        let result = convert_html_italic("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_html_italic("");
        assert_eq!(result, "");
    }
}
