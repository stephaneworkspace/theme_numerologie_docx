use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LameMajeuresDivination {
    pub id: u32,
    pub lame_majeure_id: u32,
    pub principe: Option<String>,
    pub principe_positif: Option<String>,
    pub principe_negatif: Option<String>,
    pub mots_cle_positif_l1: Option<String>,
    pub mots_cle_positif_l2: Option<String>,
    pub mots_cle_positif_l3: Option<String>,
    pub mots_cle_positif_l4: Option<String>,
    pub mots_cle_positif_l5: Option<String>,
    pub mots_cle_positif_l6: Option<String>,
    pub mots_cle_positif_l7: Option<String>,
    pub mots_cle_positif_l8: Option<String>,
    pub mots_cle_negatif_l1: Option<String>,
    pub mots_cle_negatif_l2: Option<String>,
    pub mots_cle_negatif_l3: Option<String>,
    pub mots_cle_negatif_l4: Option<String>,
    pub mots_cle_negatif_l5: Option<String>,
    pub mots_cle_negatif_l6: Option<String>,
    pub mots_cle_negatif_l7: Option<String>,
    pub mots_cle_negatif_l8: Option<String>,
    pub psychologie: Option<String>,
    pub action: Option<String>,
    pub situation: Option<String>,
    pub evenement: Option<String>,
    pub point_particulier: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LameMajeuresDivinationDetail {
    pub id: u32,
    pub lame_majeure_id: u32,
    pub principe: Option<String>,
    pub psychologie: Option<String>,
    pub psychologie_positif_l1: Option<String>,
    pub psychologie_positif_l2: Option<String>,
    pub psychologie_positif_l3: Option<String>,
    pub psychologie_positif_l4: Option<String>,
    pub psychologie_positif_l5: Option<String>,
    pub psychologie_positif_l6: Option<String>,
    pub psychologie_negatif_l1: Option<String>,
    pub psychologie_negatif_l2: Option<String>,
    pub psychologie_negatif_l3: Option<String>,
    pub psychologie_negatif_l4: Option<String>,
    pub psychologie_negatif_l5: Option<String>,
    pub psychologie_negatif_l6: Option<String>,
    pub action: Option<String>,
    pub action_positif_l1: Option<String>,
    pub action_positif_l2: Option<String>,
    pub action_positif_l3: Option<String>,
    pub action_positif_l4: Option<String>,
    pub action_positif_l5: Option<String>,
    pub action_negatif_l1: Option<String>,
    pub action_negatif_l2: Option<String>,
    pub action_negatif_l3: Option<String>,
    pub action_negatif_l4: Option<String>,
    pub action_negatif_l5: Option<String>,
    pub situation: Option<String>,
    pub situation_positif_l1: Option<String>,
    pub situation_positif_l2: Option<String>,
    pub situation_positif_l3: Option<String>,
    pub situation_positif_l4: Option<String>,
    pub situation_positif_l5: Option<String>,
    pub situation_positif_l6: Option<String>,
    pub situation_negatif_l1: Option<String>,
    pub situation_negatif_l2: Option<String>,
    pub situation_negatif_l3: Option<String>,
    pub situation_negatif_l4: Option<String>,
    pub situation_negatif_l5: Option<String>,
    pub situation_negatif_l6: Option<String>,
    pub evenement: Option<String>,
    pub evenement_positif_l1: Option<String>,
    pub evenement_positif_l2: Option<String>,
    pub evenement_positif_l3: Option<String>,
    pub evenement_positif_l4: Option<String>,
    pub evenement_positif_l5: Option<String>,
    pub evenement_positif_l6: Option<String>,
    pub evenement_negatif_l1: Option<String>,
    pub evenement_negatif_l2: Option<String>,
    pub evenement_negatif_l3: Option<String>,
    pub evenement_negatif_l4: Option<String>,
    pub evenement_negatif_l5: Option<String>,
    pub evenement_negatif_l6: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LameMajeuresDivinationPrincipes {
    pub id: u32,
    pub lame_majeure_id: u32,
    pub les_principes: Option<String>,
    pub les_domaines: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DivinationAssociationsClassiques {
    pub id: String,
    pub lame_majeure_id: u32,
    pub nom: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DivinationPersonnages {
    pub id: String,
    pub lame_majeure_id: u32,
    pub nom: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LameMajeuresNumerologie {
    pub id: u32,
    pub lame_majeure_id: u32,
    pub archetype_l1: Option<String>,
    pub archetype_l2: Option<String>,
    pub archetype_l3: Option<String>,
    pub principes_l1: Option<String>,
    pub principes_l2: Option<String>,
    pub principes_l3: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieAspect {
    pub id: String,
    pub lame_majeure_id: u32,
    pub nom: Option<String>,
    pub polarite: Option<String>,
    #[serde(rename = "swBold", default)]
    pub sw_bold: bool,
    #[serde(rename = "swCaractereIntime", default)]
    pub sw_caractere_intime: bool,
    #[serde(rename = "swCaractereSocial", default)]
    pub sw_caractere_social: bool,
    #[serde(rename = "swComportementIntime", default)]
    pub sw_comportement_intime: bool,
    #[serde(rename = "swComportementSocial", default)]
    pub sw_comportement_social: bool,
    pub explication: Option<String>,
    pub def_robert_l1: Option<String>,
    pub def_robert_l2: Option<String>,
    pub def_robert_l3: Option<String>,
    pub def_robert_l4: Option<String>,
    pub def_robert_l5: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieMotCle {
    pub id: String,
    pub lame_majeure_id: u32,
    pub mot_cle: String,
    pub mot_cle_indice: Option<String>,
    pub polarite: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieNoteDeCours {
    pub id: String,
    pub lame_majeure_id: u32,
    pub html_body_one_note_raw: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub mots_cles: Vec<NumerologieMotCle>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieMessageKarmique {
    pub id: String,
    pub lame_majeure_id: u32,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologiePersonaliteProfonde {
    pub id: String,
    pub lame_majeure_id: u32,
    pub html_body_one_note_raw: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieCaractereIntime {
    pub id: String,
    pub lame_majeure_id: u32,
    pub html_body_one_note_raw: String,
    pub html_body_one_note_raw_intuition: String,
    pub html_body_one_note_raw_intuition_metaphore: String,
    pub nombre_ami: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieIntellect {
    pub id: String,
    pub lame_majeure_id: u32,
    pub html_body_one_note_raw: String,
    pub html_body_one_note_raw_metaphore: String,
    pub mois_similaire: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieCaractereSocial {
    pub id: String,
    pub lame_majeure_id: u32,
    pub html_body_one_note_raw: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieNoeudEmotionnel {
    pub id: String,
    pub lame_majeure_id: u32,
    pub html_body_one_note_raw: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieComportementIntime {
    pub id: String,
    pub lame_majeure_id: u32,
    pub html_body_one_note_raw: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieComportementSocial {
    pub id: String,
    pub lame_majeure_id: u32,
    pub html_body_one_note_raw: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologiePersonaliteExterieure {
    pub id: String,
    pub lame_majeure_id: u32,
    pub html_body_one_note_raw: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NumerologieVocabulaireDivers {
    pub id: String,
    pub lame_majeure_id: Option<u32>,
    pub nom: Option<String>,
    pub def_robert_l1: Option<String>,
    pub def_robert_l2: Option<String>,
    pub def_robert_l3: Option<String>,
    pub def_robert_l4: Option<String>,
    pub def_robert_l5: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct LameMajeureSummary {
    pub id: u32,
    pub nombre: u32,
    pub nombre_romain: String,
    pub nom: Option<String>,
    pub cartouche_grimaud: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LameMajeureDetail {
    pub id: u32,
    pub nombre: u32,
    pub nombre_romain: String,
    pub nom: Option<String>,
    pub cartouche_grimaud: Option<String>,
    pub lame_majeures_divination: Option<LameMajeuresDivination>,
    pub lame_majeures_divination_detail: Option<LameMajeuresDivinationDetail>,
    pub lame_majeures_divination_principes: Option<LameMajeuresDivinationPrincipes>,
    pub divination_associations_classiques: Vec<DivinationAssociationsClassiques>,
    pub divination_personnages: Vec<DivinationPersonnages>,
    pub lame_majeures_numerologie: Option<LameMajeuresNumerologie>,
    pub numerologie_aspects: Vec<NumerologieAspect>,
    pub numerologie_mots_cle: Vec<NumerologieMotCle>,
    pub numerologie_note_de_cours: Vec<NumerologieNoteDeCours>,
    pub numerologie_message_karmique: Option<NumerologieMessageKarmique>,
    pub numerologie_personalite_profonde: Option<NumerologiePersonaliteProfonde>,
    pub numerologie_caractere_intime: Option<NumerologieCaractereIntime>,
    pub numerologie_intellect: Option<NumerologieIntellect>,
    pub numerologie_caractere_social: Option<NumerologieCaractereSocial>,
    pub numerologie_noeud_emotionnel: Option<NumerologieNoeudEmotionnel>,
    pub numerologie_comportement_intime: Option<NumerologieComportementIntime>,
    pub numerologie_comportement_social: Option<NumerologieComportementSocial>,
    pub numerologie_personalite_exterieure: Option<NumerologiePersonaliteExterieure>,
    pub numerologie_vocabulaire_divers: Vec<NumerologieVocabulaireDivers>,
}