mod core_docx;

// mod tools;
use std::fs::File;
use std::io::Read;
use docx_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tools::run()?;
    let path = std::path::Path::new("./output/examples/image_inline.docx");
    let file = File::create(path).unwrap();
    let mut img = File::open("./images/02.png").unwrap();
    let mut buf = Vec::new();
    let _ = img.read_to_end(&mut buf).unwrap();

    let pic = Pic::new(&buf.as_slice()).size(320 * 9525, 240 * 9525);

    Docx::new()
        .add_paragraph(Paragraph::new().
            add_run(Run::new()
                .add_text("🐱")
                .add_image(pic)))
        .add_table(core_docx::titre_1("Numérologie")?)
        .add_paragraph(Paragraph::new().
            add_run(Run::new()
                .add_text("")))
        .add_table(core_docx::titre_2("Thème")?)
        .build()
        .pack(file)?;
    Ok(())
}