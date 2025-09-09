use regex::Regex;
use scraper::{Html, Selector, ElementRef};

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

pub fn extract_supers(html: &str) -> String {
    let document = Html::parse_fragment(html);

    // Candidat pour tous les types d'exposants connus
    let selector = Selector::parse(
        r#"span[style*="vertical-align: super"],
           span[style*="baseline-shift"],
           span[style*="mso-text-raise"],
           span[style*="position"][style*="relative"]"#
    ).unwrap();

    let mut result = html.to_string();

    for el in document.select(&selector) {
        let text: String = el.text().collect::<Vec<_>>().join("").trim().to_string();
        if !text.is_empty() {
            let replacement = format!("###{}###", text);
            // remplace le span entier par le texte entouré
            result = result.replace(&el.html(), &replacement);
        }
    }

    // Supprime toutes les balises HTML restantes
    let re_tags = Regex::new(r"<[^>]+>").unwrap();
    let cleaned = re_tags.replace_all(&result, "").to_string();

    // Normalisation des espaces
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}