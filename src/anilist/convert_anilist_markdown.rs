use std::borrow::Cow;
use crate::anilist::convert_anilist_bold::convert_anilist_bold;
use crate::anilist::convert_anilist_italic::convert_anilist_italic;
use crate::anilist::convert_anilist_spoiler::convert_anilist_spoiler;
use crate::common::add_antislash::add_anti_slash;
use crate::html::convert_html_entity::convert_html_entity;
use crate::html::convert_html_line_break::convert_html_line_break;
use crate::html::convert_html_link::convert_html_link;
use crate::html::convert_html_strikethrough::convert_html_strikethrough;

pub fn convert_anilist_flavored_markdown(value: &str) -> Cow<str> {
    let mut result = value;
    result = add_anti_slash(result);


    result = convert_anilist_bold(result);
    result = convert_anilist_italic(result);
    result = convert_anilist_spoiler(result);


    result = convert_html_link(result);
    result = convert_html_entity(result);
    result = convert_html_line_break(result);
    result = convert_html_strikethrough(result);


    result = remove_p_align(result);
    result = convert_blockquote(result);
    result = convert_link_to_discord_markdown(result);
    result = remove_image(result);
    result = convert_h_header(result);
    result = remove_horizontal_line(result);
    result = convert_list(result);
    result = remove_code_block(result);
    result = convert_spoiler(result);
    result = convert_html_entity_to_real_char(result);
    result = convert_html_line_break_to_line_break(result);

    Cow::from(result)
}