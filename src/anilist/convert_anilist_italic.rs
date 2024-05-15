use std::borrow::Cow;
use crate::html::convert_html_italic::convert_html_italic;

pub fn convert_anilist_italic(italic: &str) -> Cow<str> {
    let italic = italic.replace("_", "*");
    let italic = convert_html_italic(&italic);
    italic
}