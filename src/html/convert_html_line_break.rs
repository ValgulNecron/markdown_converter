use std::borrow::Cow;

pub fn convert_html_line_break(value: &str) -> Cow<str> {
    Cow::from(value.replace("<br>", "\n"))
}