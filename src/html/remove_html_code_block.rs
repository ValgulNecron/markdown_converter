use std::borrow::Cow;

use regex::Regex;

/// `remove_html_code_block` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to remove HTML code block tags from a given string.
/// It specifically targets the following patterns:
/// - HTML code tags: `<code>` and `</code>`
/// - HTML preformatted text tags: `<pre>` and `</pre>`
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which code block tags should be removed.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all code block tags removed.
///
pub fn remove_html_code_block(value: &str) -> Cow<str> {
    // <code> or <pre> or </code> or </pre>
    let re = Regex::new(r#"<code[^>]*>"#).unwrap();
    let value = re.replace_all(value, "");
    let re = Regex::new(r#"<pre[^>]*>"#).unwrap();
    let value = re.replace_all(&value, "");
    let re = Regex::new(r#"</code>"#).unwrap();
    let value = re.replace_all(&value, "");
    let re = Regex::new(r#"</pre>"#).unwrap();
    let value = re.replace_all(&value, "");

    Cow::Owned(value.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_tags_removed() {
        let result = remove_html_code_block("<code>let x = 10;</code>");
        assert_eq!(result, "let x = 10;");
    }

    #[test]
    fn pre_tags_removed() {
        let result = remove_html_code_block("<pre>let y = 20;</pre>");
        assert_eq!(result, "let y = 20;");
    }

    #[test]
    fn multiple_tags_removed() {
        let result = remove_html_code_block("<code>let x = 10;</code><pre>let y = 20;</pre>");
        assert_eq!(result, "let x = 10;let y = 20;");
    }

    #[test]
    fn no_tags_unchanged() {
        let result = remove_html_code_block("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = remove_html_code_block("");
        assert_eq!(result, "");
    }
}
