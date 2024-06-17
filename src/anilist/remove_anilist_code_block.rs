use std::borrow::Cow;

use regex::Regex;

/// `remove_anilist_code_block` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to remove single (`) and triple backticks (```) from a given string.
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which backticks should be removed.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all single and triple backticks removed.
///
pub fn remove_anilist_code_block(value: &str) -> Cow<str> {
    // remove ` or ```
    let re = Regex::new(r#"`"#).unwrap();
    let value = re.replace_all(value, "");
    let re = Regex::new(r#"```"#).unwrap();
    let value = re.replace_all(&value, "");
    Cow::Owned(value.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_backtick_removed() {
        let result = remove_anilist_code_block("`Hello` `World`");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn triple_backtick_removed() {
        let result = remove_anilist_code_block("```Hello``` ```World```");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn mixed_backticks_removed() {
        let result = remove_anilist_code_block("`Hello` ```World```");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn no_backticks_unchanged() {
        let result = remove_anilist_code_block("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = remove_anilist_code_block("");
        assert_eq!(result, "");
    }
}
