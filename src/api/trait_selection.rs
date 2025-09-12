use reqwest::{Client, Response};
use crate::api::{Numerologie, ThemeNumerologie};
use crate::api::numerologie::HtmlNBR;
use crate::TNumerologieClient;

pub trait TraitSelectionThemeNumerologie {
    fn new_sans_cartes(numerologie: Numerologie, token: String) -> Self
    where
        Self: Sized;
}

pub trait TraitSelectionNumerologie {
    fn new_sans_cartes(token_n: String, token_t: String) -> Self
    where
        Self: Sized;
    async fn get_index_sans_cartes(&self, id: u32) -> Result<ThemeNumerologie, reqwest::Error>;
}
impl TraitSelectionNumerologie for TNumerologieClient {
    fn new_sans_cartes(token_n: String, token_t: String) -> Self {
        Self {
            base_url: "https://t.bressani.dev:1178".to_string(),
            path_cartes: "".to_string(),
            token_n,
            token_t,
        }
    }

    async fn get_index_sans_cartes(&self, id: u32) -> Result<ThemeNumerologie, reqwest::Error> {
        let url = format!("{}/api/numerologie/{}", self.base_url, id);
        let client = Client::new();
        let resp: Response = client
            .get(&url)
            .bearer_auth(&self.token_t)
            .send()
            .await?
            .error_for_status()?; // transforme les réponses 4xx/5xx en erreur

        let numerologie: Numerologie = resp.json().await?;
        Ok(ThemeNumerologie::new_sans_cartes(numerologie, self.token_n.as_str().to_string()))
    }

}
impl TraitSelectionThemeNumerologie for ThemeNumerologie {
    fn new_sans_cartes(numerologie: Numerologie, token: String) -> Self {
        Self {
            base_url: "https://numerologie.bressani.dev:1122".to_string(),
            numerologie,
            token,
            path_cartes: "".to_string(),
            cai_lame: None,
            cai_mots_cles: vec![],
            cai_carte: vec![],
            cai_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            cai_aspects: vec![],
            cae_lame: None,
            cae_mots_cles: vec![],
            cae_carte: vec![],
            cae_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            cae_aspects: vec![],
            int_lame: None,
            int_mots_cles: vec![],
            int_carte: vec![],
            int_html: "".to_string(),
            coi_lame: None,
            coi_mots_cles: vec![],
            coi_carte: vec![],
            coi_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            coi_aspects: vec![],
            coe_lame: None,
            coe_mots_cles: vec![],
            coe_carte: vec![],
            coe_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            coe_aspects: vec![],
            nem_lame: None,
            nem_mots_cles: vec![],
            nem_carte: vec![],
            nem_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            nem_aspects: vec![],
            pex_lame: None,
            pex_mots_cles: vec![],
            pex_carte: vec![],
            pex_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            pex_aspects: vec![],
            ppr_lame: None,
            ppr_mots_cles: vec![],
            ppr_carte: vec![],
            ppr_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            ppr_aspects: vec![],
        }
    }
}
