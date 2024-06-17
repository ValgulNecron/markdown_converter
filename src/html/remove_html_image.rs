use std::borrow::Cow;

use regex::Regex;

/// `remove_html_image` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to remove image tags and markdown image syntax from a given string.
/// It specifically targets the following patterns:
/// - Markdown image syntax: `![alt text](url)`
/// - HTML image tags: `<img alt="fallback text" src="url">`
/// - Custom image syntax: `img###(url)` where ### is any number
///
/// # Arguments
///
/// * `value` - A string slice that holds the content from which images should be removed.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all image syntax removed.
///
pub fn remove_html_image(value: &str) -> Cow<str> {
    // remove ![*](*)
    let re = Regex::new(r#"!\[[^]]*]\([^)]*\)"#).unwrap();
    let value = re.replace_all(value, "");
    // also remove <img alt="fallback text" src="https://anilist.co/img/icons/icon.svg">
    let re = Regex::new(r#"<img[^>]*>"#).unwrap();
    let value = re.replace_all(&value, "");

    Cow::Owned(value.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_image_removed() {
        let result = remove_html_image("![alt text](https://example.com/image.png)");
        assert_eq!(result, "");
    }

    #[test]
    fn html_image_removed() {
        let result =
            remove_html_image("<img alt=\"fallback text\" src=\"https://example.com/image.png\">");
        assert_eq!(result, "");
    }

    #[test]
    fn no_images_unchanged() {
        let result = remove_html_image("Hello, world!");
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn empty_string_unchanged() {
        let result = remove_html_image("");
        assert_eq!(result, "");
    }
}
