use serde::Deserialize;
use reqwest::{Client, Response};
use crate::api::{LameMajeureDetail, NumerologieCaractereIntime};
use crate::core_docx::{ColorEnum, NumerologieAspects};
use crate::html_tools::extract_supers_and_bold_and_italic;

pub struct ThemeNumerologie {
    base_url: String,
    pub numerologie: Numerologie,
    pub token: String,
    path_cartes: String,
    pub cai_aspects: Vec<NumerologieAspects>,
}

pub enum TraitementNumerologie {
    Cai,
    Cae,
    Int,
    Coi,
    Coe,
    Nem,
    Pex,
    Ppr,
    Rha,
}

struct ComposeAspect {
    cai_aspects_b: Vec<String>,
    cai_aspects_r: Vec<String>,
    cai_bold_aspects: Vec<String>,
}

pub const SW_DEBUG: bool = false;

impl ThemeNumerologie {
    pub fn new(numerologie: Numerologie, token: String) -> Self {
        Self {
            base_url: "https://numerologie.bressani.dev:1122".to_string(),
            numerologie,
            token,
            path_cartes: "/Users/stephane/Code/rust/ref/theme_numerologie_docx/images/TAROT-GRIMAUD".to_string(), // TODO later
            cai_aspects: vec![],
        }
    }

    /// get_cai permet de charger les données Cai
    pub async fn get_cai(&mut self, carte: u32) -> Result<(i32, LameMajeureDetail, Option<NumerologieCaractereIntime>), reqwest::Error> {
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
                Ok(detail) => println!("Deserialized : {:?}", detail),
                Err(e) => println!("Erreur de désérialisation : {}", e),
            }
            eprintln!("Debug");
            std::process::exit(1);
        } else {
            let lame_majeure_detail: LameMajeureDetail = resp.json().await?;
            let cai = lame_majeure_detail.numerologie_caractere_intime.clone();
            let html: String = cai
                .as_ref() // convertit Option<T> en Option<&T>
                .map(|c| c.html_body_one_note_raw.clone())
                .unwrap_or_else(|| "".to_string());
            let html_b: String = cai
                .as_ref() // convertit Option<T> en Option<&T>
                .map(|c| c.html_body_one_note_raw_b.clone())
                .unwrap_or_else(|| "".to_string());
            let html_r: String = cai
                .as_ref() // convertit Option<T> en Option<&T>
                .map(|c| c.html_body_one_note_raw_r.clone())
                .unwrap_or_else(|| "".to_string());
            self.compute_aspect(TraitementNumerologie::Cai, &lame_majeure_detail, html, html_b, html_r);
            Ok((self.numerologie.interpretation_cai, lame_majeure_detail, cai))
        }
    }

    pub async fn get_carte(&self, id: i32) -> Result<Vec<u8>, std::io::Error> {
        let file_name = format!("{}.jpg", id);
        let path = std::path::Path::new(&self.path_cartes).join(file_name);
        let data = tokio::fs::read(path).await?;
        Ok(data)
    }

    /// compute_aspect permet de préparer les champs d'aspects:
    /// self.cai_aspects
    /// et ainsi de suite... // TODO à faire
    fn compute_aspect(&mut self, type_traitement: TraitementNumerologie, lame_majeur_detail: &LameMajeureDetail, _: String, html_b: String, html_r: String) {
        let mut cai_aspects_b: Vec<String> = vec![];
        let mut cai_aspects_r: Vec<String> = vec![];
        let mut bold_aspects: Vec<String> = vec![];
        bold_aspects = lame_majeur_detail.numerologie_aspects.as_slice()
            .iter()
            .filter(|x| {
                x.sw_bold && x.nom.clone().is_some()
            })
            .map(|x| {
                x.nom.clone().unwrap()
            })
            .collect();
        // (_, _) = html_tools::extract_supers_and_bold_and_italic(&text.html_body_one_note_raw.as_str());
        (_, cai_aspects_b) = extract_supers_and_bold_and_italic(html_b.as_str());
        (_, cai_aspects_r) = extract_supers_and_bold_and_italic(html_r.as_str());
        let mut traitement = vec![];
        traitement = cai_aspects_b.into_iter().map(|x| {
            let mut find = false;
            for y in bold_aspects.as_slice().iter() {
                if x == *y {
                    find = true;
                    break;
                }
            }
            NumerologieAspects {
                aspect: x,
                color: ColorEnum::Bleu,
                sw_bold: find,
            }
        }).collect();
        traitement.extend(
            cai_aspects_r
                .into_iter()
                .map(|x| {
                    let sw_bold = bold_aspects.iter().any(|y| x == *y);
                    NumerologieAspects {
                        aspect: x,
                        color: ColorEnum::Rouge,
                        sw_bold,
                    }
                })
        );
        match type_traitement {
            TraitementNumerologie::Cai => {
                self.cai_aspects = traitement
            }
            TraitementNumerologie::Cae => {}
            TraitementNumerologie::Int => {}
            TraitementNumerologie::Coi => {}
            TraitementNumerologie::Coe => {}
            TraitementNumerologie::Nem => {}
            TraitementNumerologie::Pex => {}
            TraitementNumerologie::Ppr => {}
            TraitementNumerologie::Rha => {}
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
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

    pub async fn get_index(&self, id: u32) -> Result<ThemeNumerologie, reqwest::Error> {
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