use regex::Regex;
use std::borrow::Cow;

pub fn remove_html_image(value: &str) -> Cow<str> {
    // remove ![*](*)
    let re = Regex::new(r#"!\[[^]]*]\([^)]*\)"#).unwrap();
    let value = re.replace_all(value, "");
    // also remove <img alt="fallback text" src="https://anilist.co/img/icons/icon.svg">
    let re = Regex::new(r#"<img[^>]*>"#).unwrap();
    let value = re.replace_all(&value, "");

    // also remove img###(https://anilist.co/img/icons/icon.svg) where ### is any number
    let re = Regex::new(r#"img\d+"#).unwrap();
    let value = re.replace_all(&value, "");

    Cow::Owned(value.into_owned())
}
