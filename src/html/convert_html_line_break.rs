use std::borrow::Cow;

/// `convert_html_line_break` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert HTML line break tags to markdown syntax in a given string.
/// It specifically targets the following pattern:
/// - HTML line break tags: `<br>`
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which line break tags should be converted.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all line break tags converted to markdown syntax.
///
pub fn convert_html_line_break(value: &str) -> Cow<str> {
    Cow::Owned(value.replace("<br>", "\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_break_tag_converted() {
        let result = convert_html_line_break("Hello<br>World");
        assert_eq!(result, "Hello\nWorld");
    }

    #[test]
    fn multiple_line_break_tags_converted() {
        let result = convert_html_line_break("Hello<br>World<br>Again");
        assert_eq!(result, "Hello\nWorld\nAgain");
    }

    #[test]
    fn no_tags_unchanged() {
        let result = convert_html_line_break("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_html_line_break("");
        assert_eq!(result, "");
    }
}
