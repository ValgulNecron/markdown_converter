use crate::html::convert_html_italic::convert_html_italic;
use std::borrow::Cow;

/// `convert_anilist_italic` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert Anilist italic syntax to standard markdown italic syntax.
/// It specifically targets and converts the following patterns:
/// - Underscores (_) are replaced with asterisks (*).
/// - HTML italic tags are converted to markdown italic syntax.
///
/// # Arguments
///
/// * `italic` - A string slice that holds the content to be converted from Anilist italic syntax to standard markdown italic syntax.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all the conversions applied.
///
pub fn convert_anilist_italic(italic: &str) -> Cow<str> {
    let italic = italic.replace("_", "*");
    Cow::Owned(convert_html_italic(&italic).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn underscores_converted_to_asterisks() {
        let result = convert_anilist_italic("_Hello_ _World_");
        assert_eq!(result, "*Hello* *World*");
    }

    #[test]
    fn html_italic_tags_converted() {
        let result = convert_anilist_italic("<i>Hello</i> <i>World</i>");
        assert_eq!(result, "*Hello* *World*");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_anilist_italic("");
        assert_eq!(result, "");
    }

    #[test]
    fn no_italic_tags_unchanged() {
        let result = convert_anilist_italic("Hello World");
        assert_eq!(result, "Hello World");
    }
}
