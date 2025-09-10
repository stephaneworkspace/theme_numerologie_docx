use bytes::Bytes;
use serde::Deserialize;
use reqwest::{Client, Error, Response};
use reqwest::header::ACCEPT;
use crate::api::{LameMajeureDetail, NumerologieCaractereIntime};

pub struct ThemeNumerologie {
    base_url: String,
    pub numerologie: Numerologie,
    pub token: String,
    path_cartes: String
}
pub const SW_DEBUG: bool = false;

impl ThemeNumerologie {
    pub fn new(numerologie: Numerologie, token: String) -> Self {
        Self {
            base_url: "https://numerologie.bressani.dev:1122".to_string(),
            numerologie,
            token,
            path_cartes: "/Users/stephane/Code/rust/ref/theme_numerologie_docx/images/TAROT-GRIMAUD".to_string(), // TODO later
        }
    }

    // Personalité profonde
    pub async fn get_cai(&self, carte: u32) ->  Result<(&i32, LameMajeureDetail, Option<NumerologieCaractereIntime>), reqwest::Error> {
        let url = format!("{}/api/lame_majeures/{}", self.base_url, carte);
        let client = Client::new();
        let resp: Response =
            client
                .get(&url)
                .bearer_auth(&self.token)
                .send()
                .await?
                .error_for_status()?;
        if SW_DEBUG {
            let body: String = resp.text().await?;
            //println!("{}", body);
            let lame_majeure_detail: Result<LameMajeureDetail, serde_json::Error> = serde_json::from_str(&body);
            match lame_majeure_detail {
                Ok(detail) => println!("Deserialized: {:?}", detail),
                Err(e) => println!("Erreur de désérialisation: {}", e),
            }
            eprintln!("Debug");
            std::process::exit(1);
        } else {
            let lame_majeure_detail: LameMajeureDetail = resp.json().await?;
            let cai = lame_majeure_detail.numerologie_caractere_intime.clone();
            Ok((&self.numerologie.interpretation_cai, lame_majeure_detail, cai))
        }
    }

    pub async fn get_carte(&self, id: i32) -> Result<Vec<u8>, std::io::Error> {
        let file_name = format!("{}.jpg", id);
        let path = std::path::Path::new(&self.path_cartes).join(file_name);
        let data = tokio::fs::read(path).await?;
        Ok(data)
        /* TOO SLOW
        let url = format!("{}/api/cartes/{}", self.base_url, id);
        let client = Client::new();
        let resp: Response = client
            .get(&url)
            .header(ACCEPT, "image/jpg")
            .query(&[("jeu", "GRIMAUD_C")])
            .bearer_auth(&self.token)
            .send()
            .await?
            .error_for_status()?; // transforme les réponses 4xx/5xx en erreur
        Ok(resp.bytes().await?)*/
    }
}

#[derive(Debug, Deserialize)]
pub struct Numerologie {
    pub id: i32,
    pub numerologie_type: i32,
    pub resume_rapide: String,
    pub text: String,
    pub png_b64: String,
    pub png_simple_b64: String,
    pub jour: i32,
    pub mois: i32,
    pub annee: i32,
    pub interpretation_cae: i32,
    pub interpretation_01_cae_i: String,
    pub interpretation_cai: i32,
    pub interpretation_01_cai_i: String,
    pub interpretation_coe: i32,
    pub interpretation_01_coe_i: String,
    pub interpretation_coi: i32,
    pub interpretation_01_coi_i: String,
    pub interpretation_int: i32,
    pub interpretation_01_int_i: String,
    pub interpretation_nem: i32,
    pub interpretation_01_nem_i: String,
    pub interpretation_pex: i32,
    pub interpretation_01_pex_i: String,
    pub interpretation_ppr: i32,
    pub interpretation_01_ppr_i: String,
    pub interpretation_rha: i32,
    pub interpretation_01_rha_i: String,
}

#[derive(Clone)]
pub struct TNumerologieClient {
    base_url: String,
    token_n: String,
    token_t: String,
}

impl TNumerologieClient {
    pub fn new(token_n: String, token_t: String) -> Self {
        Self {
            base_url: "https://t.bressani.dev:1178".to_string(),
            token_n,
            token_t,
        }
    }

    pub async fn get_index(&self, id: i32) -> Result<ThemeNumerologie, reqwest::Error> {
        let url = format!("{}/api/numerologie/{}", self.base_url, id);
        let client = Client::new();
        let resp: Response = client
            .get(&url)
            .bearer_auth(&self.token_t)
            .send()
            .await?
            .error_for_status()?; // transforme les réponses 4xx/5xx en erreur

        let numerologie: Numerologie = resp.json().await?;
        Ok(ThemeNumerologie::new(numerologie, self.token_n.as_str().to_string()))
    }
}