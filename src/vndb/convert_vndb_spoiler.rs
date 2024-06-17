use std::borrow::Cow;

pub fn convert_vndb_spoiler(value: &str) -> Cow<str> {
    Cow::Owned(value.replace("[spoiler]", "||").replace("[/spoiler]", "||"))
}
