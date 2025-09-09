use docx_rs::{Paragraph, Run, RunFonts};

fn make_run(text: &str) -> Run {
    Run::new()
        .add_text(text)
        .size(crate::core_docx::core_docx::FONT_SIZE_NORMAL * 2)
        .fonts(RunFonts::new()
            .ascii(crate::core_docx::core_docx::FONT)
            .hi_ansi(crate::core_docx::core_docx::FONT)
            .cs(crate::core_docx::core_docx::FONT))
}

pub fn parse_paragraph(text: &str) -> Paragraph {
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
                para = para.add_run(make_run(before));
                remaining = rest;
            }

            if remaining.starts_with("_BBB") {
                let rest = &remaining[4..];
                if let Some(end) = rest.find("BBB_") {
                    let bold_text = &rest[..end];
                    para = para.add_run(make_run(bold_text).bold());

                    // vérifie si le prochain caractère n’est pas un espace ou une virgule
                    if let Some(next_char) = rest[end..].chars().next() {
                        if !next_char.is_whitespace() && next_char != ',' {
                            // ajoute un espace “neutre” pour Word
                            para = para.add_run(make_run("\u{00A0}"));
                        }
                    }

                    remaining = &rest[end + 4..];
                    continue;
                } else {
                    para = para.add_run(make_run(remaining));
                    break;
                }
            }

            if remaining.starts_with("III_") {
                let rest = &remaining[4..];
                if let Some(end) = rest.find("III_") {
                    let italic_text = &rest[..end];
                    para = para.add_run(make_run(italic_text).italic());

                    // vérifie si le prochain caractère n’est pas un espace ou une virgule
                    if let Some(next_char) = rest[end..].chars().next() {
                        if !next_char.is_whitespace() && next_char != ',' {
                            // ajoute un espace “neutre” pour Word
                            para = para.add_run(make_run("\u{00A0}"));
                        }
                    }

                    remaining = &rest[end + 4..];
                    continue;
                } else {
                    para = para.add_run(make_run(remaining));
                    break;
                }
            }

            if remaining.starts_with("###") {
                let rest = &remaining[3..];
                if let Some(end) = rest.find("###") {
                    let underlined_text = &rest[..end];
                    para = para.add_run(make_run(underlined_text).underline("single"));
                    remaining = &rest[end + 3..];
                    continue;
                } else {
                    para = para.add_run(make_run(remaining));
                    break;
                }
            }

            para = para.add_run(make_run(remaining));
            break;
        } else {
            para = para.add_run(make_run(remaining));
            break;
        }
    }

    para
}