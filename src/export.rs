//! HTML export — render the document model to standalone HTML.
//!
//! Upstream omawrite is a markdown editor; this port renders the same
//! `Document`/`Block` model to clean HTML with inline markdown
//! formatting (bold/italic/code/links) resolved to tags.

use crate::document::{Block, BlockKind, Document};

/// Render the document to a standalone HTML page.
pub fn render_html(doc: &Document, title: &str) -> String {
    let mut s = String::new();
    s.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n");
    s.push_str(&format!("<title>{}</title>\n", esc(title)));
    s.push_str("<style>body{max-width:70ch;margin:2em auto;font-family:serif;line-height:1.6}code{background:#eee;padding:.1em .3em}pre{background:#f5f5f5;padding:1em;overflow:auto}blockquote{border-left:3px solid #ccc;margin-left:0;padding-left:1em;color:#666}</style>\n");
    s.push_str("</head>\n<body>\n");
    for block in &doc.blocks {
        s.push_str(&render_block(block));
        s.push('\n');
    }
    s.push_str("</body>\n</html>\n");
    s
}

/// Render to HTML fragment (no html/head/body).
pub fn render_fragment(doc: &Document) -> String {
    let mut s = String::new();
    for block in &doc.blocks {
        s.push_str(&render_block(block));
        s.push('\n');
    }
    s
}

fn render_block(b: &Block) -> String {
    match &b.kind {
        BlockKind::Paragraph => format!("<p>{}</p>", inline_html(&b.text)),
        BlockKind::Heading(level) => {
            let l = (*level).clamp(1, 6);
            format!("<h{}>{}</h{}>", l, inline_html(&b.text), l)
        }
        BlockKind::Quote => format!("<blockquote><p>{}</p></blockquote>", inline_html(&b.text)),
        BlockKind::Code => {
            let cls = if b.lang.is_empty() { String::new() } else { format!(" class=\"language-{}\"", esc(&b.lang)) };
            format!("<pre><code{}>{}</code></pre>", cls, esc(&b.text))
        }
        BlockKind::ListItem => format!("<ul><li>{}</li></ul>", inline_html(&b.text)),
        BlockKind::OrderedItem => {
            let n = b.number.unwrap_or(1);
            format!("<ol start=\"{}\"><li>{}</li></ol>", n, inline_html(&b.text))
        }
        BlockKind::Rule => "<hr>".to_string(),
        BlockKind::Image => format!("<img src=\"{}\" alt=\"{}\">", esc(&b.text), esc(&b.text)),
    }
}

/// Inline markdown → HTML: `code`, **bold**, *italic*, [text](url), ![alt](src).
fn inline_html(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while !rest.is_empty() {
        // Find earliest delimiter among ` ** * [ ![
        let candidates = [
            ('`', rest.find('`')),
            ('*', rest.find("**").map(|i| (i, true)).or_else(|| rest.find('*').map(|i| (i, false))).map(|(i,_)| i)),
            ('[', rest.find('[')),
        ];
        let mut earliest: Option<(usize, char)> = None;
        for (c, pos) in candidates {
            if let Some(p) = pos {
                if earliest.map(|(e, _)| p < e).unwrap_or(true) {
                    earliest = Some((p, c));
                }
            }
        }
        let Some((pos, delim)) = earliest else {
            out.push_str(&esc(rest));
            break;
        };
        out.push_str(&esc(&rest[..pos]));
        rest = &rest[pos..];
        match delim {
            '`' => {
                if let Some(end) = rest[1..].find('`') {
                    out.push_str(&format!("<code>{}</code>", esc(&rest[1..1 + end])));
                    rest = &rest[1 + end + 1..];
                } else {
                    out.push('`');
                    rest = &rest[1..];
                }
            }
            '*' => {
                if rest.starts_with("**") {
                    if let Some(end) = rest[2..].find("**") {
                        out.push_str(&format!("<strong>{}</strong>", inline_html(&rest[2..2 + end])));
                        rest = &rest[2 + end + 2..];
                    } else {
                        out.push_str("**");
                        rest = &rest[2..];
                    }
                } else if let Some(end) = rest[1..].find('*') {
                    out.push_str(&format!("<em>{}</em>", inline_html(&rest[1..1 + end])));
                    rest = &rest[1 + end + 1..];
                } else {
                    out.push('*');
                    rest = &rest[1..];
                }
            }
            '[' => {
                // Link [text](url)
                if let Some(close) = rest.find(']') {
                    let inner = &rest[1..close];
                    let after = &rest[close + 1..];
                    if after.starts_with('(') {
                        if let Some(url_end) = after[1..].find(')') {
                            let url = &after[1..1 + url_end];
                            out.push_str(&format!("<a href=\"{}\">{}</a>", esc(url), inline_html(inner)));
                            rest = &after[1 + url_end + 1..];
                            continue;
                        }
                    }
                }
                out.push('[');
                rest = &rest[1..];
            }
            _ => unreachable!(),
        }
    }
    out
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Write HTML to a file.
pub fn write_html(doc: &Document, path: &str, title: &str) -> std::io::Result<()> {
    std::fs::write(path, render_html(doc, title))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc() -> Document {
        let mut d = Document::default();
        d.blocks.push(Block::heading(1, "Title"));
        d.blocks.push(Block::paragraph("Hello **bold** and `code`"));
        d.blocks.push(Block { kind: BlockKind::Code, text: "fn main() {}".into(), lang: "rust".into(), number: None });
        d
    }

    #[test]
    fn test_render() {
        let html = render_html(&doc(), "My Doc");
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<code>code</code>"));
        assert!(html.contains("language-rust"));
    }

    #[test]
    fn test_link() {
        let mut d = Document::default();
        d.blocks.push(Block::paragraph("see [docs](https://example.com)"));
        let html = render_fragment(&d);
        assert!(html.contains("<a href=\"https://example.com\">docs</a>"));
    }
}
