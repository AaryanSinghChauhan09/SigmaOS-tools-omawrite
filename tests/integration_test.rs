//! Integration tests for sigmaos-tools-omawrite.

use sigmaos_tools_omawrite::*;

#[test]
fn test_roundtrip() {
    let md = "# Title\n\nPara **bold** text.\n\n- a\n- b\n\n1. one\n2. two\n\n```rust\nfn x() {}\n```\n";
    let doc = Document::from_markdown(md);
    let out = doc.to_markdown();
    assert!(out.contains("# Title"));
    assert!(out.contains("- a"));
    assert!(out.contains("1. one"));
    assert!(out.contains("```rust"));
}

#[test]
fn test_editing_ops() {
    let mut doc = Document::new();
    doc.blocks[0].text = "hello world".into();
    doc.split_block(5);
    assert_eq!(doc.blocks[0].text, "hello");
    assert_eq!(doc.blocks[1].text, " world");
    doc.merge_with_previous();
    assert_eq!(doc.blocks.len(), 1);
}

#[test]
fn test_stats() {
    let doc = Document::from_markdown("One two three.\n\nFour five.");
    let s = reading_stats(&doc.plain_text());
    assert_eq!(s.words, 5);
    assert_eq!(s.paragraphs, 2);
}
