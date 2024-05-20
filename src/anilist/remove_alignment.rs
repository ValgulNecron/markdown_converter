use crate::html::remove_html_p_align::remove_html_p_align;
use std::borrow::Cow;

/// `remove_anilist_alignment` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to remove Anilist alignment syntax from the input string.
/// It specifically targets and removes the following patterns:
/// - Triple tilde (˜˜˜) characters are removed.
/// - HTML paragraph alignment tags are removed.
///
/// # Arguments
///
/// * `value` - A string slice that holds the content to be cleaned from Anilist alignment syntax.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all the alignment syntax removed.
///
pub fn remove_anilist_alignment(value: &str) -> Cow<str> {
    let value = value.replace("˜˜˜", "");
    let value = remove_html_p_align(&value);
    Cow::Owned(value.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triple_tilde_removed() {
        let result = remove_anilist_alignment("˜˜˜Hello˜˜˜ World");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn html_paragraph_alignment_tags_removed() {
        let result = remove_anilist_alignment("<p align=\"center\">Hello</p> World");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = remove_anilist_alignment("");
        assert_eq!(result, "");
    }

    #[test]
    fn no_alignment_tags_unchanged() {
        let result = remove_anilist_alignment("Hello World");
        assert_eq!(result, "Hello World");
    }
}
