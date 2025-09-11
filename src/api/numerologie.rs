use serde::Deserialize;
use reqwest::{Client, Response};
use crate::api::{LameMajeureDetail, NumerologieCaractereIntime, NumerologieCaractereSocial, NumerologieComportementIntime, NumerologieComportementSocial, NumerologieIntellect, NumerologieNoeudEmotionnel, NumerologiePersonaliteExterieure, NumerologiePersonaliteProfonde};
use crate::core_docx::{ColorEnum, NumerologieAspects};
use crate::html_tools::extract_supers_and_bold_and_italic;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug)]
pub struct ThemeNumerologie {
    base_url: String,
    pub numerologie: Numerologie,
    pub token: String,
    path_cartes: String,
    lame_majeure_detail: LameMajeureDetail,
    pub cai_html: HtmlNBR,
    pub cai_aspects: Vec<NumerologieAspects>,
    pub cae_html: HtmlNBR,
    pub cae_aspects: Vec<NumerologieAspects>,
    pub int_html: String,
    pub coi_html: HtmlNBR,
    pub coi_aspects: Vec<NumerologieAspects>,
    pub coe_html: HtmlNBR,
    pub coe_aspects: Vec<NumerologieAspects>,
    pub nem_html: HtmlNBR,
    pub nem_aspects: Vec<NumerologieAspects>,
    pub pex_html: HtmlNBR,
    pub pex_aspects: Vec<NumerologieAspects>,
    pub ppr_html: HtmlNBR,
    pub ppr_aspects: Vec<NumerologieAspects>,
}

#[derive(Clone, Debug, EnumIter)]
pub enum TraitementNumerologie {
    Cai,
    Cae,
    Int,
    Coi,
    Coe,
    Nem,
    Pex,
    Ppr,
}

#[derive(Clone, Debug)]
pub struct HtmlNBR {
    pub html: String,
    pub html_b: String,
    pub html_r: String,
}

