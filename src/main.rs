mod core_docx;
mod api;
mod password;

use std::ffi::CString;
// mod tools;
use std::fs::File;
use std::io::Read;
use docx_rs::*;
use docx_rs::XMLElement::Num;
use crate::api::{MultiAuth, TNumerologieClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = password::load_password("Secrets.yaml");
    let auth = MultiAuth::new(password.unwrap()).await;
    let (token_n, token_t) = auth.get_token();

    println!("Token N: {:?}", token_n);
    println!("Token T: {:?}", token_t);

    let mut png: String = "".to_string();
    if let Some(token) = token_t {
        let client = TNumerologieClient::new(token.to_string());
        match client.get_index(1).await {
            Ok(ok) => {
                png = ok.png_simple_b64;
                println!("{:?}", png);
            },
            Err(e) => {
                eprintln!("Erreur de traitement: {}", e);
                std::process::exit(1);
            },
        }
    } else {
        eprintln!("Erreur: token_t vide");
        std::process::exit(1);
    }
    println!("{:?}", png);

    //tools::run()?;
    let path = std::path::Path::new("./output/examples/image_inline.docx");
    let file = File::create(path).unwrap();
    let mut img = File::open("./images/02.png").unwrap();
    let mut buf = Vec::new();
    let _ = img.read_to_end(&mut buf).unwrap();

    let width = ((720 as f64) * 192.0 * 38.8).round() as u32;
    let height = ((397 as f64) * 192.0 * 38.8).round() as u32;
    let pic = Pic::new(&buf.as_slice()).size(width, height);

    Docx::new()
        .add_table(core_docx::titre_1("Numérologie")?)
        .add_paragraph(Paragraph::new().
            add_run(Run::new()
                .add_text("")))
        .add_table(core_docx::titre_2("Thème")?)
        .add_table(core_docx::theme_2(pic, "Stéphane Bressani", "03.04.1986")?)
        .add_paragraph(Paragraph::new().
            add_run(Run::new()
                .add_text("")))
        .add_table(core_docx::titre_2("Meilleur moyen pour se connecter à son intuition")?)
        .add_table(core_docx::content_2("Le meilleur moyen...")?)
        .build()
        .pack(file)?;
    Ok(())
}