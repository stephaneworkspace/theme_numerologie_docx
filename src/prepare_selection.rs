use std::io::ErrorKind;
use base64::Engine;
use base64::engine::general_purpose;
use docx_rs::{AbstractNumbering, BreakType, Comment, Docx, Footer, Level, LevelJc, LevelText, NumberFormat, Numbering, PageNum, Paragraph, Pic, Run, SpecialIndentType, Start, XMLDocx};
use crate::api::{TNumerologieClient, ThemeNumerologie};
use crate::core_docx;
use crate::api::numerologie::TraitementNumerologie;

pub async fn prepare_selection(token_n: Option<String>, token_t: Option<String>, id: u32, traitement: TraitementNumerologie) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(t_n) = token_n {
        if let Some(t_t) = token_t {
            let client = TNumerologieClient::new_sans_cartes(t_n, t_t);
            match client.get_index_sans_cartes(id).await {
                Ok(mut ok) => {
                    match ok.get_traitement(traitement).await {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(Box::new(e));
                        },
                    }
                },
                Err(e) => {
                    return Err(Box::new(e));
                },
            }
        } else {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Erreur: token_t vide",
            )));
        }
    } else {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::InvalidInput,
            "Erreur: token_n vide",
        )));
    }
    Ok(())
}
