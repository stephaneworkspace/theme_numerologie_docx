use serde::Deserialize;
use reqwest::{Client, Response};
use base64::engine::general_purpose;
use base64::Engine;

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

pub struct TNumerologieClient {
    base_url: String,
    token: String,
}

impl TNumerologieClient {
    pub fn new(token: String) -> Self {
        Self {
            base_url: "https://t.bressani.dev:1178".to_string(),
            token,
        }
    }

    pub async fn get_index(&self, id: i32) -> Result<Numerologie, reqwest::Error> {
        let url = format!("{}/api/numerologie/{}", self.base_url, id);
        let client = Client::new();
        let resp: Option<Response> = client
            .get(&url)
            //.header("Accept", "application/json")
            .bearer_auth(&self.token)
            .send()
            .await
            .ok(); // TODO catch l'erreur 200 error_for_status()?
        //let body = resp.unwrap().text().await?;
        //println!("{}", body);
        let numerologie: Numerologie = resp.unwrap().json().await?;
        Ok(numerologie)
    }
}

// Example function showing how to decode png_simple_b64 and use it
// This is not part of the original file, but the instructions imply modifying main.rs accordingly.
// The following code snippet should be integrated in main.rs or wherever appropriate:

/*
use some_image_display_crate::Pic; // Hypothetical import

fn process_numerologie_image(numerologie: &Numerologie, width: u32, height: u32) -> Result<Pic, String> {
    let buf = general_purpose::STANDARD.decode(&numerologie.png_simple_b64)
        .map_err(|e| format!("Erreur lors du décodage Base64 de png_simple_b64: {}", e))?;
    // let pic = Pic::new(&buf.as_slice()).size(width, height);
    // return Ok(pic);
}
*/

// The commented out code for reading image from disk should be removed from main.rs as per instructions.