// mod tools;

use std::fs::File;
use std::io::Read;
use docx_rs::*;
use docx_rs::RunFonts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tools::run()?;
    let path = std::path::Path::new("./output/examples/image_inline.docx");
    let file = File::create(path).unwrap();
    let mut img = File::open("./images/02.png").unwrap();
    let mut buf = Vec::new();
    let _ = img.read_to_end(&mut buf).unwrap();

    let pic = Pic::new(&buf.as_slice()).size(320 * 9525, 240 * 9525);

    let table = Table::new(vec![TableRow::new(vec![
        TableCell::new()
            .shading(Shading::new().fill("d1d0d1"))
            .add_paragraph(Paragraph::new().add_run(
                Run::new()
                    .add_text("Numérologie")
                    .size(18 * 2)
                    .fonts(
                        RunFonts::new()
                            .ascii("Calibri")
                            .hi_ansi("Calibri")
                            .cs("Calibri")
                    )
                    .bold()
                ).align(AlignmentType::Left)
            )
            .vertical_align(VAlignType::Center)
            .vertical_merge(VMergeType::Restart),
    ])]).width(5000, WidthType::Pct);

    Docx::new()
        .add_paragraph(Paragraph::new().
            add_run(Run::new()
                .add_text("🐱")
                .add_image(pic)))
        .add_table(table)
        .build()
        .pack(file)?;
    Ok(())
}