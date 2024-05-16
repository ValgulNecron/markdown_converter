use regex::Regex;
use std::borrow::Cow;

pub fn convert_html_blockquote(value: &str) -> Cow<str> {
    let mut value = value
        .replace("<blockquote>", "> ")
        .replace("</blockquote>", "");

    let re = Regex::new(r#">+"#).unwrap();
    value = re.replace_all(value.as_str(), ">").to_string();

    Cow::Owned(value)
}
