use regex::Regex;
use std::borrow::Cow;

pub fn remove_html_code_block(value: &str) -> Cow<str> {
    // <code> or <pre> or </code> or </pre>
    let re = Regex::new(r#"<code[^>]*>"#).unwrap();
    let value = re.replace_all(value, "");
    let re = Regex::new(r#"<pre[^>]*>"#).unwrap();
    let value = re.replace_all(&value, "");
    let re = Regex::new(r#"</code>"#).unwrap();
    let value = re.replace_all(&value, "");
    let re = Regex::new(r#"</pre>"#).unwrap();
    let value = re.replace_all(&value, "");

    Cow::Owned(value.into_owned())
}
