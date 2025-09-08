use serde::Deserialize;
use reqwest::{Client, Response};

pub struct Const;

impl Const {
    pub fn api() -> &'static str {
        "https://numerologie.bressani.dev:1122"
    }
    pub fn api_t() -> &'static str {
        "https://t.bressani.dev:1178"
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    token: String,
}

pub struct MultiAuth {
    pub token_n: Option<String>,
    pub token_t: Option<String>,
}

impl MultiAuth {
    pub async fn new(password: String) -> Self {
        let url_n = Const::api().to_string();
        let token_n = Self::fetch_token(&url_n, password.as_str().to_string());
        let url_t = format!("{}/public", Const::api_t());
        let token_t = Self::fetch_token(&url_t, password.as_str().to_string());
        MultiAuth {
            token_n: token_n.await,
            token_t: token_t.await,
        }
    }

    async fn fetch_token(base_url: &String, password: String) -> Option<String> {
        let client = Client::new();
        let url = format!("{}/token_ipad?password={}", base_url, password);

        let resp: Option<Response> = client
            .post(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .ok();

        //println!("-> {:?}", resp); // Le status est ici
        let body = resp?.text().await.ok()?;
        //println!("body: {:?}", body);
        let token_resp: TokenResponse = serde_json::from_str(&body).ok()?;
        //println!("token: {}", token_resp.token);

        Some(token_resp.token)
    }

    pub fn get_token(&self) -> (Option<String>, Option<String>) {
        (self.token_n.as_ref().cloned(), self.token_t.as_ref().cloned())
    }
}