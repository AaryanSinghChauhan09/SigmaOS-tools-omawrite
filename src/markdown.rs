//! Markdown serialization/parsing for documents.

use crate::document::{Block, BlockKind, Document};

/// Render a document to markdown text.
pub fn render_markdown(doc: &Document) -> String {
    let mut out = String::new();
    let mut in_list = false;
    let mut list_num = 0u32;
    for b in &doc.blocks {
        match b.kind {
            BlockKind::Heading(level) => {
                if in_list { out.push('\n'); in_list = false; }
                out.push_str(&"#".repeat(level as usize));
                out.push(' ');
                out.push_str(&b.text);
                out.push_str("\n\n");
            }
            BlockKind::Paragraph => {
                if in_list { out.push('\n'); in_list = false; }
                out.push_str(&b.text);
                out.push_str("\n\n");
            }
            BlockKind::Quote => {
                if in_list { out.push('\n'); in_list = false; }
                for line in b.text.lines() {
                    out.push_str("> ");
                    out.push_str(line);
                    out.push('\n');
                }
                out.push('\n');
            }
            BlockKind::Code => {
                if in_list { out.push('\n'); in_list = false; }
                out.push_str("```");
                if !b.lang.is_empty() {
                    out.push_str(&b.lang);
                }
                out.push('\n');
                out.push_str(&b.text);
                if !b.text.ends_with('\n') {
                    out.push('\n');
                }
                out.push_str("```\n\n");
            }
            BlockKind::ListItem => {
                if !in_list || list_num != 0 {
                    if in_list { out.push('\n'); }
                    in_list = true;
                    list_num = 0;
                }
                out.push_str("- ");
                out.push_str(&b.text);
                out.push('\n');
            }
            BlockKind::OrderedItem => {
                if !in_list || list_num == 0 {
                    if in_list { out.push('\n'); }
                    in_list = true;
                    list_num = b.number.unwrap_or(1);
                }
                out.push_str(&format!("{}. ", list_num));
                out.push_str(&b.text);
                out.push('\n');
                list_num += 1;
            }
            BlockKind::Rule => {
                if in_list { out.push('\n'); in_list = false; }
                out.push_str("---\n\n");
            }
            BlockKind::Image => {
                if in_list { out.push('\n'); in_list = false; }
                out.push_str(&format!("![{}]\n\n", b.text));
            }
        }
    }
    out.trim_end().to_string() + "\n"
}

/// Parse markdown text into a Document.
pub fn parse_document(text: &str) -> Document {
    let mut doc = Document::default();
    doc.blocks.clear();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;
    let mut para_buf = String::new();
    let mut list_kind: Option<BlockKind> = None;
    let mut list_num = 1u32;

    fn flush_para(doc: &mut Document, buf: &mut String) {
        if !buf.trim().is_empty() {
            doc.blocks.push(Block {
                kind: BlockKind::Paragraph,
                text: buf.trim().to_string(),
                lang: String::new(),
                number: None,
            });
        }
        buf.clear();
    }

    while i < lines.len() {
        let line = lines[i];
        let t = line.trim();
        // Fenced code
        if t.starts_with("```") {
            flush_para(&mut doc, &mut para_buf);
            list_kind = None;
            let lang = t[3..].trim().to_string();
            let mut code = String::new();
            i += 1;
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                code.push_str(lines[i]);
                code.push('\n');
                i += 1;
            }
            i += 1;
            doc.blocks.push(Block {
                kind: BlockKind::Code,
                text: code.trim_end().to_string(),
                lang,
                number: None,
            });
            continue;
        }
        // Heading
        let hashes = t.chars().take_while(|c| *c == '#').count();
        if (1..=6).contains(&hashes) && t[hashes..].starts_with(' ') {
            flush_para(&mut doc, &mut para_buf);
            list_kind = None;
            doc.blocks.push(Block::heading(hashes as u8, t[hashes..].trim()));
            i += 1;
            continue;
        }
        // Horizontal rule
        if t == "---" || t == "***" {
            flush_para(&mut doc, &mut para_buf);
            list_kind = None;
            doc.blocks.push(Block { kind: BlockKind::Rule, text: String::new(), lang: String::new(), number: None });
            i += 1;
            continue;
        }
        // Quote
        if t.starts_with("> ") || t == ">" {
            flush_para(&mut doc, &mut para_buf);
            list_kind = None;
            doc.blocks.push(Block {
                kind: BlockKind::Quote,
                text: t.trim_start_matches('>').trim().to_string(),
                lang: String::new(),
                number: None,
            });
            i += 1;
            continue;
        }
        // List items
        if t.starts_with("- ") || t.starts_with("* ") {
            flush_para(&mut doc, &mut para_buf);
            if list_kind != Some(BlockKind::ListItem) {
                list_kind = Some(BlockKind::ListItem);
            }
            doc.blocks.push(Block {
                kind: BlockKind::ListItem,
                text: t[2..].to_string(),
                lang: String::new(),
                number: None,
            });
            i += 1;
            continue;
        }
        if is_ordered(t) {
            flush_para(&mut doc, &mut para_buf);
            if list_kind != Some(BlockKind::OrderedItem) {
                list_kind = Some(BlockKind::OrderedItem);
                list_num = 1;
            }
            let content = t.split_once('.').map(|(_, c)| c.trim()).unwrap_or(t);
            doc.blocks.push(Block {
                kind: BlockKind::OrderedItem,
                text: content.to_string(),
                lang: String::new(),
                number: Some(list_num),
            });
            list_num += 1;
            i += 1;
            continue;
        }
        // Blank line ends paragraph.
        if t.is_empty() {
            flush_para(&mut doc, &mut para_buf);
            list_kind = None;
            i += 1;
            continue;
        }
        // Paragraph continuation.
        list_kind = None;
        if !para_buf.is_empty() {
            para_buf.push(' ');
        }
        para_buf.push_str(t);
        i += 1;
    }
    flush_para(&mut doc, &mut para_buf);
    if doc.blocks.is_empty() {
        doc.blocks.push(Block::paragraph(""));
    }
    doc.cursor = 0;
    doc
}

fn is_ordered(t: &str) -> bool {
    let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
    !digits.is_empty() && (t[digits.len()..].starts_with(". ") || t[digits.len()..].starts_with(") "))
}
