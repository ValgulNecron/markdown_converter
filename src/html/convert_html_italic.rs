use std::borrow::Cow;

pub fn convert_html_italic(italic: &str) -> Cow<str> {
    Cow::Owned(
        italic
            .replace("<i>", "_")
            .replace("</i>", "_")
            .replace("<em>", "_")
            .replace("</em>", "_"),
    )
}
