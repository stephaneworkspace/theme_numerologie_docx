use std::fs::File;
use std::io::Read;
use zip::read::ZipArchive;
use docx_rs::*;
use roxmltree::Document;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 1️⃣ Ouvrir le DOCX existant
    let file = File::open("./input/numerologie.docx")?;
    let mut archive = ZipArchive::new(file)?;

    // 2️⃣ Lire document.xml
    let mut doc_xml = String::new();
    archive.by_name("word/document.xml")?.read_to_string(&mut doc_xml)?;

    // 3️⃣ Parser le XML
    let xml_doc = Document::parse(&doc_xml)?;

    // 4️⃣ Créer le nouveau DOCX
    let mut new_doc = Docx::new();

    // 5️⃣ Parcourir les paragraphes et les runs
    for node in xml_doc.descendants() {
        if node.tag_name().name() == "p" {
            let mut paragraph = Paragraph::new();
            for run in node.descendants().filter(|n| n.tag_name().name() == "t") {
                if let Some(text) = run.text() {
                    paragraph = paragraph.add_run(Run::new().add_text(text));
                }
            }
            new_doc = new_doc.add_paragraph(paragraph);
        }
    }

    // 6️⃣ Écrire le nouveau DOCX
    let out_file = File::create("nouveau.docx")?;
    new_doc.build().pack(out_file)?;

    println!("Nouveau DOCX créé avec succès !");
    Ok(())
}