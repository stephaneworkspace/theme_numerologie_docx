use docx_rs::*;
use docx_rs::RunFonts;

const FONT_SIZE_TITRE_1: usize = 18;
const FONT_SIZE_TITRE_2: usize = 11;
const FONT_SIZE_NORMAL: usize = 11;
const SHADE_TITRE_1: &str = "d1d0d1";
const SHADE_TITRE_2: &str = "e7e7e7";
const FONT: &str = "Calibri";

pub fn titre_1(titre: &str) -> Result<Table, Box<dyn std::error::Error>> {
    let table = Table::new(vec![TableRow::new(vec![
        TableCell::new()
            .shading(Shading::new().fill(SHADE_TITRE_1))
            .add_paragraph(Paragraph::new().add_run(
                Run::new()
                    .add_text(titre)
                    .size(FONT_SIZE_TITRE_1 * 2)
                    .fonts(
                        RunFonts::new()
                            .ascii(FONT)
                            .hi_ansi(FONT)
                            .cs(FONT)
                    )
                    .bold()
            ).align(AlignmentType::Left)
                .line_spacing(LineSpacing::new().after(100))
            )
            .vertical_align(VAlignType::Center)
            .vertical_merge(VMergeType::Restart),
    ])])
        .width(5000, WidthType::Pct)
        .margins(
            TableCellMargins::new()
                .margin_top(80, WidthType::Dxa));  // Header only
    Ok(table)
}

pub fn titre_2(titre: &str) -> Result<Table, Box<dyn std::error::Error>> {
    let table = Table::new(vec![TableRow::new(vec![
        TableCell::new()
            .shading(Shading::new().fill(SHADE_TITRE_2))
            .add_paragraph(Paragraph::new().add_run(
                Run::new()
                    .add_text(titre)
                    .size(FONT_SIZE_TITRE_2 * 2)
                    .fonts(
                        RunFonts::new()
                            .ascii(FONT)
                            .hi_ansi(FONT)
                            .cs(FONT)
                    )
                    .bold()
            ).align(AlignmentType::Left)
                .indent(Some(50), Some(SpecialIndentType::FirstLine(0)), Some(0), Some(0))
            )
            .vertical_align(VAlignType::Center)
            .vertical_merge(VMergeType::Restart),
    ])]).width(5000, WidthType::Pct);
    Ok(table)
}
pub fn theme_2(pic: Pic, name: &str, date: &str) -> Result<Table, Box<dyn std::error::Error>> {
    let table = Table::new(vec![TableRow::new(vec![
        TableCell::new()
            //.shading(Shading::new().fill(WHITE_2))
            .add_paragraph(Paragraph::new().add_run(
                Run::new()
                    .add_image(pic)
            ).align(AlignmentType::Left))
            .add_paragraph(Paragraph::new().add_run(
                Run::new()
                    .add_text(format!("Nom : {}", name))
                    .size(FONT_SIZE_NORMAL * 2)
                    .fonts(
                        RunFonts::new()
                            .ascii(FONT)
                            .hi_ansi(FONT)
                            .cs(FONT)
                    )
            ).align(AlignmentType::Left))
            .add_paragraph(Paragraph::new().add_run(
                Run::new()
                    .add_text(format!("Date : {}", date))
                    .size(FONT_SIZE_NORMAL * 2)
                    .fonts(
                        RunFonts::new()
                            .ascii(FONT)
                            .hi_ansi(FONT)
                            .cs(FONT)
                    )
            ).align(AlignmentType::Left))
    ])])
        .width(5000, WidthType::Pct)
        .margins(
            TableCellMargins::new()
                .margin_top(100, WidthType::Dxa)); // A ajouter à chaque content
    Ok(table)
}
use docx_rs::*;

