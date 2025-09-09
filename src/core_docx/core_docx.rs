use docx_rs::*;
use docx_rs::RunFonts;
use docx_rs::XMLElement::{Indent, Spacing};

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
            if let Some(start) = remaining.find("_BBB") {
                let (before, rest) = remaining.split_at(start);
                if !before.is_empty() {
                    para = para.add_run(Run::new().add_text(before));
                }

                // On coupe l'ouverture "_BBB" (4 caractères)
                let rest = &rest[4..];

                if let Some(end) = rest.find("BBB_") {
                    let bold_text = &rest[..end]; // texte entre _BBB et BBB_
                    para = para.add_run(Run::new().add_text(bold_text).bold());

                    // on avance après la balise fermante "BBB_" (4 caractères)
                    remaining = &rest[end + 4..];
                    continue;
                } else { break; }
            } else if let Some(start) = remaining.find("III_") {
                let (before, rest) = remaining.split_at(start);
                if !before.is_empty() {
                    para = para.add_run(Run::new().add_text(before));
                }
                if let Some(end) = rest[4..].find("III_") {
                    let italic_text = &rest[4..4 + end];
                    para = para.add_run(Run::new().add_text(italic_text).italic());
                    remaining = &rest[5 + end + 4..];
                    continue;
                } else { break; }
            } else if let Some(start) = remaining.find("###") {
                let (before, rest) = remaining.split_at(start);
                if !before.is_empty() {
                    para = para.add_run(Run::new().add_text(before));
                }
                if let Some(end) = rest[3..].find("###") {
                    let sup_text = &rest[3..3 + end];
                    // Texte en exposant avec RunProperties
                    let sup_run = Run::new()
                        .add_text(sup_text);
                       // .property(RunProperties::new().vert_align(VerticalAlignType::Superscript));
                    para = para.add_run(sup_run);
                    remaining = &rest[3 + end + 3..];
                    continue;
                } else { break; }
            } else {
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