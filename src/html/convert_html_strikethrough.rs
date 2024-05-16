use std::borrow::Cow;

/// `convert_html_strikethrough` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert HTML strikethrough tags to markdown syntax in a given string.
/// It specifically targets the following patterns:
/// - HTML strikethrough tags: `<del>` and `</del>`
/// - HTML strike tags: `<strike>` and `</strike>`
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which strikethrough tags should be converted.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all strikethrough tags converted to markdown syntax.
///
pub fn convert_html_strikethrough(value: &str) -> Cow<str> {
    Cow::Owned(
        value
            .replace("<del>", "~~")
            .replace("</del>", "~~")
            .replace("<strike>", "~~")
            .replace("</strike>", "~~"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn del_tags_converted() {
        let result = convert_html_strikethrough("<del>strikethrough</del>");
        assert_eq!(result, "~~strikethrough~~");
    }

    #[test]
    fn strike_tags_converted() {
        let result = convert_html_strikethrough("<strike>strikethrough</strike>");
        assert_eq!(result, "~~strikethrough~~");
    }

    #[test]
    fn multiple_tags_converted() {
        let result = convert_html_strikethrough("<del>strikethrough</del><strike>strikethrough</strike>");
        assert_eq!(result, "~~strikethrough~~~~strikethrough~~");
    }

    #[test]
    fn no_tags_unchanged() {
        let result = convert_html_strikethrough("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_html_strikethrough("");
        assert_eq!(result, "");
    }
}