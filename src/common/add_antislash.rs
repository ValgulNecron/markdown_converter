use std::borrow::Cow;

pub fn add_anti_slash(value: &str) -> Cow<str> {
    let value = value.replace('`', "\\`");
    Cow::from(value)
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