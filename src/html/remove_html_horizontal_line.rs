use regex::Regex;
use std::borrow::Cow;

/// `remove_html_horizontal_line` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to remove horizontal line syntax from a given string.
/// It specifically targets the following patterns:
/// - HTML horizontal line tags: `<hr>` and `<hr />`
/// - Markdown horizontal line syntax: `---` or `***` (3 or more)
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which horizontal lines should be removed.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all horizontal line syntax removed.
///
pub fn remove_html_horizontal_line(value: &str) -> Cow<str> {
    let re = Regex::new(r#"<hr>"#).unwrap();
    let value = re.replace_all(value, "");
    // also remove <hr />
    let re = Regex::new(r#"<hr\s*/>"#).unwrap();
    let value = re.replace_all(&value, "");
    // if there is --- or *** can be 3 or more
    let re = Regex::new(r#"^-{3,}$"#).unwrap();
    let value = re.replace_all(&value, "");
    let re = Regex::new(r#"^\*{3,}$"#).unwrap();
    let value = re.replace_all(&value, "");

    Cow::Owned(value.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_horizontal_line_removed() {
        let result = remove_html_horizontal_line("<hr>");
        assert_eq!(result, "");
    }

    #[test]
    fn html_horizontal_line_with_slash_removed() {
        let result = remove_html_horizontal_line("<hr />");
        assert_eq!(result, "");
    }

    #[test]
    fn markdown_horizontal_line_dash_removed() {
        let result = remove_html_horizontal_line("---");
        assert_eq!(result, "");
    }

    #[test]
    fn markdown_horizontal_line_asterisk_removed() {
        let result = remove_html_horizontal_line("***");
        assert_eq!(result, "");
    }

    #[test]
    fn no_horizontal_lines_unchanged() {
        let result = remove_html_horizontal_line("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = remove_html_horizontal_line("");
        assert_eq!(result, "");
    }
}