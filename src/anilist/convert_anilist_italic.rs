pub fn convert_anilist_italic(italic: &str) -> &str {
    let italic = italic.replace("_", "*");
    let italic = convert_html_italic(&italic);
    &italic
}