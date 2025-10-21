/*
Trait pour un traitement dans exemples/selection.rs
---------------------------------------------------
Cet exemple permet de faire des traitements de texte
Donc rien avoir avec le nom original de ce crate,
c'est pour du traitement :
Html -> Balise spécial -> SwiftUi
 */
use chrono::{DateTime, Utc};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Map;
use strum_macros::EnumIter;
use crate::api::{LameMajeureDetail, Numerologie, NumerologieMotCle, ThemeNumerologie, TraitementNumerologie};
use crate::api::numerologie::HtmlNBR;
use crate::core_docx::{ColorEnum, NumerologieAspects};
use crate::html_tools::extract_supers_and_bold_and_italic;
use crate::TNumerologieClient;

pub trait TraitSelectionThemeNumerologie {
    fn new_sans_cartes(numerologie: Numerologie, token: String) -> Self
    where
        Self: Sized;
    fn selection_traitement_json(
        &self,
        traitement: TraitementNumerologie,
        carte: Option<u32>,
    ) -> impl std::future::Future<Output = Result<(String), reqwest::Error>> + Send;
}

pub trait TraitSelectionNumerologie {
    fn new_sans_cartes(token_n: String, token_t: String) -> Self
    where
        Self: Sized;
    fn get_index_sans_cartes(
        &self,
        id: u32,
    ) -> impl std::future::Future<Output = Result<ThemeNumerologie, reqwest::Error>> + Send;
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
            rha_lame: None,
            rha_mots_cles: vec![],
            rha_carte: vec![],
            rha_html: HtmlNBR {
                html: "".to_string(),
                html_b: "".to_string(),
                html_r: "".to_string(),
            },
            rha_aspects: vec![],
        }
    }
    async fn selection_traitement_json(&self, traitement: TraitementNumerologie, carte: Option<u32>) -> Result<(String), reqwest::Error> {
        let carte = if carte.is_some() {
            carte.unwrap()
        } else {
            match &traitement {
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
                TraitementNumerologie::Rha => {
                    self.numerologie.interpretation_rha
                }
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
            TraitementNumerologie::Rha => {
                Some(resp.json().await?)
            }
        };
        let l = &lame.unwrap();
        let lt_cai= &l.numerologie_caractere_intime;
        let lt_cae= &l.numerologie_caractere_social;
        let lt_int= &l.numerologie_intellect;
        let lt_coi= &l.numerologie_comportement_intime;
        let lt_coe= &l.numerologie_comportement_social;
        let lt_nem= &l.numerologie_noeud_emotionnel;
        let lt_ppr= &l.numerologie_personalite_profonde;
        let lt_pex= &l.numerologie_personalite_exterieure;
        let lt_rha= &l.numerologie_recherche_harmonie;
        let lv = &l.numerologie_vocabulaire_divers;
        let la = &l.numerologie_aspects;
        let (html_lame, html_lame_b, html_lame_r) = match &traitement {
            TraitementNumerologie::Cai => {
                (lt_cai.as_ref().unwrap().html_body_one_note_raw.clone(),
                 lt_cai.as_ref().unwrap().html_body_one_note_raw_b.clone(),
                 lt_cai.as_ref().unwrap().html_body_one_note_raw_r.clone())
            }
            TraitementNumerologie::Cae => {
                (lt_cae.as_ref().unwrap().html_body_one_note_raw.clone(),
                 lt_cae.as_ref().unwrap().html_body_one_note_raw_b.clone(),
                 lt_cae.as_ref().unwrap().html_body_one_note_raw_r.clone())
            }
            TraitementNumerologie::Coi => {
                (lt_coi.as_ref().unwrap().html_body_one_note_raw.clone(),
                 lt_coi.as_ref().unwrap().html_body_one_note_raw_b.clone(),
                 lt_coi.as_ref().unwrap().html_body_one_note_raw_r.clone())
            }
            TraitementNumerologie::Coe => {
                (lt_coe.as_ref().unwrap().html_body_one_note_raw.clone(),
                 lt_coe.as_ref().unwrap().html_body_one_note_raw_b.clone(),
                 lt_coe.as_ref().unwrap().html_body_one_note_raw_r.clone())
            }
            TraitementNumerologie::Nem => {
                (lt_nem.as_ref().unwrap().html_body_one_note_raw.clone(),
                 lt_nem.as_ref().unwrap().html_body_one_note_raw_b.clone(),
                 lt_nem.as_ref().unwrap().html_body_one_note_raw_r.clone())
            }
            TraitementNumerologie::Pex => {
                (lt_pex.as_ref().unwrap().html_body_one_note_raw.clone(),
                 lt_pex.as_ref().unwrap().html_body_one_note_raw_b.clone(),
                 lt_pex.as_ref().unwrap().html_body_one_note_raw_r.clone())
            }
            TraitementNumerologie::Ppr => {
                (lt_ppr.as_ref().unwrap().html_body_one_note_raw.clone(),
                 lt_ppr.as_ref().unwrap().html_body_one_note_raw_b.clone(),
                 lt_ppr.as_ref().unwrap().html_body_one_note_raw_r.clone())
            }
            TraitementNumerologie::Rha => {
                (lt_rha.as_ref().unwrap().html_body_one_note_raw.clone(),
                 lt_rha.as_ref().unwrap().html_body_one_note_raw_b.clone(),
                 lt_rha.as_ref().unwrap().html_body_one_note_raw_r.clone())
            },
            TraitementNumerologie::Int => {
                (lt_int.as_ref().unwrap().html_body_one_note_raw.clone(),
                 "".to_string(),
                 "".to_string())
            }
        };
        let res = extract_supers_and_bold_and_italic(html_lame.as_str(), false);
        let res_b = extract_supers_and_bold_and_italic(html_lame_b.as_str(), false);
        let res_r = extract_supers_and_bold_and_italic(html_lame_r.as_str(), false);
        let mut bold_aspects: Vec<String> = vec![];
        bold_aspects = l.clone().numerologie_aspects.as_slice()
            .iter()
            .filter(|x| {
                x.sw_bold && x.nom.clone().is_some()
            })
            .map(|x| {
                x.nom.clone().unwrap()
            })
            .collect();
        let mut ndc_all_aspects_b: Vec<String> = vec![];
        ndc_all_aspects_b = l.clone().numerologie_aspects.as_slice()
            .iter()
            .filter(|x| {
                x.polarite.as_deref() == Some("+") && x.nom.clone().is_some()
            })
            .map(|x| {
                x.nom.clone().unwrap()
            })
            .collect();
        let mut ndc_all_aspects_r: Vec<String> = vec![];
        ndc_all_aspects_r = l.clone().numerologie_aspects.as_slice()
            .iter()
            .filter(|x| {
                x.polarite.as_deref() == Some("-") && x.nom.clone().is_some()
            })
            .map(|x| {
                x.nom.clone().unwrap()
            })
            .collect();
        //---
        let mut traitement_aspects: Vec<NumerologieAspects> = vec![];
        traitement_aspects = res_b.1.as_slice().into_iter().map(|x| {
            let mut find = false;
            for y in bold_aspects.as_slice().iter() {
                if x == y {
                    find = true;
                    break;
                }
            }
            NumerologieAspects {
                aspect: x.to_string(),
                color: ColorEnum::Bleu,
                sw_bold: find,
            }
        }).collect();
        traitement_aspects.extend(
            res_r.1
                .as_slice()
                .into_iter()
                .map(|x| {
                    let sw_bold = bold_aspects.iter().any(|y| x == y);
                    NumerologieAspects {
                        aspect: x.to_string(),
                        color: ColorEnum::Rouge,
                        sw_bold,
                    }
                })
        );
        let mut aspects_cles: Vec<String> = vec![];
        //aspects_cles.extend(res.1.iter().map(|xx|xx.clone()));
        aspects_cles.extend(res_b.1.iter().map(|xx|xx.clone()));
        aspects_cles.extend(res_r.1.iter().map(|xx|xx.clone()));
        let vocabulaire_divers: Vec<SelectionVocabulaireDivers> = lv.iter().map(|x| {
            let nom: String = if x.nom.is_some() {
                x.nom.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l1: String = if x.def_robert_l1.is_some() {
                x.def_robert_l1.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l2: String = if x.def_robert_l2.is_some() {
                x.def_robert_l2.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l3: String = if x.def_robert_l3.is_some() {
                x.def_robert_l3.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l4: String = if x.def_robert_l4.is_some() {
                x.def_robert_l4.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l5: String = if x.def_robert_l5.is_some() {
                x.def_robert_l5.clone().unwrap()
            } else {
                "".to_string()
            };
            SelectionVocabulaireDivers {
                nom: nom,
                def_robert_l1: def_robert_l1,
                def_robert_l2: def_robert_l2,
                def_robert_l3: def_robert_l3,
                def_robert_l4: def_robert_l4,
                def_robert_l5: def_robert_l5,
            }

        }).collect();
        let mut selection: Selection = Selection {
            note_de_cours: vec![],
            aspects: vec![],
            traitement: SelectionTraitment {
                html: res.0,
                html_b: if res_b.0 == "" { None } else { Some(res_b.0) },
                html_r: if res_r.0 == "" { None } else { Some(res_r.0) },
                aspects_cles,
            },
            vocabulaire_divers,
        };
        let mut selection_note_de_cours: Vec<SelectionNoteDeCours> = vec![];
        for (i, x) in l.numerologie_note_de_cours.iter().enumerate() {
            let ndc_res = extract_supers_and_bold_and_italic(x.html_body_one_note_raw.as_str(), true);
            let ndc_res_r = extract_supers_and_bold_and_italic(x.html_body_one_note_raw_r.as_str(), true);
            let ndc_res_r2 = extract_supers_and_bold_and_italic(x.html_body_one_note_raw_r2.as_str(), true);
            let ndc_html_r: Option<String> = if ndc_res_r.0 == "" { None } else { Some(ndc_res_r.0)};
            let ndc_html_r2: Option<String> = if ndc_res_r2.0 == "" { None } else { Some(ndc_res_r2.0)};
            let mut aspects_cles: Vec<String> = vec![];
            aspects_cles.extend(ndc_res.1.iter().map(|xx|xx.clone()));
            aspects_cles.extend(ndc_res_r.1.iter().map(|xx|xx.clone()));
            aspects_cles.extend(ndc_res_r2.1.iter().map(|xx|xx.clone()));
            let mut ndc_mots_cles: Vec<SelectionMotCle> = vec![];
            ndc_mots_cles = x.mots_cles.as_slice().iter().map(|xx| {
                SelectionMotCle {
                    mot_cle: xx.mot_cle.clone(),
                    mot_cle_indice: if xx.mot_cle_indice.clone() == Some("".to_string()) { None } else { xx.mot_cle_indice.clone() },
                    polarite: xx.polarite.clone(),
                }
            }).collect();
            selection_note_de_cours.push(SelectionNoteDeCours {
                mots_cles: ndc_mots_cles,
                aspects_cles,
                html: ndc_res.0,
                html_r: ndc_html_r,
                html_r2: ndc_html_r2,
            });

            let mut ndc_traitement_aspects: Vec<NumerologieAspects> = vec![];
            ndc_traitement_aspects = ndc_res.1.as_slice().into_iter().map(|x| {
                let mut find = false;
                for y in bold_aspects.as_slice().iter() {
                    if x == y {
                        find = true;
                        break;
                    }
                }
                let mut color = ColorEnum::Noir;
                for y in ndc_all_aspects_b.as_slice().iter() {
                    if x == y {
                        color = ColorEnum::Bleu;
                        break;
                    }
                }
                for y in ndc_all_aspects_r.as_slice().iter() {
                    if x == y {
                        color = ColorEnum::Rouge;
                        break;
                    }
                }
                NumerologieAspects {
                    aspect: x.to_string(),
                    color: color,
                    sw_bold: find,
                }
            }).collect();
            ndc_traitement_aspects.extend(
                ndc_res_r.1
                    .as_slice()
                    .into_iter()
                    .map(|x| {
                        let mut find = false;
                        for y in bold_aspects.as_slice().iter() {
                            if x == y {
                                find = true;
                                break;
                            }
                        }
                        let mut color = ColorEnum::Noir;
                        for y in ndc_all_aspects_b.as_slice().iter() {
                            if x == y {
                                color = ColorEnum::Bleu;
                                break;
                            }
                        }
                        for y in ndc_all_aspects_r.as_slice().iter() {
                            if x == y {
                                color = ColorEnum::Rouge;
                                break;
                            }
                        }
                        NumerologieAspects {
                            aspect: x.to_string(),
                            color: color,
                            sw_bold: find,
                        }
                    })
            );
            ndc_traitement_aspects.extend(
                ndc_res_r2.1
                    .as_slice()
                    .into_iter()
                    .map(|x| {
                        let mut find = false;
                        for y in bold_aspects.as_slice().iter() {
                            if x == y {
                                find = true;
                                break;
                            }
                        }
                        let mut color = ColorEnum::Noir;
                        for y in ndc_all_aspects_b.as_slice().iter() {
                            if x == y {
                                color = ColorEnum::Bleu;
                                break;
                            }
                        }
                        for y in ndc_all_aspects_r.as_slice().iter() {
                            if x == y {
                                color = ColorEnum::Rouge;
                                break;
                            }
                        }
                        NumerologieAspects {
                            aspect: x.to_string(),
                            color: color,
                            sw_bold: find,
                        }
                    })
            );
        }
        selection.note_de_cours = selection_note_de_cours;
        selection.aspects = la.iter().map(|x| {
            let nom: String = if x.nom.is_some() {
                x.nom.clone().unwrap()
            } else {
                "".to_string()
            };
            let polarite: String = if x.polarite.is_some() {
                x.polarite.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l1: String = if x.def_robert_l1.is_some() {
                x.def_robert_l1.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l2: String = if x.def_robert_l2.is_some() {
                x.def_robert_l2.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l3: String = if x.def_robert_l3.is_some() {
                x.def_robert_l3.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l4: String = if x.def_robert_l4.is_some() {
                x.def_robert_l4.clone().unwrap()
            } else {
                "".to_string()
            };
            let def_robert_l5: String = if x.def_robert_l5.is_some() {
                x.def_robert_l5.clone().unwrap()
            } else {
                "".to_string()
            };
            SelectionAspect {
                nom: nom,
                sw_bold: x.sw_bold,
                polarite: polarite,
                def_robert_l1: def_robert_l1,
                def_robert_l2: def_robert_l2,
                def_robert_l3: def_robert_l3,
                def_robert_l4: def_robert_l4,
                def_robert_l5: def_robert_l5,
            }
        }).collect();
        let json = serde_json::to_string_pretty(&selection)
            .expect("Erreur de sérialisation Selection");
        Ok(json)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SelectionMotCle {
    pub mot_cle: String,
    pub mot_cle_indice: Option<String>,
    pub polarite: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SelectionNoteDeCours {
    mots_cles: Vec<SelectionMotCle>,
    aspects_cles: Vec<String>,
    html: String,
    html_r: Option<String>,
    html_r2: Option<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct SelectionTraitment {
    html: String,
    html_b: Option<String>,
    html_r: Option<String>,
    aspects_cles: Vec<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct SelectionAspect {
    pub nom: String,
    pub sw_bold: bool,
    pub polarite: String,
    pub def_robert_l1: String,
    pub def_robert_l2: String,
    pub def_robert_l3: String,
    pub def_robert_l4: String,
    pub def_robert_l5: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct SelectionVocabulaireDivers {
    pub nom: String,
    pub def_robert_l1: String,
    pub def_robert_l2: String,
    pub def_robert_l3: String,
    pub def_robert_l4: String,
    pub def_robert_l5: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Selection {
    pub note_de_cours: Vec<SelectionNoteDeCours>,
    pub aspects: Vec<SelectionAspect>,
    pub traitement: SelectionTraitment,
    pub vocabulaire_divers: Vec<SelectionVocabulaireDivers>
}
