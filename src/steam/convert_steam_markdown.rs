use std::borrow::Cow;
use crate::common::add_antislash::add_anti_slash;
use crate::html::convert_html_blockquote::convert_html_blockquote;
use crate::html::convert_html_bold::convert_html_bold;
use crate::html::convert_html_entity::convert_html_entity;
use crate::html::convert_html_h_header::convert_html_h_header;
use crate::html::convert_html_italic::convert_html_italic;
use crate::html::convert_html_line_break::convert_html_line_break;
use crate::html::convert_html_link::convert_html_link;
use crate::html::convert_html_strikethrough::convert_html_strikethrough;

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