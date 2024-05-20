use std::borrow::Cow;

/// `add_anti_slash` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to add an anti-slash before each backtick (`) in a given string.
///
/// # Arguments
///
/// * `value` - A string slice that holds the content where backticks should be escaped with an anti-slash.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all backticks escaped with an anti-slash.
///
pub fn add_anti_slash(value: &str) -> Cow<str> {
    Cow::Owned(value.replace('`', "\\`"))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_anti_slash_replaces_backtick_with_slashed_backtick() {
        let input = "`";
        let expected = "\\`";
        assert_eq!(add_anti_slash(input), expected);
    }

    #[test]
    fn add_anti_slash_leaves_string_without_backtick_unchanged() {
        let input = "Hello, World!";
        let expected = "Hello, World!";
        assert_eq!(add_anti_slash(input), expected);
    }

    #[test]
    fn add_anti_slash_handles_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(add_anti_slash(input), expected);
    }

    #[test]
    fn add_anti_slash_handles_multiple_backticks() {
        let input = "`Hello,` `World!`";
        let expected = "\\`Hello,\\` \\`World!\\`";
        assert_eq!(add_anti_slash(input), expected);
    }
}
