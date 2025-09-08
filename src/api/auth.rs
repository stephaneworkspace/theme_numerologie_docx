use serde::Deserialize;
use std::fs;
use reqwest::Client;

pub struct Const;

impl Const {
    pub fn api() -> &'static str {
        "https://numerologie.bressani.dev:1122"
    }
    pub fn api_t() -> &'static str {
        "https://t.bressani.dev:1178"
    }
    pub fn token(optional_password: Option<String>) -> String {
        if let Some(pass) = optional_password {
            return pass;
        }
        // lecture depuis Secrets.plist ou .env
        let secrets_path = "Secrets.plist";
        let content = fs::read_to_string(secrets_path).unwrap_or_default();
        // ⚠ Ici il faudrait parser le plist (ou autre format)
        // Pour le moment on renvoie brut
        content.trim().to_string()
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    token: String,
}

pub struct MultiAuth {
    pub password: Option<String>,
    pub token_n: Option<String>,
    pub token_t: Option<String>,
}

impl MultiAuth {
    pub async fn new(password: Option<String>) -> Self {
        let pass = Some(Const::token(password.as_ref().cloned()));
        let token_n = Self::fetch_token(Const::api(), pass.as_ref().cloned());
        //let token_t = Self::fetch_token(&format!("{}/public", Const::api_t()), pass.clone());
        /*if let Some(ref t) = token_n {
            println!("✅ Token ok pour: {} {}", Const::api(), t);
        }
        if let Some(ref t) = token_t {
            println!("✅ Token ok pour: {} {}", Const::api_t(), t);
        }
*/
        MultiAuth {
            password: pass,
            token_n: token_n.await,
            token_t: None,
        }
    }

    async fn fetch_token(base_url: &str, password: Option<String>) -> Option<String> {
        let client = Client::new();
        let url = format!("{}/token_ipad?password={}", base_url, Const::token(password));

        let resp = client
            .post(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .ok()?;

        // let token_resp: TokenResponse = resp.json().ok()?;
        println!("-> {:?}", resp);
        Some("token_resp.token".to_string())
    }

    pub fn get_token(&self) -> (Option<String>, Option<String>) {
        (self.token_n.as_ref().cloned(), self.token_t.as_ref().cloned())
    }
}