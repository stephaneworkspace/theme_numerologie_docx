use docx_rs::{Color, Paragraph, Run, RunFonts, RunProperty, VertAlignType};
use regex::bytes::Replacer;
use crate::core_docx::core_docx::ColorEnum;
//use crate::core_docx::RunPropertyExt;
// VertAlignType, XMLElement};
/*
fn run_superscript(text: &str) -> Run {
    Run::new()
        .add_text(text)
        .add_child(
            RawXML::new(r#"<w:rPr><w:vertAlign w:val="superscript"/></w:rPr>"#)
        )
}*/


fn make_run(text: &str, color: &ColorEnum) -> Run {
    Run::new()
        .color(color.hex())
        .add_text(text)
        .size(crate::core_docx::core_docx::FONT_SIZE_NORMAL * 2)
        .fonts(RunFonts::new()
            .ascii(crate::core_docx::core_docx::FONT)
            .hi_ansi(crate::core_docx::core_docx::FONT)
            .cs(crate::core_docx::core_docx::FONT))
}

pub fn parse_paragraph(text: &str, color_enum: ColorEnum) -> Paragraph {
    let mut para = Paragraph::new();

    let mut remaining = text;

    while !remaining.is_empty() {
        let pos_bold = remaining.find("_BBB");
        let pos_italic = remaining.find("III_");
        let pos_underline = remaining.find("###");

        let min_pos = [pos_bold, pos_italic, pos_underline].iter()
            .filter_map(|&p| p)
            .min();

        if let Some(pos) = min_pos {
            if pos > 0 {
                let (before, rest) = remaining.split_at(pos);
                //let before = before.trim_end().replace(" ,", ",");
                let before = before.replace(" ,", ",");
                para = para.add_run(make_run(&before, &color_enum));
                remaining = rest;
            }

            if remaining.starts_with("_BBB") {
                let rest = &remaining[4..];
                if let Some(end) = rest.find("BBB_") {
                    let mut bold_text = rest[..end].to_string();
                    let mut addSpace = false;
                    if rest[end..].starts_with(" ,") {
                        bold_text.push(',');
                        remaining = &rest[end + 2..]; // saute l'espace + virgule
                    } else {
                        if let Some(next_char) = rest[end + 4..].chars().next() {
                            if !next_char.is_whitespace() && next_char != ',' {
                                addSpace = true;
                            }
                        }
                        if addSpace {
                            bold_text.push('\u{00A0}');
                        }
                        remaining = &rest[end + 4..];
                    }
                    para = para.add_run(make_run(&bold_text, &color_enum).bold());
                    continue;
                } else {
                    para = para.add_run(make_run(remaining, &color_enum).bold());
                    break;
                    break;
                }
            }

            if remaining.starts_with("III_") {
                let rest = &remaining[4..];
                if let Some(end) = rest.find("III_") {
                    let italic_text = &rest[..end];
                    para = para.add_run(make_run(italic_text, &color_enum).italic());

                    remaining = &rest[end + 4..];
                    continue;
                } else {
                    para = para.add_run(make_run(remaining, &color_enum).italic());
                    break;
                }
            }
            if remaining.starts_with("###") {
                let rest = &remaining[3..];
                if let Some(end) = rest.find("###") {
                    let underlined_text = &rest[..end];
                    let rp = RunProperty::new()
                        .vert_align(VertAlignType::SuperScript);
                    let mut run = make_run(underlined_text, &color_enum).underline("single");
                    run.run_property = rp;
                    para = para.add_run(run);
                    remaining = &rest[end + 3..];
                    continue;
                } else {
                    para = para.add_run(make_run(remaining, &color_enum));
                    break;
                }
            }

            para = para.add_run(make_run(remaining, &color_enum));
            break;
        } else {
            para = para.add_run(make_run(remaining, &color_enum));
            break;
            break;
        }
    }

    para
}