use std::borrow::Cow;

pub fn convert_html_bold(value: &str) -> Cow<str> {
    Cow::from(value
        .replace("<strong>", "**")
        .replace("</strong>", "**")
        .replace("<b>", "**")
        .replace("</b>", "**"))
}