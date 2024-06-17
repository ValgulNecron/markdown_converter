use std::borrow::Cow;

use regex::Regex;

/// `remove_anilist_horizontal_line` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to remove Anilist horizontal line syntax from the input string.
/// It specifically targets and removes the following patterns:
/// - Three or more consecutive asterisks (***)
/// - Three or more consecutive hyphens (---)
///
/// # Arguments
///
/// * `value` - A string slice that holds the content to be cleaned from Anilist horizontal line syntax.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all the horizontal line syntax removed.
///
pub fn remove_anilist_horizontal_line(value: &str) -> Cow<str> {
    // remove --- or - - - or *** or * * *
    let re = Regex::new(r#"(\* ?\* ?\*|- ?- ?-)"#).unwrap();
    let value = re.replace_all(value, "");
    Cow::Owned(value.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consecutive_asterisks_removed() {
        let result = remove_anilist_horizontal_line("***Hello*** World");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn consecutive_asterisks_removed_with_space() {
        let result = remove_anilist_horizontal_line("* * *Hello* * * World");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn consecutive_hyphens_removed() {
        let result = remove_anilist_horizontal_line("---Hello--- World");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn consecutive_hyphens_removed_with_space() {
        let result = remove_anilist_horizontal_line("- - -Hello- - - World");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = remove_anilist_horizontal_line("");
        assert_eq!(result, "");
    }

    #[test]
    fn no_horizontal_line_tags_unchanged() {
        let result = remove_anilist_horizontal_line("Hello World");
        assert_eq!(result, "Hello World");
    }
}
