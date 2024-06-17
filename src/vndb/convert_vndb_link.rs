use std::borrow::Cow;

use regex::Regex;

pub fn convert_vndb_link(value: &str) -> Cow<str> {
    let re = Regex::new(r#"\[url=(https?://[^ ]+)](.*?)\[/url\]"#).unwrap();
    let value = re.replace_all(value, "[$2]($1)").to_string();
    let re = Regex::new(r#"\[url=(/[^ ]+)](.*?)\[/url\]"#).unwrap();
    let value = re
        .replace_all(&value, "[$2](https://vndb.org$1)")
        .to_string();
    Cow::Owned(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_tag_converted() {
        let result = convert_vndb_link(
            "[url=http://doki-doki-literature-club.wikia.com/wiki/Natsuki]DDLC Wiki[/url]",
        );
        assert_eq!(
            result,
            "[DDLC Wiki](http://doki-doki-literature-club.wikia.com/wiki/Natsuki)"
        );
    }

    #[test]
    fn link_tag_converted_with_path() {
        let result = convert_vndb_link("[url=/c64504]Yuri[/url]");
        assert_eq!(result, "[Yuri](https://vndb.org/c64504)");
    }
}
