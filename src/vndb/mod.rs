use std::borrow::Cow;

use crate::vndb::convert_vndb_link::convert_vndb_link;
use crate::vndb::convert_vndb_spoiler::convert_vndb_spoiler;

mod convert_vndb_link;
mod convert_vndb_spoiler;

pub fn convert_vndb_markdown(value: &str) -> Cow<str> {
    let mut result = Cow::Borrowed(value);
    result = Cow::Owned(convert_vndb_link(&result).into_owned());
    result = Cow::Owned(convert_vndb_spoiler(&result).into_owned());
    result
}
