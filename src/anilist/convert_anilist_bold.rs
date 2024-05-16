use crate::html::convert_html_bold::convert_html_bold;
use std::borrow::Cow;

pub fn convert_anilist_bold(bold: &str) -> Cow<str> {
    let bold = bold.replace("__", "**");
    Cow::Owned(convert_html_bold(&bold).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_anilist_bold_replaces_double_underscores() {
        let input = "__Hello, World!__";
        let expected = "**Hello, World!**";
        assert_eq!(convert_anilist_bold(input), expected);
    }

    #[test]
    fn convert_anilist_bold_handles_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(convert_anilist_bold(input), expected);
    }

    #[test]
    fn convert_anilist_bold_leaves_string_without_double_underscores_unchanged() {
        let input = "Hello, World!";
        let expected = "Hello, World!";
        assert_eq!(convert_anilist_bold(input), expected);
    }

    #[test]
    fn convert_anilist_bold_handles_multiple_double_underscores() {
        let input = "__Hello,__ __World!__";
        let expected = "**Hello,** **World!**";
        assert_eq!(convert_anilist_bold(input), expected);
    }
}
