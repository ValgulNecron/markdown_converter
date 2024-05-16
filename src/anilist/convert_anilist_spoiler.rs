use std::borrow::Cow;

/// `convert_anilist_spoiler` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert Anilist spoiler tags (~!) to Discord spoiler tags (||).
///
/// # Arguments
///
/// * `value` - A string slice that holds the content to be converted from Anilist spoiler tags to Discord spoiler tags.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all the conversions applied.
///
pub fn convert_anilist_spoiler(value: &str) -> Cow<str> {
    Cow::Owned(value.replace("~!", "||").replace("!~", "||"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spoiler_tags_converted() {
        let result = convert_anilist_spoiler("~!Hello!~ ~!World!~");
        assert_eq!(result, "||Hello|| ||World||");
    }

    #[test]
    fn no_spoiler_tags_unchanged() {
        let result = convert_anilist_spoiler("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = convert_anilist_spoiler("");
        assert_eq!(result, "");
    }

    #[test]
    fn partial_spoiler_tags_unchanged() {
        let result = convert_anilist_spoiler("~!Hello World!~");
        assert_eq!(result, "||Hello World||");
    }
}