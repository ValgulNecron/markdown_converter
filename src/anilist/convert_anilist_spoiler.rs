use std::borrow::Cow;

pub fn convert_anilist_spoiler(value: &str) -> Cow<str> {
    Cow::from(value.replace("~!", "||").replace("!~", "||"))
}
