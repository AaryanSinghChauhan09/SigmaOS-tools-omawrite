use sigmaos_tools_omawrite::{Document, ParagraphStyle};

#[test]
fn test_add_paragraph_aaa() {
    // Arrange
    let mut doc = Document::new("My Doc");
    // Act
    let id = doc.add_paragraph("Hello world", ParagraphStyle::Normal);
    // Assert
    assert_eq!(doc.paragraph_count(), 1);
    assert_eq!(doc.get_paragraph(id).unwrap().text, "Hello world");
}

#[test]
fn test_word_count_aaa() {
    // Arrange
    let mut doc = Document::new("Test");
    doc.add_paragraph("one two three", ParagraphStyle::Normal);
    doc.add_paragraph("four five", ParagraphStyle::Normal);
    // Act
    let count = doc.word_count();
    // Assert
    assert_eq!(count, 5);
}

#[test]
fn test_to_markdown_heading_aaa() {
    // Arrange
    let mut doc = Document::new("My Document");
    doc.add_paragraph("Introduction", ParagraphStyle::Heading1);
    // Act
    let md = doc.to_markdown();
    // Assert
    assert!(md.contains("# My Document"));
    assert!(md.contains("# Introduction"));
}
