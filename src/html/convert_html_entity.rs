use std::borrow::Cow;

pub fn convert_html_entity(html_entity: &str) -> Cow<str> {
    let mut html_entity: String = html_entity.to_string();

    // Common HTML entities
    html_entity = html_entity.replace("&mdash;", "—");
    html_entity = html_entity.replace("&amp;", "&");
    html_entity = html_entity.replace("&lt;", "<");
    html_entity = html_entity.replace("&gt;", ">");
    html_entity = html_entity.replace("&quot;", "\"");
    html_entity = html_entity.replace("&apos;", "'");
    html_entity = html_entity.replace("&nbsp;", " ");
    html_entity = html_entity.replace("&copy;", "©");
    html_entity = html_entity.replace("&reg;", "®");
    html_entity = html_entity.replace("&deg;", "°");
    html_entity = html_entity.replace("&plusmn;", "±");
    html_entity = html_entity.replace("&times;", "×");
    html_entity = html_entity.replace("&divide;", "÷");
    html_entity = html_entity.replace("&frac12;", "½");
    html_entity = html_entity.replace("&frac14;", "¼");
    html_entity = html_entity.replace("&frac34;", "¾");
    html_entity = html_entity.replace("&iexcl;", "¡");
    html_entity = html_entity.replace("&cent;", "¢");
    html_entity = html_entity.replace("&pound;", "£");
    html_entity = html_entity.replace("&euro;", "€");
    html_entity = html_entity.replace("&yen;", "¥");
    html_entity = html_entity.replace("&brvbar;", "¦");
    html_entity = html_entity.replace("&sect;", "§");
    html_entity = html_entity.replace("&uml;", "¨");
    html_entity = html_entity.replace("&bull;", "•");
    html_entity = html_entity.replace("&hellip;", "…");
    html_entity = html_entity.replace("&ndash;", "–");
    html_entity = html_entity.replace("&lsquo;", "‘");
    html_entity = html_entity.replace("&rsquo;", "’");
    html_entity = html_entity.replace("&laquo;", "«");
    html_entity = html_entity.replace("&raquo;", "»");
    html_entity = html_entity.replace("&larr;", "←");
    html_entity = html_entity.replace("&rarr;", "→");
    html_entity = html_entity.replace("&dagger;", "†");
    html_entity = html_entity.replace("&Dagger;", "‡");
    html_entity = html_entity.replace("&clubs;", "♣");
    html_entity = html_entity.replace("&diams;", "♦");
    html_entity = html_entity.replace("&spades;", "♠");
    html_entity = html_entity.replace("&hearts;", "♥");
    html_entity = html_entity.replace("&dales;", "♦");

    Cow::Owned(html_entity)
}
