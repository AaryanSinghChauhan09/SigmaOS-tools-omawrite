#![forbid(unsafe_code)]
//! omawrite - Writing and text processing tool for SigmaOS

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphStyle {
    Normal,
    Heading1,
    Heading2,
    Heading3,
    Quote,
    Code,
    ListItem,
}

impl ParagraphStyle {
    pub fn markdown_prefix(&self) -> &str {
        match self {
            Self::Heading1 => "# ",
            Self::Heading2 => "## ",
            Self::Heading3 => "### ",
            Self::Quote => "> ",
            Self::Code => "```\n",
            Self::ListItem => "- ",
            Self::Normal => "",
        }
    }
    pub fn markdown_suffix(&self) -> &str {
        match self {
            Self::Code => "\n```",
            _ => "",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub id: usize,
    pub text: String,
    pub style: ParagraphStyle,
}

impl Paragraph {
    pub fn new(id: usize, text: &str, style: ParagraphStyle) -> Self {
        Self {
            id,
            text: text.to_string(),
            style,
        }
    }
    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
    pub fn to_markdown(&self) -> String {
        format!(
            "{}{}{}",
            self.style.markdown_prefix(),
            self.text,
            self.style.markdown_suffix()
        )
    }
}

#[derive(Debug)]
pub struct Document {
    pub title: String,
    paragraphs: Vec<Paragraph>,
    next_id: usize,
}

impl Document {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            paragraphs: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_paragraph(&mut self, text: &str, style: ParagraphStyle) -> usize {
        let id = self.next_id;
        self.paragraphs.push(Paragraph::new(id, text, style));
        self.next_id += 1;
        id
    }

    pub fn get_paragraph(&self, id: usize) -> Option<&Paragraph> {
        self.paragraphs.iter().find(|p| p.id == id)
    }

    pub fn remove_paragraph(&mut self, id: usize) -> Result<Paragraph, String> {
        if let Some(pos) = self.paragraphs.iter().position(|p| p.id == id) {
            Ok(self.paragraphs.remove(pos))
        } else {
            Err(format!("Paragraph {} not found", id))
        }
    }

    pub fn word_count(&self) -> usize {
        self.paragraphs.iter().map(|p| p.word_count()).sum()
    }

    pub fn paragraph_count(&self) -> usize {
        self.paragraphs.len()
    }

    pub fn to_markdown(&self) -> String {
        let mut lines = vec![format!("# {}", self.title), String::new()];
        for para in &self.paragraphs {
            lines.push(para.to_markdown());
            lines.push(String::new());
        }
        lines.join("\n")
    }

    pub fn search(&self, query: &str) -> Vec<&Paragraph> {
        let q = query.to_lowercase();
        self.paragraphs
            .iter()
            .filter(|p| p.text.to_lowercase().contains(&q))
            .collect()
    }
}


#[cfg(test)]
mod oma_write_tests {
    use super::*;

    #[test]
    fn test_quote_and_code_markdown_aaa() {
        // Arrange
        let p_quote = Paragraph::new(0, "A great quote", ParagraphStyle::Quote);
        let p_code = Paragraph::new(1, "let x = 42;", ParagraphStyle::Code);
        // Act
        let md_quote = p_quote.to_markdown();
        let md_code = p_code.to_markdown();
        // Assert
        assert_eq!(md_quote, "> A great quote");
        assert!(md_code.starts_with("```
let x = 42;"));
    }

    #[test]
    fn test_remove_paragraph_aaa() {
        // Arrange
        let mut doc = Document::new("Doc");
        let id = doc.add_paragraph("Temporary text", ParagraphStyle::Normal);
        assert_eq!(doc.paragraph_count(), 1);
        // Act
        let removed = doc.remove_paragraph(id).unwrap();
        // Assert
        assert_eq!(removed.text, "Temporary text");
        assert_eq!(doc.paragraph_count(), 0);
        assert!(doc.remove_paragraph(999).is_err());
    }

    #[test]
    fn test_search_paragraphs_aaa() {
        // Arrange
        let mut doc = Document::new("Notes");
        doc.add_paragraph("Rust programming language", ParagraphStyle::Normal);
        doc.add_paragraph("Python scripting", ParagraphStyle::Normal);
        doc.add_paragraph("Systems architecture in Rust", ParagraphStyle::Heading2);
        // Act
        let matches = doc.search("rust");
        // Assert
        assert_eq!(matches.len(), 2);
    }
}
