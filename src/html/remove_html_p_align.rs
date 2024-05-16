use std::borrow::Cow;

pub fn remove_html_p_align(value: &str) -> Cow<str> {
    Cow::Owned(
        value
            .replace("<p align=\"left\">", "")
            .replace("<p align=\"center\">", "")
            .replace("<p align=\"right\">", "")
            .replace("<p align=\"justify\">", "")
            .replace("</p>", ""),
    )
}
