use std::borrow::Cow;

use regex::Regex;

use crate::html::convert_html_list::convert_html_list;

/// `convert_anilist_list` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert Anilist list syntax to standard markdown list syntax.
/// It specifically targets and converts the following patterns:
/// - Single -, *, or + are replaced with -.
/// - Sequences of - -, * -, + -, and -  are replaced with a single -.
/// - HTML lists are converted to markdown lists.
///
/// # Arguments
///
/// * `value` - A string slice that holds the content to be converted from Anilist list syntax to standard markdown list syntax.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all the conversions applied.
///
pub fn convert_anilist_list(value: &str) -> Cow<str> {
    // replace single - or * or + with -
    let re = Regex::new(r#"^[-*+]"#).unwrap();
    let mut value = re.replace_all(value, "- ").to_string();
    value = value.replace("* -", "-");
    value = value.replace("+ -", "-");
    value = value.replace("-  ", "- ");
    value = value.replace("- -", "-");

    let value = convert_html_list(&value);

    Cow::Owned(value.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_dash_converted_to_list() {
        let result = convert_anilist_list("- item1");
        assert_eq!(result, "- item1");
    }

    #[test]
    fn asterisk_converted_to_list() {
        let result = convert_anilist_list("* item2");
        assert_eq!(result, "- item2");
    }

    #[test]
    fn plus_sign_converted_to_list() {
        let result = convert_anilist_list("+ item3");
        assert_eq!(result, "- item3");
    }

    #[test]
    fn sequences_converted_to_single_dash() {
        let result = convert_anilist_list("- - item1\n* - item2\n+ - item3\n-  item4");
        assert_eq!(result, "- item1\n- item2\n- item3\n- item4");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_anilist_list("");
        assert_eq!(result, "");
    }
}
