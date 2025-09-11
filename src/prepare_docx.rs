use std::io::ErrorKind;
use base64::Engine;
use base64::engine::general_purpose;
use docx_rs::{AbstractNumbering, BreakType, Comment, Docx, Footer, Level, LevelJc, LevelText, NumberFormat, Numbering, PageNum, Paragraph, Pic, Run, SpecialIndentType, Start, XMLDocx};
use crate::api::{TNumerologieClient, ThemeNumerologie};
use crate::core_docx;

pub async fn prepare_docx(token_n: Option<String>, token_t: Option<String>, id: u32, path_cartes: String) -> Result<XMLDocx, Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = Vec::new();
    let mut numerologie: Option<ThemeNumerologie> = None;
    if let Some(t_n) = token_n {
        if let Some(t_t) = token_t {
            let client = TNumerologieClient::new(t_n, t_t, path_cartes);
            match client.get_index(id).await {
                Ok(mut ok) => {
                    match general_purpose::STANDARD.decode(&ok.numerologie.png_simple_b64) {
                        Ok(decoded) => {
                            buf = decoded;
                            if let Some(_) = &ok.get_all().await.ok() {
                                numerologie = Some(ok);
                            } else {
                                return Err(Box::new(std::io::Error::new(
                                    ErrorKind::Other,
                                    "Aucun contenu disponible",
                                )));

                            }
                        },
                        Err(e) => {
                            return Err(Box::new(e));
                        }

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

    let numerologie = numerologie.unwrap(); // TODO later

    let width = ((720 as f64) * 192.0 * 38.7).round() as u32;
    let height = ((397 as f64) * 192.0 * 38.7).round() as u32;
    let pic = Pic::new(&buf.as_slice()).size(width, height);

    let width_carte = ((40 as f64) * 192.0 * 200.0).round() as u32;
    let height_carte = ((75 as f64) * 192.0 * 200.0).round() as u32;
    let ppr_carte: Vec<u8> = numerologie.ppr_carte.as_slice().to_vec();
    let pex_carte: Vec<u8> = numerologie.pex_carte.as_slice().to_vec();
    let cai_carte: Vec<u8> = numerologie.cai_carte.as_slice().to_vec();
    let cae_carte: Vec<u8> = numerologie.cae_carte.as_slice().to_vec();
    let coi_carte: Vec<u8> = numerologie.coi_carte.as_slice().to_vec();
    let coe_carte: Vec<u8> = numerologie.coe_carte.as_slice().to_vec();
    let nem_carte: Vec<u8> = numerologie.nem_carte.as_slice().to_vec();
    let pic_ppr = Pic::new(&ppr_carte.as_slice()).size(width_carte, height_carte);
    let pic_pex = Pic::new(&pex_carte.as_slice()).size(width_carte, height_carte);
    let pic_cai = Pic::new(&cai_carte.as_slice()).size(width_carte, height_carte);
    let pic_cae = Pic::new(&cae_carte.as_slice()).size(width_carte, height_carte);
    let pic_coi = Pic::new(&coi_carte.as_slice()).size(width_carte, height_carte);
    let pic_coe = Pic::new(&coe_carte.as_slice()).size(width_carte, height_carte);
    let pic_nem = Pic::new(&nem_carte.as_slice()).size(width_carte, height_carte);

    let footer =
        Footer::new()
            .add_paragraph(
                Paragraph::new().add_run(Run::new()).add_page_num(PageNum::new()))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Les illustrations des cartes de tarot présentes dans ce document proviennent d’un ensemble restauré et modifié disponible commercialement. Leur utilisation est autorisée uniquement dans un cadre personnel ou commercial sur support physique. Toute redistribution numérique (Word, PDF, images, etc.) est interdite.
")));
    let docx = Docx::new()
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
        .add_table(core_docx::titre_2(format!("Personnalité profonde - {}", numerologie.ppr_lame.unwrap().cartouche_grimaud.unwrap()).as_str())?)
        .add_table(core_docx::content_2_trois_etape(
            pic_ppr,
            numerologie.ppr_mots_cles.as_slice(),
            numerologie.ppr_html.html.as_str(),
            numerologie.ppr_html.html_b.as_str(),
            numerologie.ppr_html.html_r.as_str(),
            numerologie.ppr_aspects.as_slice())?)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
        .add_table(core_docx::titre_2(format!("Caractère extérieur - {}", numerologie.cae_lame.unwrap().cartouche_grimaud.unwrap()).as_str())?)
        .add_table(core_docx::content_2_trois_etape(
            pic_cae,
            numerologie.cae_mots_cles.as_slice(),
            numerologie.cae_html.html.as_str(),
            numerologie.cae_html.html_b.as_str(),
            numerologie.cae_html.html_r.as_str(),
            numerologie.cae_aspects.as_slice())?)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
        .add_table(core_docx::titre_2(format!("Comportement extérieur - {}", numerologie.coe_lame.unwrap().cartouche_grimaud.unwrap()).as_str())?)
        .add_table(core_docx::content_2_trois_etape(
            pic_coe,
            numerologie.coe_mots_cles.as_slice(),
            numerologie.coe_html.html.as_str(),
            numerologie.coe_html.html_b.as_str(),
            numerologie.coe_html.html_r.as_str(),
            numerologie.coe_aspects.as_slice())?)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
        .add_table(core_docx::titre_2(format!("Caractère intérieur - {}", numerologie.cai_lame.unwrap().cartouche_grimaud.unwrap()).as_str())?)
        .add_table(core_docx::content_2_trois_etape(
            pic_cai,
            numerologie.cai_mots_cles.as_slice(),
            numerologie.cai_html.html.as_str(),
            numerologie.cai_html.html_b.as_str(),
            numerologie.cai_html.html_r.as_str(),
            numerologie.cai_aspects.as_slice())?)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
        .add_table(core_docx::titre_2(format!("Comportement intérieur - {}", numerologie.coi_lame.unwrap().cartouche_grimaud.unwrap()).as_str())?)
        .add_table(core_docx::content_2_trois_etape(
            pic_coi,
            numerologie.coi_mots_cles.as_slice(),
            numerologie.coi_html.html.as_str(),
            numerologie.coi_html.html_b.as_str(),
            numerologie.coi_html.html_r.as_str(),
            numerologie.coi_aspects.as_slice())?)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
        .add_table(core_docx::titre_2(format!("Nœud émotionnel - {}", numerologie.nem_lame.unwrap().cartouche_grimaud.unwrap()).as_str())?)
        .add_table(core_docx::content_2_trois_etape(
            pic_nem,
            numerologie.nem_mots_cles.as_slice(),
            numerologie.nem_html.html.as_str(),
            numerologie.nem_html.html_b.as_str(),
            numerologie.nem_html.html_r.as_str(),
            numerologie.nem_aspects.as_slice())?)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
        .add_table(core_docx::titre_2(format!("Personnalité extérieur - {}", numerologie.pex_lame.unwrap().cartouche_grimaud.unwrap()).as_str())?)
        .add_table(core_docx::content_2_trois_etape(
            pic_pex,
            numerologie.pex_mots_cles.as_slice(),
            numerologie.pex_html.html.as_str(),
            numerologie.pex_html.html_b.as_str(),
            numerologie.pex_html.html_r.as_str(),
            numerologie.pex_aspects.as_slice())?)
        .add_numbering(Numbering::new(2, 2))
        .build();
    Ok(docx)
}
