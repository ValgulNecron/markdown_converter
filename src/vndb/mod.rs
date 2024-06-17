mod convert_vndb_link;
mod convert_vndb_spoiler;

use std::borrow::Cow;

pub fn convert_vndb_markdown(value: &str) -> Cow<str> {
    let mut result = Cow::Borrowed(value);
    result
}