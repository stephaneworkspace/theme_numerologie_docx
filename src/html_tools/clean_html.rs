use regex::Regex;

/// Nettoie un texte HTML en enlevant toutes les balises et tout ce qui est en "super"
pub fn clean_html(html: &String) -> String {
    // Supprime tout ce qui est dans un span vertical-align: super
    let re_super = Regex::new(r#"<span[^>]*vertical-align:\s*super[^>]*>.*?</span>"#).unwrap();
    let without_super = re_super.replace_all(html, "");

    // Supprime toutes les balises HTML restantes
    let re_tags = Regex::new(r"<[^>]+>").unwrap();
    let plain_text = re_tags.replace_all(&without_super, "");

    // Trim et normalise les espaces
    plain_text.replace("\n", " ").replace("\r", " ").split_whitespace().collect::<Vec<_>>().join(" ")
}