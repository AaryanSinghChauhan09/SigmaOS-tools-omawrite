//! omawrite — minimalist markdown writing tool CLI.

use sigmaos_tools_omawrite::{Document, reading_stats};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: omawrite <file.md> [--stats]");
        std::process::exit(1);
    }
    let text = match std::fs::read_to_string(&args[0]) {
        Ok(t) => t,
        Err(e) => { eprintln!("omawrite: {e}"); std::process::exit(1); }
    };
    let doc = Document::from_markdown(&text);
    if args.iter().any(|a| a == "--stats") {
        let s = reading_stats(&doc.plain_text());
        println!("{} words, {} chars, ~{} min read, {} paragraphs",
            s.words, s.chars_with_spaces, s.reading_minutes, s.paragraphs);
    } else {
        println!("{} blocks", doc.blocks.len());
    }
}
