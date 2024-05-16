use regex::Regex;
use std::borrow::Cow;

pub fn remove_anilist_code_block(value: &str) -> Cow<str> {
    // remove ` or ```
    let re = Regex::new(r#"`"#).unwrap();
    let value = re.replace_all(value, "");
    let re = Regex::new(r#"```"#).unwrap();
    let value = re.replace_all(&value, "");
    Cow::Owned(value.into_owned())
}
