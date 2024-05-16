use std::borrow::Cow;

/// `convert_html_bold` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert HTML bold tags to markdown syntax in a given string.
/// It specifically targets the following patterns:
/// - HTML bold tags: `<strong>Text</strong>` and `<b>Text</b>`
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which bold tags should be converted.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all bold tags converted to markdown syntax.
///
pub fn convert_html_bold(value: &str) -> Cow<str> {
    Cow::Owned(
        value
            .replace("<strong>", "**")
            .replace("</strong>", "**")
            .replace("<b>", "**")
            .replace("</b>", "**"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_html_bold_replaces_strong_tags() {
        let input = "<strong>Hello, World!</strong>";
        let expected = "**Hello, World!**";
        assert_eq!(convert_html_bold(input), expected);
    }

    #[test]
    fn convert_html_bold_replaces_b_tags() {
        let input = "<b>Hello, World!</b>";
        let expected = "**Hello, World!**";
        assert_eq!(convert_html_bold(input), expected);
    }

    #[test]
    fn convert_html_bold_handles_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(convert_html_bold(input), expected);
    }

    #[test]
    fn convert_html_bold_leaves_string_without_tags_unchanged() {
        let input = "Hello, World!";
        let expected = "Hello, World!";
        assert_eq!(convert_html_bold(input), expected);
    }

    #[test]
    fn convert_html_bold_handles_multiple_tags() {
        let input = "<b>Hello,</b> <strong>World!</strong>";
        let expected = "**Hello,** **World!**";
        assert_eq!(convert_html_bold(input), expected);
    }
}
