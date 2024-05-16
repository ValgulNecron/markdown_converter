use regex::Regex;
use std::borrow::Cow;

pub fn convert_html_link(value: &str) -> Cow<str> {
    let re = Regex::new(r#"<a\s+href="([^"]+)">([^<]+)</a>"#).unwrap();
    let value = re.replace_all(value, "[$2]($1)").to_string();
    Cow::Owned(value)
}
