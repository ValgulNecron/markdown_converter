use std::borrow::Cow;

pub fn convert_html_line_break(value: &str) -> Cow<str> {
    Cow::Owned(value.replace("<br>", "\n"))
}
