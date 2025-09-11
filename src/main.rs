extern crate core;

pub mod core_docx;
mod api;
mod password;
pub mod html_tools;
mod prepare_docx;

use std::fs::File;
use std::io::{ErrorKind, Read};
use docx_rs::*;
use crate::api::{MultiAuth, Numerologie, NumerologieMotCle, TNumerologieClient, ThemeNumerologie};
use base64::engine::general_purpose;
use base64::Engine;
use docx_rs::Pic;
use crate::prepare_docx::prepare_docx;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = password::load_password("Secrets.yaml");
    let auth = MultiAuth::new(password.unwrap()).await;
    let (token_n, token_t) = auth.get_token();

    println!("Token N: {:?}", token_n);
    println!("Token T: {:?}", token_t);
    let path = std::path::Path::new("./output/examples/image_inline.docx");
    let file = File::create(path).unwrap();

    prepare_docx(token_n, token_t, 1).await.unwrap().pack(file)?;
    Ok(())
}