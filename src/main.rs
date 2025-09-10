extern crate core;

pub mod core_docx;
mod api;
mod password;
pub mod html_tools;

use std::ffi::CString;
// mod tools;
use std::fs::File;
use std::io::Read;
use docx_rs::*;
use docx_rs::XMLElement::Num;
use crate::api::{MultiAuth, NumerologieMotCle, TNumerologieClient};
use base64::engine::general_purpose;
use base64::Engine;
use docx_rs::Pic;
use crate::core_docx::{ColorEnum, NumerologieAspects};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = password::load_password("Secrets.yaml");
    let auth = MultiAuth::new(password.unwrap()).await;
    let (token_n, token_t) = auth.get_token();

    println!("Token N: {:?}", token_n);
    println!("Token T: {:?}", token_t);

    let mut buf: Vec<u8> = Vec::new();
    let mut cai_carte: Vec<u8> = Vec::new();
    let mut cai: String = String::new();
    let mut cai_b: String = String::new();
    let mut cai_r: String = String::new();
    let mut cai_cartouche: String = String::new();
    let mut cai_mots_cles: Vec<(ColorEnum, String)> = vec![];
    let mut cai_aspects_b: Vec<String> = vec![];
    let mut cai_aspects_r: Vec<String> = vec![];
    let mut cai_bold_aspects: Vec<String> = vec![];
    let mut cai_aspects: Vec<NumerologieAspects> = vec![];
    if let Some(t_n) = token_n {
        if let Some(t_t) = token_t {
            let client = TNumerologieClient::new(t_n, t_t);
            match client.get_index(43).await {
                Ok(mut ok) => {
                    match general_purpose::STANDARD.decode(&ok.numerologie.png_simple_b64) {
                        Ok(decoded) => {
                            buf = decoded;
                            if let Some((carte, lame_majeur_detail, text)) = ok.get_cai(ok.numerologie.interpretation_cai.clone() as u32).await.ok() {
                                cai_mots_cles = lame_majeur_detail.numerologie_mots_cle.as_slice()
                                    .iter()
                                    .map(|x| {
                                        if (x.polarite == Some("+".to_string())) {
                                            (ColorEnum::Bleu, x.mot_cle.clone())
                                        } else {
                                            (ColorEnum::Rouge, x.mot_cle.clone())
                                        }
                                    })
                                    .collect();
                                if let Some(cartouche) =  lame_majeur_detail.cartouche_grimaud {
                                    cai_cartouche = cartouche;
                                }
                                if let Some(text) = text {
                                    (cai, _) = html_tools::extract_supers_and_bold_and_italic(&text.html_body_one_note_raw.as_str());
                                    (cai_b, _) = html_tools::extract_supers_and_bold_and_italic(&text.html_body_one_note_raw_b.as_str());
                                    (cai_r, _) = html_tools::extract_supers_and_bold_and_italic(&text.html_body_one_note_raw_r.as_str());
                                }
                                match ok.get_carte(carte.clone()).await {
                                    Ok(cai_carte_vu8) => {
                                        cai_carte = cai_carte_vu8;
                                    },
                                    Err(e) => {
                                        eprintln!("Erreur de traitement sur la carte: {}", e);
                                        std::process::exit(1);
                                    },
                                }
                                cai_aspects = ok.cai_aspects.as_slice().to_vec()
                            } else {
                                println!("Aucun contenu disponible");
                            }
                        },
                        Err(_) => {
                            eprintln!("Erreur: base64 invalide pour png_simple_b64");
                            std::process::exit(1);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Erreur de traitement: {}", e);
                    std::process::exit(1);
                },
            }
        } else {
            eprintln!("Erreur: token_n vide");
            std::process::exit(1);
        }
    } else {
        eprintln!("Erreur: token_t vide");
        std::process::exit(1);
    }
    //println!("{:?}", buf);

    let width = ((720 as f64) * 192.0 * 38.7).round() as u32;
    let height = ((397 as f64) * 192.0 * 38.7).round() as u32;
    let pic = Pic::new(&buf.as_slice()).size(width, height);

    let path = std::path::Path::new("./output/examples/image_inline.docx");
    let file = File::create(path).unwrap();

    let width_cai = ((40 as f64) * 192.0 * 200.0).round() as u32;
    let height_cai = ((75 as f64) * 192.0 * 200.0).round() as u32;
    let pic_cai = Pic::new(&cai_carte.as_slice()).size(width_cai, height_cai);

    let footer =
        Footer::new().add_paragraph(Paragraph::new().add_run(Run::new())
            .add_page_num(PageNum::new()));
    Docx::new()
        .footer(footer)
        .add_abstract_numbering(
            AbstractNumbering::new(2).add_level(
                Level::new(
                    0,
                    Start::new(1),
                    NumberFormat::new("bullet"),
                    LevelText::new("•"),
                    LevelJc::new("left"),
                ).spacing(-74)                .indent(
                    Some(300),
                    Some(SpecialIndentType::Hanging(320)),
                    None,
                    None,
                ),
            ),
        )
        .add_numbering(Numbering::new(2, 2))
        .add_table(core_docx::titre_1("Numérologie")?)
        .add_paragraph(Paragraph::new().
            add_run(Run::new()
                .add_text("")))
        .add_table(core_docx::titre_2("Thème")?)
        .add_table(core_docx::theme_2(pic, "Stéphane Bressani", "03.04.1986")?)
        .add_paragraph(Paragraph::new().
            add_run(Run::new()
                .add_text("")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
        .add_table(core_docx::titre_2(format!("Caractère intérieur - {}", cai_cartouche).as_str())?)
        .add_table(core_docx::content_2_trois_etape(pic_cai, cai_mots_cles.as_slice(), cai.as_str(), cai_b.as_str(),cai_r.as_str(), cai_aspects.as_slice())?)
        .add_numbering(Numbering::new(2, 2))
        .build()
        .pack(file)?;
    Ok(())
}