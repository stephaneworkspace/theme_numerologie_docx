use std::io::ErrorKind;
use crate::api::{TNumerologieClient, ThemeNumerologie, TraitSelectionNumerologie, TraitSelectionThemeNumerologie};
use crate::api::numerologie::TraitementNumerologie;

pub async fn prepare_selection(token_n: Option<String>, token_t: Option<String>, id: u32, traitement: TraitementNumerologie, carte: Option<u32>) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(t_n) = token_n {
        if let Some(t_t) = token_t {
            let client = TNumerologieClient::new_sans_cartes(t_n, t_t);
            match client.get_index_sans_cartes(id).await {
                Ok(mut ok) => {
                    match ok.selection_traitement_json(traitement, carte).await {
                        Ok(json) => { return Ok(json)},
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
}
