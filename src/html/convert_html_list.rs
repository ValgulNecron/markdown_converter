use regex::Regex;
use std::borrow::Cow;

pub fn convert_html_list(value: &str) -> Cow<str> {
    let mut value = value
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<ol>", "")
        .replace("</ol>", "")
        .replace("</li>", "\\n");

    let re = Regex::new(r#"<li[^>]*>"#).unwrap();
    value = re.replace_all(value.as_str(), "- ").to_string();
    Cow::Owned(value)
}
