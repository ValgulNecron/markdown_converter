use std::borrow::Cow;

/// `remove_html_p_align` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to remove the alignment attributes from paragraph tags in HTML.
/// It specifically targets the "left", "center", "right", and "justify" alignment attributes.
/// The function also removes the opening and closing paragraph tags.
///
/// # Arguments
///
/// * `value` - A string slice that holds the HTML content.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with the alignment attributes and paragraph tags removed.
///
pub fn remove_html_p_align(value: &str) -> Cow<str> {
    Cow::Owned(
        value
            .replace("<p align=\"left\">", "")
            .replace("<p align=\"center\">", "")
            .replace("<p align=\"right\">", "")
            .replace("<p align=\"justify\">", "")
            .replace("<p>", "")
            .replace("</p>", ""),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment_left_removed() {
        let result = remove_html_p_align("<p align=\"left\">Hello, world!</p>");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn alignment_center_removed() {
        let result = remove_html_p_align("<p align=\"center\">Hello, world!</p>");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn alignment_right_removed() {
        let result = remove_html_p_align("<p align=\"right\">Hello, world!</p>");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn alignment_justify_removed() {
        let result = remove_html_p_align("<p align=\"justify\">Hello, world!</p>");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn paragraph_tags_removed() {
        let result = remove_html_p_align("<p>Hello, world!</p>");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = remove_html_p_align("");
        assert_eq!(result, "");
    }

    #[test]
    fn no_paragraph_tags_unchanged() {
        let result = remove_html_p_align("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }
}