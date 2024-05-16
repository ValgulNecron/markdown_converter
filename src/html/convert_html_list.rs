use regex::Regex;
use std::borrow::Cow;

/// `convert_html_list` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert HTML list tags to markdown syntax in a given string.
/// It specifically targets the following patterns:
/// - HTML unordered list tags: `<ul>` and `</ul>`
/// - HTML ordered list tags: `<ol>` and `</ol>`
/// - HTML list item tags: `<li>` and `</li>`
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which list tags should be converted.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all list tags converted to markdown syntax.
///
pub fn convert_html_list(value: &str) -> Cow<str> {
    let mut value = value
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<ol>", "")
        .replace("</ol>", "")
        .replace("</li>", "\\n");

    let re = Regex::new(r#"<li[^>]*>"#).unwrap();
    value = re.replace_all(value.as_str(), "- ").to_string();
    Cow::Owned(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unordered_list_converted() {
        let result = convert_html_list("<ul><li>Item 1</li><li>Item 2</li></ul>");
        assert_eq!(result, "- Item 1\\n- Item 2\\n");
    }

    #[test]
    fn ordered_list_converted() {
        let result = convert_html_list("<ol><li>Item 1</li><li>Item 2</li></ol>");
        assert_eq!(result, "- Item 1\\n- Item 2\\n");
    }

    #[test]
    fn multiple_lists_converted() {
        let result = convert_html_list("<ul><li>Item 1</li><li>Item 2</li></ul><ol><li>Item 3</li><li>Item 4</li></ol>");
        assert_eq!(result, "- Item 1\\n- Item 2\\n- Item 3\\n- Item 4\\n");
    }

    #[test]
    fn no_tags_unchanged() {
        let result = convert_html_list("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_html_list("");
        assert_eq!(result, "");
    }
}