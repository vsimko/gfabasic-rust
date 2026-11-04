//! Source identity and source-map primitives.
//!
//! This module accepts already-decoded UTF-8 text. The source dialect's file
//! encoding rules and source-composition semantics are not yet verified, so
//! this module does not read files or implement MERGE/include.

use std::sync::Arc;

/// Identifies a source file within one source map.
///
/// IDs are stable for the lifetime of the map. They are allocated in insertion
/// order and must not be persisted across compiler invocations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceId(usize);

impl SourceId {
    /// Returns the zero-based index used by this source map.
    pub const fn index(self) -> usize {
        self.0
    }
}

/// A half-open UTF-8 byte range in one source file.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SourceSpan {
    pub source: SourceId,
    pub start: usize,
    pub end: usize,
}

impl SourceSpan {
    /// Creates a span. Bounds and UTF-8 boundaries are checked by SourceMap.
    pub const fn new(source: SourceId, start: usize, end: usize) -> Self {
        Self { source, start, end }
    }

    /// Returns the number of bytes in this span.
    pub const fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns true when the span is empty.
    pub const fn is_empty(self) -> bool {
        self.start >= self.end
    }
}

/// A decoded source file retained by a source map.
#[derive(Clone, Debug)]
pub struct SourceFile {
    name: Arc<str>,
    text: Arc<str>,
    line_starts: Vec<usize>,
}

impl SourceFile {
    fn new(name: Arc<str>, text: Arc<str>) -> Self {
        let mut line_starts = vec![0];
        let bytes = text.as_bytes();
        let mut index = 0;
        while index < bytes.len() {
            match bytes[index] {
                b'\r' => {
                    index += 1;
                    if bytes.get(index) == Some(&b'\n') {
                        index += 1;
                    }
                    line_starts.push(index);
                }
                b'\n' => {
                    index += 1;
                    line_starts.push(index);
                }
                _ => index += 1,
            }
        }
        Self {
            name,
            text,
            line_starts,
        }
    }

    /// Returns the source's display name (normally a path or virtual name).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the complete decoded source text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the number of lines, including a final empty line after a
    /// trailing line ending.
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Returns a one-based (line, column) for a valid UTF-8 byte offset.
    ///
    /// Columns count Unicode scalar values, not bytes. The offset may point to
    /// the end of the file.
    pub fn line_column(&self, offset: usize) -> Option<(usize, usize)> {
        if offset > self.text.len() || !self.text.is_char_boundary(offset) {
            return None;
        }

        let line_index = self.line_starts.partition_point(|&start| start <= offset) - 1;
        let line_start = self.line_starts[line_index];
        let prefix = self.text[line_start..offset]
            .strip_suffix('\r')
            .unwrap_or(&self.text[line_start..offset]);
        let column = prefix.chars().count() + 1;
        Some((line_index + 1, column))
    }

    /// Returns a line without its line-feed or optional carriage-return.
    pub fn line_text(&self, one_based_line: usize) -> Option<&str> {
        let start = *self.line_starts.get(one_based_line.checked_sub(1)?)?;
        let mut end = self
            .line_starts
            .get(one_based_line)
            .copied()
            .unwrap_or(self.text.len());
        if end > start && self.text.as_bytes().get(end - 1) == Some(&b'\n') {
            end -= 1;
        }
        if end > start && self.text.as_bytes().get(end - 1) == Some(&b'\r') {
            end -= 1;
        }
        self.text.get(start..end)
    }
}

/// Owns source files and resolves source spans.
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    /// Creates an empty source map.
    pub const fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Adds already-decoded UTF-8 source text and returns its stable map-local ID.
    pub fn add_source(&mut self, name: impl Into<Arc<str>>, text: impl Into<Arc<str>>) -> SourceId {
        let id = SourceId(self.files.len());
        self.files.push(SourceFile::new(name.into(), text.into()));
        id
    }

    /// Returns a source file by ID.
    pub fn get(&self, id: SourceId) -> Option<&SourceFile> {
        self.files.get(id.0)
    }

    /// Returns the source text covered by a span.
    ///
    /// Returns None for an unknown source, out-of-bounds range, reversed
    /// range, or an offset that splits a UTF-8 code point.
    pub fn slice(&self, span: SourceSpan) -> Option<&str> {
        let file = self.get(span.source)?;
        file.text.get(span.start..span.end)
    }

    /// Returns the number of files in this map.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Returns true when this map contains no files.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{SourceMap, SourceSpan};

    #[test]
    fn source_ids_are_stable_and_files_are_retrievable() {
        let mut map = SourceMap::new();
        let first = map.add_source("main.gfa", "PRINT 1");
        let second = map.add_source("library.gfa", "PRINT 2");

        assert_eq!(first.index(), 0);
        assert_eq!(second.index(), 1);
        assert_eq!(map.get(first).unwrap().name(), "main.gfa");
        assert_eq!(map.get(second).unwrap().text(), "PRINT 2");
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn spans_slice_utf8_text_and_reject_invalid_boundaries() {
        let mut map = SourceMap::new();
        let source = map.add_source("unicode.gfa", "AéZ");

        assert_eq!(map.slice(SourceSpan::new(source, 1, 3)), Some("é"));
        assert_eq!(map.slice(SourceSpan::new(source, 2, 3)), None);
        assert_eq!(map.slice(SourceSpan::new(source, 3, 2)), None);
        assert_eq!(map.slice(SourceSpan::new(source, 0, 4)), Some("AéZ"));
    }

    #[test]
    fn line_and_column_are_one_based_and_count_unicode_scalars() {
        let mut map = SourceMap::new();
        let source = map.add_source("lines.gfa", "Aé\r\nB\n");

        let file = map.get(source).unwrap();
        assert_eq!(file.line_count(), 3);
        assert_eq!(file.line_column(0), Some((1, 1)));
        assert_eq!(file.line_column(1), Some((1, 2)));
        assert_eq!(file.line_column(3), Some((1, 3)));
        assert_eq!(file.line_column(4), Some((1, 3)));
        assert_eq!(file.line_column(5), Some((2, 1)));
        assert_eq!(file.line_column(7), Some((3, 1)));
        assert_eq!(file.line_text(1), Some("Aé"));
        assert_eq!(file.line_text(2), Some("B"));
        assert_eq!(file.line_text(3), Some(""));
        assert_eq!(file.line_column(2), None);
    }

    #[test]
    fn carriage_return_only_line_endings_are_supported() {
        let mut map = SourceMap::new();
        let source = map.add_source("classic.gfa", "A\rB\r");

        let file = map.get(source).unwrap();
        assert_eq!(file.line_count(), 3);
        assert_eq!(file.line_text(1), Some("A"));
        assert_eq!(file.line_text(2), Some("B"));
        assert_eq!(file.line_text(3), Some(""));
        assert_eq!(file.line_column(2), Some((2, 1)));
    }

    #[test]
    fn empty_source_has_one_empty_line() {
        let mut map = SourceMap::new();
        let source = map.add_source("empty.gfa", "");

        let file = map.get(source).unwrap();
        assert_eq!(file.line_count(), 1);
        assert_eq!(file.line_column(0), Some((1, 1)));
        assert_eq!(file.line_text(1), Some(""));
    }
}
