use std::borrow::Cow;

use regex::Regex;

pub fn convert_vndb_link(value: &str) -> Cow<str> {
    let re = Regex::new(r#"\[url=(https?://[^ ]+)](.*?)\[/url\]"#).unwrap();
    let value = re.replace_all(value, "[$2]($1)").to_string();
    let re = Regex::new(r#"\[url=(/[^ ]+)]([a-zA-Z0-9]*)\[/url]"#).unwrap();
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

    #[test]
    fn link_tag_converted_text_with_path() {
        let result = convert_vndb_link(
            "softer interior as a result of insecurity, convincing [url=/c64502]Monika[/url] enough to class Natsuki as the embodiment of a tsundere. While she is impulsive and can speak without thinking, she truly care",
        );
assert_eq!(
            result,
            "softer interior as a result of insecurity, convincing [Monika](https://vndb.org/c64502) enough to class Natsuki as the embodiment of a tsundere. While she is impulsive and can speak without thinking, she truly care"
        );
    }

    #[test]
    fn link_tag_converted_text_multiple_with_path() {
        let result = convert_vndb_link(
            "Natsuki is a brash, blunt, cranky, seemingly arrogant girl with a cute, softer interior as a result of insecurity, convincing [url=/c64502]Monika[/url] enough to class Natsuki as the embodiment of a tsundere. While she is impulsive and can speak without thinking, she truly cares about her friends and, even when she has obvious anger issues, doesn't enjoy fights or arguments with people. [spoiler]Over the course of the game, we see that she worries about [url=/c64504]Yuri[/url] and on one occasion, gives the [url=/c64506]Protagonist[/url] a note asking him to help [url=/c64504]Yuri[/url], fearing that if she spoke out then it would cause more arguments.[/spoiler] She is very stubborn and has a hard time expressing how she feels and what she wants. When repeatedly challenged, she often becomes awkward and tongue-tied, then aggressive, and then simply bursts into tears.",
        );
        assert_eq!(
            result,
            "Natsuki is a brash, blunt, cranky, seemingly arrogant girl with a cute, softer interior as a result of insecurity, convincing [Monika](https://vndb.org/c64502) enough to class Natsuki as the embodiment of a tsundere. While she is impulsive and can speak without thinking, she truly cares about her friends and, even when she has obvious anger issues, doesn't enjoy fights or arguments with people. [spoiler]Over the course of the game, we see that she worries about [Yuri](https://vndb.org/c64504) and on one occasion, gives the [Protagonist](https://vndb.org/c64506) a note asking him to help [Yuri](https://vndb.org/c64504), fearing that if she spoke out then it would cause more arguments.[/spoiler] She is very stubborn and has a hard time expressing how she feels and what she wants. When repeatedly challenged, she often becomes awkward and tongue-tied, then aggressive, and then simply bursts into tears."
        );
    }
}
