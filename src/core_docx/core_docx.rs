use docx_rs::*;
use docx_rs::RunFonts;

pub const FONT_SIZE_TITRE_1: usize = 18;
pub const FONT_SIZE_TITRE_2: usize = 11;
pub const FONT_SIZE_NORMAL: usize = 11;
pub const SHADE_TITRE_1: &str = "d1d0d1";
pub const SHADE_TITRE_2: &str = "e7e7e7";
pub const FONT: &str = "Calibri";

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
use crate::core_docx::parse_to_docx::parse_paragraph;

pub fn content_2(content: &str) -> Result<Table, Box<dyn std::error::Error>> {

    let table = Table::new(vec![TableRow::new(vec![TableCell::new()
                                                       .add_paragraph(
                                                           parse_paragraph(content)
                                                               .size(FONT_SIZE_NORMAL * 2)
                                                               .fonts(RunFonts::new()
                                                                   .ascii(FONT)
                                                                   .hi_ansi(FONT)
                                                                   .cs(FONT))
                                                               .align(AlignmentType::Left),
                                                       ),
    ])])
        .width(5000, WidthType::Pct)
        .margins(TableCellMargins::new().margin_top(100, WidthType::Dxa));

    Ok(table)
}