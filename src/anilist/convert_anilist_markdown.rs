use crate::anilist::convert_anilist_bold::convert_anilist_bold;
use crate::anilist::convert_anilist_italic::convert_anilist_italic;
use crate::anilist::convert_anilist_list::convert_anilist_list;
use crate::anilist::convert_anilist_spoiler::convert_anilist_spoiler;
use crate::common::add_antislash::add_anti_slash;
use crate::html::convert_html_blockquote::convert_html_blockquote;
use crate::html::convert_html_entity::convert_html_entity;
use crate::html::convert_html_h_header::convert_html_h_header;
use crate::html::convert_html_line_break::convert_html_line_break;
use crate::html::convert_html_link::convert_html_link;
use crate::html::convert_html_strikethrough::convert_html_strikethrough;
use crate::html::remove_html_horizontal_line::remove_html_horizontal_line;
use crate::html::remove_html_image::remove_html_image;
use crate::html::remove_html_p_align::remove_html_p_align;
use std::borrow::Cow;
use crate::anilist::remove_anilist_code_block::remove_anilist_code_block;

/// `convert_anilist_flavored_markdown` is a function that takes a string reference as an input and returns a Cow<str>.
/// This function is designed to convert a given string from Anilist-flavored markdown to standard markdown.
/// It specifically targets and converts the following patterns:
/// - Backticks (`) are escaped with an anti-slash.
/// - Anilist bold tags are converted to markdown bold syntax.
/// - Anilist italic tags are converted to markdown italic syntax.
/// - Anilist spoiler tags are converted to Discord spoiler tags.
/// - Anilist code blocks are removed.
/// - HTML links are converted to markdown links.
/// - HTML entities are converted to their corresponding characters.
/// - HTML line breaks are converted to markdown line breaks.
/// - HTML strikethrough tags are converted to markdown strikethrough syntax.
/// - HTML blockquote tags are converted to markdown blockquote syntax.
/// - HTML header tags are converted to markdown header syntax.
/// - HTML paragraph alignment tags are removed.
/// - HTML images are removed.
/// - HTML horizontal lines are removed.
/// - Anilist lists are converted to markdown lists.
///
/// # Arguments
///
/// * `value` - A string slice that holds the content to be converted from Anilist-flavored markdown to standard markdown.
///
/// # Returns
///
/// This function returns a Cow<str> which is an owned string with all the conversions applied.
///
pub fn convert_anilist_flavored_markdown(value: &str) -> Cow<str> {
    let mut result = Cow::Borrowed(value);

    result = Cow::Owned(add_anti_slash(&result).into_owned());
    result = Cow::Owned(convert_anilist_bold(&result).into_owned());
    result = Cow::Owned(convert_anilist_italic(&result).into_owned());
    result = Cow::Owned(convert_anilist_spoiler(&result).into_owned());
    result = Cow::Owned(remove_anilist_code_block(&result).into_owned());

    result = Cow::Owned(convert_html_link(&result).into_owned());
    result = Cow::Owned(convert_html_entity(&result).into_owned());
    result = Cow::Owned(convert_html_line_break(&result).into_owned());
    result = Cow::Owned(convert_html_strikethrough(&result).into_owned());
    result = Cow::Owned(convert_html_blockquote(&result).into_owned());
    result = Cow::Owned(convert_html_h_header(&result).into_owned());
    result = Cow::Owned(remove_html_p_align(&result).into_owned());
    result = Cow::Owned(remove_html_image(&result).into_owned());
    result = Cow::Owned(remove_html_horizontal_line(&result).into_owned());
    result = Cow::Owned(convert_anilist_list(&result).into_owned());

    result
}