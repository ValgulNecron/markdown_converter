use std::borrow::Cow;
use regex::Regex;

pub fn convert_vndb_spoiler(value: &str) -> Cow<str> {
    Cow::Owned(value.replace("[spoiler]", "||").replace("[/spoiler]", "||"))
}