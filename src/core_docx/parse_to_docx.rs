use docx_rs::{Paragraph, Run};

pub fn parse_paragraph(text: &str) -> Paragraph {
    let mut para = Paragraph::new();
    let mut remaining = text;

    while !remaining.is_empty() {
        let pos_bold = remaining.find("_BBB");
        let pos_italic = remaining.find("III_");
        let pos_underline = remaining.find("###");

        // Find the earliest marker position
        let min_pos = [pos_bold, pos_italic, pos_underline].iter()
            .filter_map(|&p| p)
            .min();

        if let Some(pos) = min_pos {
            // Add text before the marker as normal run if any
            if pos > 0 {
                let (before, rest) = remaining.split_at(pos);
                para = para.add_run(Run::new().add_text(before));
                remaining = rest;
            }

            if let Some(start_pos) = remaining.find("_BBB") {
                if start_pos == 0 {
                    let rest = &remaining[4..]; // remove "_BBB"
                    if let Some(end) = rest.find("BBB_") {
                        let bold_text = &rest[..end];

                        // Add space if next char after closing marker is not whitespace or ','
                        let bold_run_text = if let Some(next_char) = rest[end + 4..].chars().next() {
                            if !next_char.is_whitespace() && next_char != ',' {
                                format!("{} ", bold_text)
                            } else {
                                bold_text.to_string()
                            }
                        } else {
                            bold_text.to_string()
                        };

                        para = para.add_run(Run::new().add_text(bold_run_text).bold());

                        remaining = &rest[end + 4..];
                        continue;
                    } else {
                        // No closing marker, treat as literal
                        para = para.add_run(Run::new().add_text(remaining));
                        break;
                    }
                }
            }

            if let Some(start_pos) = remaining.find("III_") {
                if start_pos == 0 {
                    let rest = &remaining[4..]; // remove "III_"
                    if let Some(end) = rest.find("III_") {
                        let italic_text = &rest[..end];

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

                        remaining = &rest[end + 4..];
                        continue;
                    } else {
                        // No closing marker, treat as literal
                        para = para.add_run(Run::new().add_text(remaining));
                        break;
                    }
                }
            }

            if let Some(start_pos) = remaining.find("###") {
                if start_pos == 0 {
                    let rest = &remaining[3..]; // remove opening "###"
                    if let Some(end) = rest.find("###") {
                        let underlined_text = &rest[..end];
                        para = para.add_run(Run::new()
                            .add_text(underlined_text)
                            .underline("single"));
                        remaining = &rest[end + 3..];
                        continue;
                    } else {
                        // No closing marker, treat as literal
                        para = para.add_run(Run::new().add_text(remaining));
                        break;
                    }
                }
            }

            // If marker not at start (should not happen due to above split), treat as normal text
            para = para.add_run(Run::new().add_text(remaining));
            break;
        } else {
            // No markers found, add remaining as normal text and break
            para = para.add_run(Run::new().add_text(remaining));
            break;
        }
    }

    para
}
