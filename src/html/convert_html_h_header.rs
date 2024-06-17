use std::borrow::Cow;

use regex::Regex;

/// `convert_html_h_header` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert HTML header tags to markdown syntax in a given string.
/// It specifically targets the following patterns:
/// - HTML header tags: `<h1>Text</h1>` to `<h6>Text</h6>`
/// - HTML header underline styles: `Text\n=====` and `Text\n-----`
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which header tags should be converted.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all header tags converted to markdown syntax.
///
pub fn convert_html_h_header(value: &str) -> Cow<str> {
    let mut value = value
        .replace("<h1>", "# ")
        .replace("</h1>", "")
        .replace("<h2>", "## ")
        .replace("</h2>", "")
        .replace("<h3>", "### ")
        .replace("</h3>", "")
        .replace("<h4>", "#### ")
        .replace("</h4>", "")
        .replace("<h5>", "##### ")
        .replace("</h5>", "")
        .replace("<h6>", "###### ")
        .replace("</h6>", "");
    // replace multiple = or - with # or ##
    let re = Regex::new(r#"^=+$"#).unwrap();
    value = re.replace_all(value.as_str(), "#").to_string();
    let re = Regex::new(r#"^-+$"#).unwrap();
    value = re.replace_all(value.as_str(), "##").to_string();

    Cow::Owned(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h1_tag_converted() {
        let result = convert_html_h_header("<h1>Hello</h1>");
        assert_eq!(result, "# Hello");
    }

    #[test]
    fn h2_tag_converted() {
        let result = convert_html_h_header("<h2>World</h2>");
        assert_eq!(result, "## World");
    }

    #[test]
    fn multiple_header_tags_converted() {
        let result = convert_html_h_header("<h1>Hello</h1> <h2>World</h2>");
        assert_eq!(result, "# Hello ## World");
    }

    #[test]
    fn no_tags_unchanged() {
        let result = convert_html_h_header("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_html_h_header("");
        assert_eq!(result, "");
    }
}
