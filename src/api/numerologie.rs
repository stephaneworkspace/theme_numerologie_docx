use std::fs;
use serde::Deserialize;
use reqwest::{Client, Response};
use crate::api::{LameMajeureDetail, NumerologieCaractereIntime, NumerologieCaractereSocial, NumerologieComportementIntime, NumerologieComportementSocial, NumerologieIntellect, NumerologieMotCle, NumerologieNoeudEmotionnel, NumerologiePersonaliteExterieure, NumerologiePersonaliteProfonde};
use crate::core_docx::{ColorEnum, NumerologieAspects};
use crate::html_tools::extract_supers_and_bold_and_italic;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::api::trait_selection::TraitSelectionThemeNumerologie;

#[derive(Clone, Debug)]
pub struct ThemeNumerologie {
    pub base_url: String,
    pub numerologie: Numerologie,
    pub token: String,
    pub path_cartes: String,
    pub cai_lame: Option<LameMajeureDetail>,
    pub cai_mots_cles: Vec<(ColorEnum, String)>,
    pub cai_carte: Vec<u8>,
    pub cai_html: HtmlNBR,
    pub cai_aspects: Vec<NumerologieAspects>,
    pub cae_lame: Option<LameMajeureDetail>,
    pub cae_mots_cles: Vec<(ColorEnum, String)>,
    pub cae_carte: Vec<u8>,
    pub cae_html: HtmlNBR,
    pub cae_aspects: Vec<NumerologieAspects>,
    pub int_lame: Option<LameMajeureDetail>,
    pub int_mots_cles: Vec<(ColorEnum, String)>,
    pub int_carte: Vec<u8>,
    pub int_html: String,
    pub coi_lame: Option<LameMajeureDetail>,
    pub coi_mots_cles: Vec<(ColorEnum, String)>,
    pub coi_carte: Vec<u8>,
    pub coi_html: HtmlNBR,
    pub coi_aspects: Vec<NumerologieAspects>,
    pub coe_lame: Option<LameMajeureDetail>,
    pub coe_mots_cles: Vec<(ColorEnum, String)>,
    pub coe_carte: Vec<u8>,
    pub coe_html: HtmlNBR,
    pub coe_aspects: Vec<NumerologieAspects>,
    pub nem_lame: Option<LameMajeureDetail>,
    pub nem_mots_cles: Vec<(ColorEnum, String)>,
    pub nem_carte: Vec<u8>,
    pub nem_html: HtmlNBR,
    pub nem_aspects: Vec<NumerologieAspects>,
    pub pex_lame: Option<LameMajeureDetail>,
    pub pex_mots_cles: Vec<(ColorEnum, String)>,
    pub pex_carte: Vec<u8>,
    pub pex_html: HtmlNBR,
    pub pex_aspects: Vec<NumerologieAspects>,
    pub ppr_lame: Option<LameMajeureDetail>,
    pub ppr_mots_cles: Vec<(ColorEnum, String)>,
    pub ppr_carte: Vec<u8>,
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

// pub const SW_DEBUG: bool = false;

impl ThemeNumerologie {
    pub fn new(numerologie: Numerologie, token: String, path_cartes: String) -> Self {
        Self {
            base_url: "https://numerologie.bressani.dev:1122".to_string(),
            numerologie,
            token,
            path_cartes,
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

    /*pub async fn get_cai(&mut self, carte: u32) -> Result<(i32, LameMajeureDetail), reqwest::Error> {
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
    }*/

    /// Calcul du tout pour traitement docx-rs
    pub async fn get_all(&mut self) -> Result<(()), reqwest::Error> {
        for x in TraitementNumerologie::iter() {
            let carte: u32 = match &x  {
                TraitementNumerologie::Cai => {
                    self.numerologie.interpretation_cai
                },
                TraitementNumerologie::Cae => {
                    self.numerologie.interpretation_cae
                }
                TraitementNumerologie::Int => {
                    self.numerologie.interpretation_int
                }
                TraitementNumerologie::Coi => {
                    self.numerologie.interpretation_coi
                }
                TraitementNumerologie::Coe => {
                    self.numerologie.interpretation_coe
                }
                TraitementNumerologie::Nem => {
                    self.numerologie.interpretation_nem
                }
                TraitementNumerologie::Pex => {
                    self.numerologie.interpretation_pex
                }
                TraitementNumerologie::Ppr => {
                    self.numerologie.interpretation_ppr
                }
            };
            let url = format!("{}/api/lame_majeures/{}", self.base_url, carte);
            let client = Client::new();
            let resp: Response =
                client
                    .get(&url)
                    .bearer_auth(&self.token)
                    .send()
                    .await?
                    .error_for_status()?;
            match &x {
                TraitementNumerologie::Cai => {
                    self.cai_lame = Some(resp.json().await?);
                }
                TraitementNumerologie::Cae => {
                    self.cae_lame = Some(resp.json().await?);
                }
                TraitementNumerologie::Int => {
                    self.int_lame = Some(resp.json().await?);
                }
                TraitementNumerologie::Coi => {
                    self.coi_lame = Some(resp.json().await?);
                }
                TraitementNumerologie::Coe => {
                    self.coe_lame = Some(resp.json().await?);
                }
                TraitementNumerologie::Nem => {
                    self.nem_lame = Some(resp.json().await?);
                }
                TraitementNumerologie::Pex => {
                    self.pex_lame = Some(resp.json().await?);
                }
                TraitementNumerologie::Ppr => {
                    self.ppr_lame = Some(resp.json().await?);
                }
            }
        }
        self.compute_html_and_aspect_data();
        Ok(())
    }

    /// Traitement pour selection pour text pure rust pour Swift
    pub async fn get_traitement(&self, traitement: TraitementNumerologie) -> Result<(()), reqwest::Error> {
        let carte: u32 = match &traitement  {
            TraitementNumerologie::Cai => {
                self.numerologie.interpretation_cai
            },
            TraitementNumerologie::Cae => {
                self.numerologie.interpretation_cae
            }
            TraitementNumerologie::Int => {
                self.numerologie.interpretation_int
            }
            TraitementNumerologie::Coi => {
                self.numerologie.interpretation_coi
            }
            TraitementNumerologie::Coe => {
                self.numerologie.interpretation_coe
            }
            TraitementNumerologie::Nem => {
                self.numerologie.interpretation_nem
            }
            TraitementNumerologie::Pex => {
                self.numerologie.interpretation_pex
            }
            TraitementNumerologie::Ppr => {
                self.numerologie.interpretation_ppr
            }
        };
        let url = format!("{}/api/lame_majeures/{}", self.base_url, carte);
        let client = Client::new();
        let resp: Response =
            client
                .get(&url)
                .bearer_auth(&self.token)
                .send()
                .await?
                .error_for_status()?;
        let lame: Option<LameMajeureDetail> = match &traitement {
            TraitementNumerologie::Cai => {
                Some(resp.json().await?)
            }
            TraitementNumerologie::Cae => {
                Some(resp.json().await?)
            }
            TraitementNumerologie::Int => {
                Some(resp.json().await?)
            }
            TraitementNumerologie::Coi => {
                Some(resp.json().await?)
            }
            TraitementNumerologie::Coe => {
                Some(resp.json().await?)
            }
            TraitementNumerologie::Nem => {
                Some(resp.json().await?)
            }
            TraitementNumerologie::Pex => {
                Some(resp.json().await?)
            }
            TraitementNumerologie::Ppr => {
                Some(resp.json().await?)
            }
        };
        let html_lame: String = match &traitement {
            TraitementNumerologie::Cai => {
                lame.unwrap().numerologie_caractere_intime.unwrap().html_body_one_note_raw
            }
            TraitementNumerologie::Cae => {
                lame.unwrap().numerologie_caractere_intime.unwrap().html_body_one_note_raw
            }
            TraitementNumerologie::Int => {
                lame.unwrap().numerologie_caractere_intime.unwrap().html_body_one_note_raw
            }
            TraitementNumerologie::Coi => {
                lame.unwrap().numerologie_caractere_intime.unwrap().html_body_one_note_raw
            }
            TraitementNumerologie::Coe => {
                lame.unwrap().numerologie_caractere_intime.unwrap().html_body_one_note_raw
            }
            TraitementNumerologie::Nem => {
                lame.unwrap().numerologie_caractere_intime.unwrap().html_body_one_note_raw
            }
            TraitementNumerologie::Pex => {
                lame.unwrap().numerologie_caractere_intime.unwrap().html_body_one_note_raw
            }
            TraitementNumerologie::Ppr => {
                lame.unwrap().numerologie_caractere_intime.unwrap().html_body_one_note_raw
            }
        };
        let res = extract_supers_and_bold_and_italic(html_lame.as_str());
        println!("{}",res.0);
        Ok(())
    }

    fn get_carte(&self, id: u32) -> Result<Vec<u8>, String> {
        //std::io::Error> {
        let file_name = format!("{}.jpg", id);
        let path = std::path::Path::new(&self.path_cartes).join(file_name);
        let data = fs::read(&path);
        match data {
            Ok(ok) => Ok(ok),
            Err(e) => Err(format!("Mauvais path: {:?} - {:?}", path.as_path().to_str(), e))
        }
    }

    /// La fonction est privée: compute_html_and_aspect
    /// - Permet de préparer les champs en fonction du type de traitement
    fn compute_html_and_aspect_data(&mut self) {
        let mut lame_majeure_detail: Option<LameMajeureDetail> = None;
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
                    cai = self.cai_lame.clone().unwrap().numerologie_caractere_intime.clone(); // TODO
                    lame_majeure_detail = self.cai_lame.clone();
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
                    cae = self.cae_lame.clone().unwrap().numerologie_caractere_social.clone();
                    lame_majeure_detail = self.cae_lame.clone();
                    html = cae
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_b = cae
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = cae
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());
                }
                TraitementNumerologie::Int => {
                    int = self.int_lame.clone().unwrap().numerologie_intellect.clone();
                    lame_majeure_detail = self.int_lame.clone();
                    html = int
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                }
                TraitementNumerologie::Coi => {
                    coi = self.coi_lame.clone().unwrap().numerologie_comportement_intime.clone(); // TODO
                    lame_majeure_detail = self.coi_lame.clone();
                    html = coi
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_b = coi
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = coi
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());
                }
                TraitementNumerologie::Coe => {
                    coe = self.coe_lame.clone().unwrap().numerologie_comportement_social.clone();
                    lame_majeure_detail = self.coe_lame.clone();
                    html = coe
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_b = coe
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = coe
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());
                }
                TraitementNumerologie::Nem => {
                    nem = self.nem_lame.clone().unwrap().numerologie_noeud_emotionnel.clone();
                    lame_majeure_detail = self.nem_lame.clone();
                    html = nem
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_b = nem
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = nem
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());
                }
                TraitementNumerologie::Pex => {
                    pex = self.pex_lame.clone().unwrap().numerologie_personalite_exterieure.clone();
                    lame_majeure_detail = self.pex_lame.clone();
                    html = pex
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_b = pex
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = pex
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());
                }
                TraitementNumerologie::Ppr => {
                    ppr = self.ppr_lame.clone().unwrap().numerologie_personalite_profonde.clone();
                    lame_majeure_detail = self.ppr_lame.clone();
                    html = ppr
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_b = ppr
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_b.clone())
                        .unwrap_or_else(|| "".to_string());
                    html_r = ppr
                        .as_ref() // convertit Option<T> en Option<&T>
                        .map(|c| c.html_body_one_note_raw_r.clone())
                        .unwrap_or_else(|| "".to_string());
                }
            }
            let mut aspects_b: Vec<String> = vec![];
            let mut aspects_r: Vec<String> = vec![];
            let mut bold_aspects: Vec<String> = vec![];
            bold_aspects = lame_majeure_detail.clone().unwrap().numerologie_aspects.as_slice()
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
            let mut traitement_aspects = vec![];
            traitement_aspects = aspects_b.into_iter().map(|x| {
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
            traitement_aspects.extend(
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
                    let (mots_cles, html_struct) = traiter_lame(
                        &self.cai_lame,
                        |id| self.get_carte(id),
                        html,
                        html_b,
                        html_r,
                    );
                    self.cai_mots_cles = mots_cles;
                    self.cai_carte = self.get_carte(self.cai_lame.as_ref().unwrap().id).unwrap(); // TODO intercepté le error IO
                    self.cai_html = html_struct;
                    self.cai_aspects = traitement_aspects;
                }
                TraitementNumerologie::Cae => {
                    let (mots_cles, html_struct) = traiter_lame(
                        &self.cae_lame,
                        |id| self.get_carte(id),
                        html,
                        html_b,
                        html_r,
                    );
                    self.cae_mots_cles = mots_cles;
                    self.cae_carte = self.get_carte(self.cae_lame.as_ref().unwrap().id).unwrap(); // TODO intercepté le error IO
                    self.cae_html = html_struct;
                    self.cae_aspects = traitement_aspects;
                }
                TraitementNumerologie::Int => {
                    let (mots_cles, html_struct) = traiter_lame(
                        &self.int_lame,
                        |id| self.get_carte(id),
                        html,
                        html_b,
                        html_r,
                    );
                    self.int_mots_cles = mots_cles;
                    self.int_carte = self.get_carte(self.int_lame.as_ref().unwrap().id).unwrap(); // TODO intercepté le error IO
                    self.int_html = html_struct.html;
                }
                TraitementNumerologie::Coi => {
                    let (mots_cles, html_struct) = traiter_lame(
                        &self.coi_lame,
                        |id| self.get_carte(id),
                        html,
                        html_b,
                        html_r,
                    );
                    self.coi_mots_cles = mots_cles;
                    self.coi_carte = self.get_carte(self.coi_lame.as_ref().unwrap().id).unwrap(); // TODO intercepté le error IO
                    self.coi_html = html_struct;
                    self.coi_aspects = traitement_aspects;
                }
                TraitementNumerologie::Coe => {
                    let (mots_cles, html_struct) = traiter_lame(
                        &self.coe_lame,
                        |id| self.get_carte(id),
                        html,
                        html_b,
                        html_r,
                    );
                    self.coe_mots_cles = mots_cles;
                    self.coe_carte = self.get_carte(self.coe_lame.as_ref().unwrap().id).unwrap(); // TODO intercepté le error IO
                    self.coe_html = html_struct;
                    self.coe_aspects = traitement_aspects;
                }
                TraitementNumerologie::Nem => {
                    let (mots_cles, html_struct) = traiter_lame(
                        &self.nem_lame,
                        |id| self.get_carte(id),
                        html,
                        html_b,
                        html_r,
                    );
                    self.nem_mots_cles = mots_cles;
                    self.nem_carte = self.get_carte(self.nem_lame.as_ref().unwrap().id).unwrap(); // TODO intercepté le error IO
                    self.nem_html = html_struct;
                    self.nem_aspects = traitement_aspects;
                }
                TraitementNumerologie::Pex => {
                    let (mots_cles, html_struct) = traiter_lame(
                        &self.pex_lame,
                        |id| self.get_carte(id),
                        html,
                        html_b,
                        html_r,
                    );
                    self.pex_mots_cles = mots_cles;
                    self.pex_carte = self.get_carte(self.pex_lame.as_ref().unwrap().id).unwrap(); // TODO intercepté le error IO
                    self.pex_html = html_struct;
                    self.pex_aspects = traitement_aspects;
                }
                TraitementNumerologie::Ppr => {
                    let (mots_cles, html_struct) = traiter_lame(
                        &self.ppr_lame,
                        |id| self.get_carte(id),
                        html,
                        html_b,
                        html_r,
                    );
                    self.ppr_mots_cles = mots_cles;
                    self.ppr_carte = self.get_carte(self.ppr_lame.as_ref().unwrap().id).unwrap(); // TODO intercepté le error IO
                    self.ppr_html = html_struct;
                    self.ppr_aspects = traitement_aspects;
                }
            }
        });
    }
}