pub fn content_2(content: &str) -> Result<Table, Box<dyn std::error::Error>> {
    fn parse_paragraph(text: &str) -> Paragraph {
        let mut para = Paragraph::new();
        let mut remaining = text;

        while !remaining.is_empty() {
            // ===== Bold =====
            if let Some(start) = remaining.find("_BBB") {
                let (before, rest) = remaining.split_at(start);
                if !before.is_empty() {
                    para = para.add_run(Run::new().add_text(before));
                }

                let rest = &rest[4..]; // on enlève "_BBB"

                if let Some(end) = rest.find("BBB_") {
                    let mut bold_text = &rest[..end]; // texte entre _BBB et BBB_

                    // Ajouter un espace si le caractère suivant n'est pas déjà un espace
                    let bold_run_text = if let Some(next_char) = rest[end + 4..].chars().next() {
                        if !next_char.is_whitespace() && next_char != ',' {
                            format!("{} ", bold_text) // nouvelle String
                        } else {
                            bold_text.to_string() // convertir &str en String
                        }
                    } else {
                        bold_text.to_string()
                    };

                    para = para.add_run(Run::new().add_text(bold_run_text).bold());

                    remaining = &rest[end + 4..]; // après la fermeture
                    continue;
                } else { break; }
            }
            // ===== Italic =====
            else if let Some(start) = remaining.find("III_") {
                let (before, rest) = remaining.split_at(start);
                if !before.is_empty() {
                    para = para.add_run(Run::new().add_text(before));
                }

                let rest = &rest[4..]; // on enlève "III_"

                if let Some(end) = rest.find("III_") {
                    let mut italic_text = &rest[..end];

                    let italic_run_text = if let Some(next_char) = rest[end + 4..].chars().next() {
                        if !next_char.is_whitespace() && next_char != ',' {
                            format!("{} ", italic_text)
                        } else {
                            italic_text.to_string()
                        }
                    } else {
                        italic_text.to_string()
                    };

                    para = para.add_run(Run::new().add_text(italic_run_text).italic());

                    remaining = &rest[end + 4..]; // après la fermeture
                    continue;
                } else { break; }
            }
            // ===== Underline =====
            else if let Some(start) = remaining.find("###") {
                let (before, rest) = remaining.split_at(start);
                if !before.is_empty() {
                    para = para.add_run(Run::new().add_text(before));
                }

                let rest = &rest[3..]; // on enlève la première balise "###"

                if let Some(end) = rest.find("###") {
                    let underlined_text = &rest[..end];

                    para = para.add_run(Run::new()
                        .add_text(underlined_text)
                        .underline("single"));

                    remaining = &rest[end + 3..]; // avancer après la fermeture
                    continue;
                } else {
                    // si balise fermante manquante, on ajoute le reste tel quel
                    para = para.add_run(Run::new().add_text("###").add_text(rest));
                    break;
                }
            }
                /*
            // ===== Superscript (désactivé) =====
            else if let Some(start) = remaining.find("###") {
                let (before, rest) = remaining.split_at(start);
                if !before.is_empty() {
                    para = para.add_run(Run::new().add_text(before));
                }
                /*
                                if let Some(end) = rest[3..].find("###") {
                                    let sup_text = &rest[3..3 + end];
                                    // Superscript désactivé pour l'instant
                                    para = para.add_run(Run::new().add_text(sup_text).property(RunProperties::new().vert_align(VerticalAlignType::Superscript)));

                                    remaining = &rest[3 + end + 3..];
                                    continue;
                                } else { break; }
                            }
                            */
                if let Some(start) = remaining.find("###") {
                    let (before, rest) = remaining.split_at(start);
                    if !before.is_empty() {
                        para = para.add_run(Run::new().add_text(before));
                    }

                    if let Some(end) = rest[3..].find("###") {
                        let underlined_text = &rest[3..3 + end];

                        // Crée un Run souligné
                        para = para.add_run(Run::new()
                            .add_text(underlined_text)
                            .underline("single"));

                        remaining = &rest[3 + end + 3..];
                        continue;
                    } else { break; }
                }
            }*/
            // ===== Texte normal =====
            else {
                para = para.add_run(Run::new().add_text(remaining));
                break;
            }
        }

        para
    }

    let table = Table::new(vec![TableRow::new(vec![TableCell::new()
                                                       .add_paragraph(
                                                           parse_paragraph(content)
                                                               .size(FONT_SIZE_NORMAL * 2)
                                                               .fonts(RunFonts::new().ascii(FONT).hi_ansi(FONT).cs(FONT))
                                                               .align(AlignmentType::Left),
                                                       ),
    ])])
        .width(5000, WidthType::Pct)
        .margins(TableCellMargins::new().margin_top(100, WidthType::Dxa));

    Ok(table)
}