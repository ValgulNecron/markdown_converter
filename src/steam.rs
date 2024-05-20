use crate::common::add_anti_slash;
use crate::html::convert_html_blockquote::convert_html_blockquote;
use crate::html::convert_html_bold::convert_html_bold;
use crate::html::convert_html_entity::convert_html_entity;
use crate::html::convert_html_h_header::convert_html_h_header;
use crate::html::convert_html_italic::convert_html_italic;
use crate::html::convert_html_line_break::convert_html_line_break;
use crate::html::convert_html_link::convert_html_link;
use crate::html::convert_html_strikethrough::convert_html_strikethrough;
use std::borrow::Cow;

/// `convert_steam_flavored_markdown` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert a given string from Steam-flavored markdown to standard markdown.
/// It specifically targets and converts the following patterns:
/// - Backticks (`) are escaped with an anti-slash.
/// - HTML entities are converted to their corresponding characters.
/// - HTML links are converted to markdown links.
/// - HTML line breaks are converted to markdown line breaks.
/// - HTML bold tags are converted to markdown bold syntax.
/// - HTML strikethrough tags are converted to markdown strikethrough syntax.
/// - HTML blockquote tags are converted to markdown blockquote syntax.
/// - HTML header tags are converted to markdown header syntax.
/// - HTML italic tags are converted to markdown italic syntax.
///
/// # Arguments
///
/// * `value` - A string slice that holds the content to be converted from Steam-flavored markdown to standard markdown.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all the conversions applied.
///
pub fn convert_steam_flavored_markdown(value: &str) -> Cow<str> {
    let mut result = Cow::Borrowed(value);
    result = Cow::Owned(add_anti_slash(&result).into_owned());
    result = Cow::Owned(convert_html_entity(&result).into_owned());
    result = Cow::Owned(convert_html_link(&result).into_owned());
    result = Cow::Owned(convert_html_line_break(&result).into_owned());
    result = Cow::Owned(convert_html_bold(&result).into_owned());
    result = Cow::Owned(convert_html_strikethrough(&result).into_owned());
    result = Cow::Owned(convert_html_blockquote(&result).into_owned());
    result = Cow::Owned(convert_html_h_header(&result).into_owned());
    result = Cow::Owned(convert_html_italic(&result).into_owned());

    result
}
