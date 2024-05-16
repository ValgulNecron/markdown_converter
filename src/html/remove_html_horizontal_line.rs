use regex::Regex;
use std::borrow::Cow;

pub fn remove_html_horizontal_line(value: &str) -> Cow<str> {
    let re = Regex::new(r#"<hr>"#).unwrap();
    let value = re.replace_all(value, "");
    // also remove <hr />
    let re = Regex::new(r#"<hr\s*/>"#).unwrap();
    let value = re.replace_all(&value, "");
    // if there is --- or *** can be 3 or more
    let re = Regex::new(r#"^-{3,}$"#).unwrap();
    let value = re.replace_all(&value, "");
    let re = Regex::new(r#"^\*{3,}$"#).unwrap();
    let value = re.replace_all(&value, "");

    Cow::Owned(value.into_owned())
}
