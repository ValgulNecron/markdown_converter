use std::borrow::Cow;

pub fn convert_html_italic(italic: &str) -> Cow<str> {
    let italic = italic.replace("<i>", "_")
        .replace("</i>", "_")
        .replace("<em>", "_")
        .replace("</em>", "_");
    Cow::from(italic)
}