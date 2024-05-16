use crate::html::convert_html_italic::convert_html_italic;
use std::borrow::Cow;

pub fn convert_anilist_italic(italic: &str) -> Cow<str> {
    let italic = italic.replace("_", "*");
    Cow::Owned(convert_html_italic(&italic).into_owned())
}
