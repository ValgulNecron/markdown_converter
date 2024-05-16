use crate::html::convert_html_list::convert_html_list;
use regex::Regex;
use std::borrow::Cow;

pub fn convert_anilist_list(value: &str) -> Cow<str> {
    // replace single - or * or + with -
    let re = Regex::new(r#"^[-*+]"#).unwrap();
    let mut value = re.replace_all(value, "- ").to_string();
    value = value.replace("- -", "-");
    value = value.replace("* -", "-");
    value = value.replace("+ -", "-");
    value = value.replace("-  ", "- ");

    let value = convert_html_list(&value);

    Cow::Owned(value.into_owned())
}