#[derive(Clone, Debug)]
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
            lame_majeure_detail: LameMajeureDetail {
                id: 0,
                nombre: 0,
                nombre_romain: "".to_string(),
                nom: None,
                cartouche_grimaud: None,
                lame_majeures_divination: None,
                lame_majeures_divination_detail: None,
                lame_majeures_divination_principes: None,
                divination_associations_classiques: vec![],
                divination_personnages: vec![],
                lame_majeures_numerologie: None,
                numerologie_aspects: vec![],
                numerologie_mots_cle: vec![],
                numerologie_note_de_cours: vec![],
                numerologie_message_karmique: None,
                numerologie_personalite_profonde: None,
                numerologie_caractere_intime: None,
                numerologie_intellect: None,
                numerologie_caractere_social: None,
                numerologie_noeud_emotionnel: None,
                numerologie_comportement_intime: None,
                numerologie_comportement_social: None,
                numerologie_personalite_exterieure: None,
                numerologie_vocabulaire_divers: vec![],
            },
            cai_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            cai_aspects: vec![],
            cae_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            cae_aspects: vec![],
            int_html: "".to_string(),
            coi_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            coi_aspects: vec![],
            coe_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            coe_aspects: vec![],
            nem_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            nem_aspects: vec![],
            pex_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            pex_aspects: vec![],
            ppr_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            ppr_aspects: vec![],
        }
    }

    /// get_cai permet de charger les données Cai
    pub async fn get_cai(&mut self, carte: u32) -> Result<(i32, LameMajeureDetail), reqwest::Error> {
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
            self.compute_html_and_aspect(lame_majeure_detail);
            Ok((self.numerologie.interpretation_cai, self.get_lame_majeure_detail()))
        }
    }

    pub async fn get_carte(&self, id: i32) -> Result<Vec<u8>, std::io::Error> {
        let file_name = format!("{}.jpg", id);
        let path = std::path::Path::new(&self.path_cartes).join(file_name);
        let data = tokio::fs::read(path).await?;
        Ok(data)
    }

    pub fn get_lame_majeure_detail(&self) -> LameMajeureDetail {
        self.lame_majeure_detail.clone()
    }

    /// La fonction: compute_html_and_aspect
    /// - Permet de préparer les champs en fonction du type de traitement
    fn compute_html_and_aspect(&mut self, lame_majeur_detail: LameMajeureDetail) {
        self.lame_majeure_detail = lame_majeur_detail;
        let mut cai: Option<NumerologieCaractereIntime> = None;
        let mut cae: Option<NumerologieCaractereSocial> = None;
        let mut int: Option<NumerologieIntellect> = None;
        let mut coi: Option<NumerologieComportementIntime> = None;
        let mut coe: Option<NumerologieComportementSocial> = None;
        let mut nem: Option<NumerologieNoeudEmotionnel> = None;
        let mut pex: Option<NumerologiePersonaliteExterieure> = None;
        let mut ppr: Option<NumerologiePersonaliteProfonde> = None;
        TraitementNumerologie::iter().for_each(|x| {
            let mut html: String = "".to_string();
            let mut html_b: String = "".to_string();
            let mut html_r: String = "".to_string();
            match x.clone() {
                TraitementNumerologie::Cai => {
                    cai = self.lame_majeure_detail.numerologie_caractere_intime.clone();
                    html = cai
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_b = cai
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = cai
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());
                }
                TraitementNumerologie::Cae => {
                    cae = self.lame_majeure_detail.numerologie_caractere_social.clone();
                    html = cae
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    /*
                    html_b = cae
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = cae
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());*/
                }
                TraitementNumerologie::Int => {
                    int = self.lame_majeure_detail.numerologie_intellect.clone();
                    html = int
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                }
                TraitementNumerologie::Coi => {
                    coi = self.lame_majeure_detail.numerologie_comportement_intime.clone();
                    html = coi
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    /*
                    html_b = coi
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = coi
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());*/
                }
                TraitementNumerologie::Coe => {
                    coe = self.lame_majeure_detail.numerologie_comportement_social.clone();
                    html = coe
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    /*
                    html_b = coe
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = coe
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());*/
                }
                TraitementNumerologie::Nem => {
                    nem = self.lame_majeure_detail.numerologie_noeud_emotionnel.clone();
                    html = nem
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    /*
                    html_b = nem
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = nem
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());*/
                }
                TraitementNumerologie::Pex => {
                    pex = self.lame_majeure_detail.numerologie_personalite_exterieure.clone();
                    html = pex
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    /*
                    html_b = pex
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = pex
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());*/
                }
                TraitementNumerologie::Ppr => {
                    ppr = self.lame_majeure_detail.numerologie_personalite_profonde.clone();
                    html = ppr
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    /*
                    html_b = ppr
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = ppr
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());*/
                }
            }
            let mut aspects_b: Vec<String> = vec![];
            let mut aspects_r: Vec<String> = vec![];
            let mut bold_aspects: Vec<String> = vec![];
            bold_aspects = self.lame_majeure_detail.numerologie_aspects.as_slice()
                .iter()
                .filter(|x| {
                    x.sw_bold && x.nom.clone().is_some()
                })
                .map(|x| {
                    x.nom.clone().unwrap()
                })
                .collect();
            (html, _) = extract_supers_and_bold_and_italic(html.as_str());
            (html_b, aspects_b) = extract_supers_and_bold_and_italic(html_b.as_str());
            (html_r, aspects_r) = extract_supers_and_bold_and_italic(html_r.as_str());
            let mut traitement = vec![];
            traitement = aspects_b.into_iter().map(|x| {
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
                aspects_r
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
            match x.clone() {
                TraitementNumerologie::Cai => {
                    self.cai_html = HtmlNBR {
                        html,
                        html_b,
                        html_r,
                    };
                    self.cai_aspects = traitement;
                }
                TraitementNumerologie::Cae => {
                    self.cae_html = HtmlNBR {
                        html,
                        html_b,
                        html_r,
                    };
                    self.cae_aspects = traitement;
                }
                TraitementNumerologie::Int => {
                    self.int_html = html;
                }
                TraitementNumerologie::Coi => {
                    self.coi_html = HtmlNBR {
                        html,
                        html_b,
                        html_r,
                    };
                    self.coi_aspects = traitement;
                }
                TraitementNumerologie::Coe => {
                    self.coe_html = HtmlNBR {
                        html,
                        html_b,
                        html_r,
                    };
                    self.coe_aspects = traitement;
                }
                TraitementNumerologie::Nem => {
                    self.nem_html = HtmlNBR {
                        html,
                        html_b,
                        html_r,
                    };
                    self.nem_aspects = traitement;
                }
                TraitementNumerologie::Pex => {
                    self.pex_html = HtmlNBR {
                        html,
                        html_b,
                        html_r,
                    };
                    self.pex_aspects = traitement;
                }
                TraitementNumerologie::Ppr => {
                    self.ppr_html = HtmlNBR {
                        html,
                        html_b,
                        html_r,
                    };
                    self.ppr_aspects = traitement;
                }
            }
        });
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