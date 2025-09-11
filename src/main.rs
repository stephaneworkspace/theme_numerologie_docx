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
use crate::api::{MultiAuth, Numerologie, NumerologieMotCle, TNumerologieClient, ThemeNumerologie};
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
    let mut numerologie: ThemeNumerologie = ThemeNumerologie::new(Numerologie {
        id: 0,
        numerologie_type: 0,
        resume_rapide: "".to_string(),
        text: "".to_string(),
        png_b64: "".to_string(),
        png_simple_b64: "".to_string(),
        jour: 0,
        mois: 0,
        annee: 0,
        interpretation_cae: 0,
        interpretation_01_cae_i: "".to_string(),
        interpretation_cai: 0,
        interpretation_01_cai_i: "".to_string(),
        interpretation_coe: 0,
        interpretation_01_coe_i: "".to_string(),
        interpretation_coi: 0,
        interpretation_01_coi_i: "".to_string(),
        interpretation_int: 0,
        interpretation_01_int_i: "".to_string(),
        interpretation_nem: 0,
        interpretation_01_nem_i: "".to_string(),
        interpretation_pex: 0,
        interpretation_01_pex_i: "".to_string(),
        interpretation_ppr: 0,
        interpretation_01_ppr_i: "".to_string(),
        interpretation_rha: 0,
        interpretation_01_rha_i: "".to_string(),
    }, "".to_string());
    if let Some(t_n) = token_n {
        if let Some(t_t) = token_t {
            let client = TNumerologieClient::new(t_n, t_t);
            match client.get_index(43).await {
                Ok(mut ok) => {
                    match general_purpose::STANDARD.decode(&ok.numerologie.png_simple_b64) {
                        Ok(decoded) => {
                            buf = decoded;
                            if let Some(_) = &ok.get_all().await.ok() {
                                numerologie = ok.clone();
                            } else {
                                println!("Aucun contenu disponible");
                            }
                        },
                        Err(_) => {
                            println!("Aucun contenu disponible");
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
    let cai_carte: Vec<u8> = numerologie.cai_carte.as_slice().to_vec();
    let cai: String = numerologie.cai_html.html;
    let cai_b: String = numerologie.cai_html.html_b;
    let cai_r: String = numerologie.cai_html.html_r;
    let cai_cartouche: String = numerologie.cai_lame.clone().unwrap().cartouche_grimaud.unwrap().to_string();
    let cai_mots_cles: Vec<(ColorEnum, String)> = numerologie.cai_mots_cles.as_slice().to_vec();
    let cai_aspects: Vec<NumerologieAspects> = numerologie.cai_aspects.as_slice().to_vec();
    let ppr_carte: Vec<u8> = numerologie.ppr_carte.as_slice().to_vec();
    let ppr: String = numerologie.ppr_html.html;
    let ppr_b: String = numerologie.ppr_html.html_b;
    let ppr_r: String = numerologie.ppr_html.html_r;
    let ppr_cartouche: String = numerologie.ppr_lame.clone().unwrap().cartouche_grimaud.unwrap().to_string();
    let ppr_mots_cles: Vec<(ColorEnum, String)> = numerologie.ppr_mots_cles.as_slice().to_vec();
    let ppr_aspects: Vec<NumerologieAspects> = numerologie.ppr_aspects.as_slice().to_vec();

    let width = ((720 as f64) * 192.0 * 38.7).round() as u32;
    let height = ((397 as f64) * 192.0 * 38.7).round() as u32;
    let pic = Pic::new(&buf.as_slice()).size(width, height);

    let path = std::path::Path::new("./output/examples/image_inline.docx");
    let file = File::create(path).unwrap();

    let width_cai = ((40 as f64) * 192.0 * 200.0).round() as u32;
    let height_cai = ((75 as f64) * 192.0 * 200.0).round() as u32;
    let pic_cai = Pic::new(&cai_carte.as_slice()).size(width_cai, height_cai);
    let pic_ppr = Pic::new(&ppr_carte.as_slice()).size(width_cai, height_cai);

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
        .add_table(core_docx::titre_2(format!("Personalité profonde - {}", ppr_cartouche).as_str())?)
        .add_table(core_docx::content_2_trois_etape(pic_ppr, ppr_mots_cles.as_slice(), ppr.as_str(), ppr_b.as_str(),ppr_r.as_str(), ppr_aspects.as_slice())?)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
        .add_table(core_docx::titre_2(format!("Caractère intérieur - {}", cai_cartouche).as_str())?)
        .add_table(core_docx::content_2_trois_etape(pic_cai, cai_mots_cles.as_slice(), cai.as_str(), cai_b.as_str(),cai_r.as_str(), cai_aspects.as_slice())?)
        .add_numbering(Numbering::new(2, 2))
        .build()
        .pack(file)?;
    Ok(())
}