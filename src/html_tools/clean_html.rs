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

    // Sélecteurs pour les différents styles
    let selector_super = Selector::parse(r#"span[style*="vertical-align: super"]"#).unwrap();
    let selector_bold = Selector::parse(r#"span[style*="font-weight: bold"]"#).unwrap();
    let selector_italic = Selector::parse(r#"span[style*="font-style: italic"]"#).unwrap();

    let mut result = html.to_string();

    // Traiter les super first: remplacer le span par ###texte###
    for el in document.select(&selector_super) {
        let text: String = el.text().collect::<Vec<_>>().join("").trim().to_string();
        if !text.is_empty() {
            // Ne pas modifier si déjà entouré par ###
            if !(text.starts_with("###") && text.ends_with("###")) {
                let replacement = format!("###{}###", text);
                // remplace le span entier par le texte entouré
                result = result.replace(&el.html(), &replacement);
            }
        }
    }

    // Reparse le résultat pour appliquer les styles bold et italic
    let document = Html::parse_fragment(&result);

    // Traiter le bold: entoure par _BBB...BBB_ si pas déjà entouré
    for el in document.select(&selector_bold) {
        let text: String = el.text().collect::<Vec<_>>().join("").trim().to_string();
        if !text.is_empty() {
            // Ne pas modifier si déjà entouré par _BBB...BBB_
            if !(text.starts_with("_BBB") && text.ends_with("BBB_")) {
                // Ne pas modifier si texte est déjà entouré par ### (super)
                if !(text.starts_with("###") && text.ends_with("###")) {
                    let replacement = format!("_BBB{}BBB_", text);
                    result = result.replace(&el.html(), &replacement);
                }
            }
        }
    }

    // Reparse le résultat pour appliquer italic
    let document = Html::parse_fragment(&result);

    // Traiter le italic: entoure par _III...III_ si pas déjà entouré
    for el in document.select(&selector_italic) {
        let text: String = el.text().collect::<Vec<_>>().join("").trim().to_string();
        if !text.is_empty() {
            // Ne pas modifier si déjà entouré par _III...III_
            if !(text.starts_with("_III") && text.ends_with("III_")) {
                // Ne pas modifier si texte est déjà entouré par ### (super)
                if !(text.starts_with("###") && text.ends_with("###")) {
                    let replacement = format!("_III{}III_", text);
                    result = result.replace(&el.html(), &replacement);
                }
            }
        }
    }

    // Supprime toutes les balises HTML restantes
    let re_tags = Regex::new(r"<[^>]+>").unwrap();
    let cleaned = re_tags.replace_all(&result, "").to_string();

    // Normalisation des espaces
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}