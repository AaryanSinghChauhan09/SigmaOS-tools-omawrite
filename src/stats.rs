//! Writing statistics — words, reading time, etc.

/// Reading statistics for a document.
#[derive(Debug, Clone, Default)]
pub struct ReadingStats {
    /// Word count.
    pub words: usize,
    /// Character count (no whitespace).
    pub chars: usize,
    /// Character count (with whitespace).
    pub chars_with_spaces: usize,
    /// Estimated reading time in minutes (200 wpm).
    pub reading_minutes: u32,
    /// Sentence count (rough: `.`, `!`, `?` terminators).
    pub sentences: usize,
    /// Paragraph count.
    pub paragraphs: usize,
}

/// Compute stats for a document's plain text.
pub fn reading_stats(text: &str) -> ReadingStats {
    let words = text.split_whitespace().count();
    let chars_with_spaces = text.chars().count();
    let chars = text.chars().filter(|c| !c.is_whitespace()).count();
    let sentences = text
        .chars()
        .filter(|c| matches!(c, '.' | '!' | '?'))
        .count()
        .max(1);
    let paragraphs = text.split("\n\n").filter(|p| !p.trim().is_empty()).count();
    ReadingStats {
        words,
        chars,
        chars_with_spaces,
        reading_minutes: ((words as f64) / 200.0).ceil() as u32,
        sentences,
        paragraphs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats() {
        let s = reading_stats("Hello world. This is a test.\n\nSecond paragraph here.");
        assert_eq!(s.words, 9);
        assert_eq!(s.paragraphs, 2);
        assert_eq!(s.sentences, 3);
        assert_eq!(s.reading_minutes, 1);
    }

    #[test]
    fn test_empty() {
        let s = reading_stats("");
        assert_eq!(s.words, 0);
        assert_eq!(s.sentences, 1); // min 1
    }
}
