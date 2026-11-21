//! Document model — block-structured writing document.

/// Block kind (upstream ParagraphStyle extended).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    /// Body paragraph.
    Paragraph,
    /// Heading level 1-6.
    Heading(u8),
    /// Blockquote.
    Quote,
    /// Fenced code block.
    Code,
    /// Unordered list item.
    ListItem,
    /// Ordered list item.
    OrderedItem,
    /// Horizontal rule.
    Rule,
    /// Image.
    Image,
}

/// A document block.
#[derive(Debug, Clone)]
pub struct Block {
    /// Kind.
    pub kind: BlockKind,
    /// Text content (markdown inline syntax preserved).
    pub text: String,
    /// Code block language (Code only).
    pub lang: String,
    /// Ordered list number (OrderedItem only).
    pub number: Option<u32>,
}

impl Block {
    /// New paragraph.
    pub fn paragraph(text: impl Into<String>) -> Self {
        Self { kind: BlockKind::Paragraph, text: text.into(), lang: String::new(), number: None }
    }

    /// New heading.
    pub fn heading(level: u8, text: impl Into<String>) -> Self {
        Self { kind: BlockKind::Heading(level.min(6)), text: text.into(), lang: String::new(), number: None }
    }
}

/// The document: ordered blocks + cursor position.
#[derive(Debug, Default)]
pub struct Document {
    /// Blocks in order.
    pub blocks: Vec<Block>,
    /// Cursor block index.
    pub cursor: usize,
}

impl Document {
    /// New empty document (starts with one paragraph).
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::paragraph("")],
            cursor: 0,
        }
    }

    /// Load from markdown text.
    pub fn from_markdown(text: &str) -> Self {
        crate::markdown::parse_document(text)
    }

    /// Serialize to markdown.
    pub fn to_markdown(&self) -> String {
        crate::markdown::render_markdown(self)
    }

    /// Current block.
    pub fn current(&self) -> Option<&Block> {
        self.blocks.get(self.cursor)
    }

    /// Mutable current block.
    pub fn current_mut(&mut self) -> Option<&mut Block> {
        self.blocks.get_mut(self.cursor)
    }

    /// Insert a new block after cursor.
    pub fn insert_block(&mut self, block: Block) {
        self.cursor += 1;
        self.blocks.insert(self.cursor, block);
    }

    /// Split current block at cursor: text before stays, after goes to new block.
    /// `pos` is char offset in current block text.
    pub fn split_block(&mut self, pos: usize) {
        if let Some(b) = self.current_mut() {
            let text: Vec<char> = b.text.chars().collect();
            let pos = pos.min(text.len());
            let after: String = text[pos..].iter().collect();
            b.text = text[..pos].iter().collect();
            let new_block = Block {
                kind: b.kind,
                text: after,
                lang: b.lang.clone(),
                number: b.number,
            };
            self.insert_block(new_block);
        }
    }

    /// Merge current block into previous (backspace at line start).
    pub fn merge_with_previous(&mut self) {
        if self.cursor > 0 && self.cursor < self.blocks.len() {
            let cur = self.blocks.remove(self.cursor);
            self.cursor -= 1;
            if let Some(prev) = self.blocks.get_mut(self.cursor) {
                prev.text.push('\n');
                prev.text.push_str(&cur.text);
            }
        }
    }

    /// Move cursor up/down (clamped).
    pub fn move_cursor(&mut self, delta: isize) {
        let new = (self.cursor as isize + delta).clamp(0, self.blocks.len() as isize - 1);
        self.cursor = new as usize;
    }

    /// Total characters across all blocks.
    pub fn char_count(&self) -> usize {
        self.blocks.iter().map(|b| b.text.chars().count()).sum()
    }

    /// Plain text (blocks joined by newlines, markdown stripped of block markers).
    pub fn plain_text(&self) -> String {
        self.blocks
            .iter()
            .map(|b| b.text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_blocks() {
        let mut d = Document::new();
        d.insert_block(Block::heading(1, "Title"));
        d.insert_block(Block::paragraph("Hello world"));
        assert_eq!(d.blocks.len(), 3);
        assert_eq!(d.blocks[1].kind, BlockKind::Heading(1));
    }

    #[test]
    fn test_split_block() {
        let mut d = Document::new();
        d.blocks[0].text = "hello world".into();
        d.split_block(5);
        assert_eq!(d.blocks[0].text, "hello");
        assert_eq!(d.blocks[1].text, " world");
        assert_eq!(d.cursor, 1);
    }

    #[test]
    fn test_merge_previous() {
        let mut d = Document::new();
        d.insert_block(Block::paragraph("second"));
        d.merge_with_previous();
        assert_eq!(d.blocks.len(), 1);
        assert!(d.blocks[0].text.contains("second"));
    }

    #[test]
    fn test_markdown_roundtrip() {
        let md = "# Hello\n\nWorld **bold**\n\n- item1\n- item2\n";
        let d = Document::from_markdown(md);
        let out = d.to_markdown();
        assert!(out.contains("# Hello"));
        assert!(out.contains("World **bold**"));
        assert!(out.contains("- item1"));
    }
}
