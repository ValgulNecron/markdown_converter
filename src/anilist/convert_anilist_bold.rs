use std::borrow::Cow;
use crate::html::convert_html_bold::convert_html_bold;

pub fn convert_anilist_bold(bold: &str) -> Cow<str> {
    let bold = bold.replace("__", "**");
    let bold = convert_html_bold(&bold);
    bold
}