fn transformer_mots_cles(mots: &[NumerologieMotCle]) -> Vec<(ColorEnum, String)> {
    mots.iter()
        .map(|x| {
            let couleur = if x.polarite.as_deref() == Some("+") {
                ColorEnum::Bleu
            } else {
                ColorEnum::Rouge
            };
            (couleur, x.mot_cle.clone())
        })
        .collect()
}

fn traiter_lame(
    lame: &Option<LameMajeureDetail>,
    get_carte: impl Fn(u32) -> Result<Vec<u8>, String>,
    html: String,
    html_b: String,
    html_r: String,
) -> (Vec<(ColorEnum, String)>, HtmlNBR) {
    let mots_cles = transformer_mots_cles(&lame.as_ref().unwrap().numerologie_mots_cle);

    let carte = match get_carte(lame.as_ref().unwrap().id) {
        Ok(vu8) => vu8,
        Err(e) => {
            eprintln!("Erreur de traitement sur la carte: {}", e);
            std::process::exit(1);
        }
    };

    let html_struct = HtmlNBR { html, html_b, html_r };

    (mots_cles, html_struct)
}

#[derive(Debug, Deserialize, Clone)]
pub struct Numerologie {
    pub id: u32,
    pub numerologie_type: u32,
    pub resume_rapide: String,
    pub text: String,
    pub png_b64: String,
    pub png_simple_b64: String,
    pub jour: u32,
    pub mois: u32,
    pub annee: u32,
    pub interpretation_cae: u32,
    pub interpretation_01_cae_i: String,
    pub interpretation_cai: u32,
    pub interpretation_01_cai_i: String,
    pub interpretation_coe: u32,
    pub interpretation_01_coe_i: String,
    pub interpretation_coi: u32,
    pub interpretation_01_coi_i: String,
    pub interpretation_int: u32,
    pub interpretation_01_int_i: String,
    pub interpretation_nem: u32,
    pub interpretation_01_nem_i: String,
    pub interpretation_pex: u32,
    pub interpretation_01_pex_i: String,
    pub interpretation_ppr: u32,
    pub interpretation_01_ppr_i: String,
    pub interpretation_rha: u32,
    pub interpretation_01_rha_i: String,
}

#[derive(Clone)]
pub struct TNumerologieClient {
    pub base_url: String,
    pub path_cartes: String,
    pub token_n: String,
    pub token_t: String,
}

impl TNumerologieClient {
    pub fn new(token_n: String, token_t: String, path_cartes: String) -> Self {
        Self {
            base_url: "https://t.bressani.dev:1178".to_string(),
            path_cartes,
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
        Ok(ThemeNumerologie::new(numerologie, self.token_n.as_str().to_string(), self.path_cartes.clone()))
    }
}