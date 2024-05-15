use std::borrow::Cow;

pub fn convert_html_strikethrough(value: &str) -> Cow<str> {
    Cow::from(value
        .replace("<del>", "~~")
        .replace("</del>", "~~")
        .replace("<strike>", "~~")
        .replace("</strike>", "~~"))
}