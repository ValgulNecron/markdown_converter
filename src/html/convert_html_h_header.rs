use regex::Regex;
use std::borrow::Cow;

pub fn convert_html_h_header(value: &str) -> Cow<str> {
    let mut value = value
        .replace("<h1>", "# ")
        .replace("</h1>", "")
        .replace("<h2>", "## ")
        .replace("</h2>", "")
        .replace("<h3>", "### ")
        .replace("</h3>", "")
        .replace("<h4>", "#### ")
        .replace("</h4>", "")
        .replace("<h5>", "##### ")
        .replace("</h5>", "")
        .replace("<h6>", "###### ")
        .replace("</h6>", "");
    // replace multiple = or - with # or ##
    let re = Regex::new(r#"^=+$"#).unwrap();
    value = re.replace_all(value.as_str(), "#").to_string();
    let re = Regex::new(r#"^-+$"#).unwrap();
    value = re.replace_all(value.as_str(), "##").to_string();

    Cow::Owned(value)
}
