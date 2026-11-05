//! Structured diagnostics and source-location reporting.
//!
//! This module is independent of the lexer, parser, semantic analyzer, and
//! backend. Frontend stages can construct diagnostics with source spans and
//! the source map can resolve them for presentation.

use std::fmt;

use unicode_width::UnicodeWidthStr;

use crate::source::{SourceMap, SourceSpan};

/// The impact a diagnostic has on the current operation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
}

impl DiagnosticSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
        }
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// The primary compiler boundary or reason associated with a diagnostic.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiagnosticCategory {
    Source,
    Lexical,
    Syntax,
    Semantic,
    Unsupported,
    Profile,
    Target,
    Configuration,
    Toolchain,
    Runtime,
}

impl DiagnosticCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Lexical => "lexical",
            Self::Syntax => "syntax",
            Self::Semantic => "semantic",
            Self::Unsupported => "unsupported",
            Self::Profile => "profile",
            Self::Target => "target",
            Self::Configuration => "configuration",
            Self::Toolchain => "toolchain",
            Self::Runtime => "runtime",
        }
    }
}

impl fmt::Display for DiagnosticCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Additional source context associated with a diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelatedInformation {
    pub span: SourceSpan,
    pub message: String,
}

/// A structured compiler diagnostic.
///
/// Codes are owned by the caller and should remain stable once published.
/// Source spans use the UTF-8 byte-offset convention defined by SourceSpan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub code: String,
    pub severity: DiagnosticSeverity,
    pub category: DiagnosticCategory,
    pub message: String,
    pub primary_span: Option<SourceSpan>,
    pub related: Vec<RelatedInformation>,
    pub notes: Vec<String>,
    pub help: Option<String>,
    pub references: Vec<String>,
}

impl Diagnostic {
    /// Creates a diagnostic without a source location or supplemental context.
    pub fn new(
        code: impl Into<String>,
        severity: DiagnosticSeverity,
        category: DiagnosticCategory,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            category,
            message: message.into(),
            primary_span: None,
            related: Vec::new(),
            notes: Vec::new(),
            help: None,
            references: Vec::new(),
        }
    }

    /// Sets the primary source range.
    pub fn with_primary_span(mut self, span: SourceSpan) -> Self {
        self.primary_span = Some(span);
        self
    }

    /// Adds a related source range and its explanation.
    pub fn with_related(mut self, span: SourceSpan, message: impl Into<String>) -> Self {
        self.related.push(RelatedInformation {
            span,
            message: message.into(),
        });
        self
    }

    /// Adds a supporting note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Adds a safe, actionable next step.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Adds a documentation, issue, or evidence reference.
    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.references.push(reference.into());
        self
    }

    /// Renders a deterministic human-readable diagnostic.
    ///
    /// Invalid or unavailable source locations are omitted rather than
    /// panicking. Source names and supplemental text have control characters
    /// escaped so they cannot inject extra terminal lines.
    pub fn render(&self, sources: &SourceMap) -> String {
        let mut output = String::new();

        if let Some(span) = self.primary_span
            && let Some(location) = render_location(sources, span)
        {
            output.push_str(&location);
            output.push_str(": ");
        }

        output.push_str(self.severity.as_str());
        output.push('[');
        output.push_str(&escape_controls(&self.code));
        output.push_str("]: ");
        output.push_str(&escape_controls(&self.message));

        if let Some(span) = self.primary_span
            && let Some(snippet) = render_snippet(sources, span)
        {
            output.push('\n');
            output.push_str(&snippet);
        }

        for related in &self.related {
            if let Some(location) = render_location(sources, related.span) {
                output.push('\n');
                output.push_str("  = related ");
                output.push_str(&location);
                output.push_str(": ");
                output.push_str(&escape_controls(&related.message));
            }
        }

        for note in &self.notes {
            output.push('\n');
            output.push_str("  = note: ");
            output.push_str(&escape_controls(note));
        }

        if let Some(help) = &self.help {
            output.push('\n');
            output.push_str("  = help: ");
            output.push_str(&escape_controls(help));
        }

        for reference in &self.references {
            output.push('\n');
            output.push_str("  = reference: ");
            output.push_str(&escape_controls(reference));
        }

        output
    }
}

fn render_location(sources: &SourceMap, span: SourceSpan) -> Option<String> {
    let file = sources.get(span.source)?;
    // Validate both bounds and UTF-8 boundaries before presenting coordinates.
    sources.slice(span)?;
    let (line, column) = file.line_column(span.start)?;
    Some(format!("{}:{line}:{column}", escape_controls(file.name())))
}

fn render_snippet(sources: &SourceMap, span: SourceSpan) -> Option<String> {
    let file = sources.get(span.source)?;
    sources.slice(span)?;
    let (line, column) = file.line_column(span.start)?;
    let line_text = file.line_text(line)?;

    let prefix: String = line_text.chars().take(column.saturating_sub(1)).collect();
    let selected_count = if span.is_empty() {
        0
    } else {
        let (end_line, end_column) = file.line_column(span.end)?;
        if end_line == line {
            end_column.saturating_sub(column)
        } else {
            line_text
                .chars()
                .count()
                .saturating_sub(column.saturating_sub(1))
        }
    };
    let selected: String = line_text
        .chars()
        .skip(column.saturating_sub(1))
        .take(selected_count)
        .collect();

    let prefix_width = display_width(&prefix);
    let marker_width = if span.is_empty() {
        1
    } else {
        display_width(&selected).max(1)
    };
    let line_number_width = line.to_string().len();

    Some(format!(
        "  {line:>line_number_width$} | {}\n  {:>line_number_width$} | {}{}",
        escape_controls(line_text),
        "",
        " ".repeat(prefix_width),
        "^".repeat(marker_width),
    ))
}

fn display_width(text: &str) -> usize {
    let mut width = 0;
    let mut visible = String::with_capacity(text.len());

    for character in text.chars() {
        if character == '\t' || character.is_control() {
            width += UnicodeWidthStr::width(visible.as_str());
            visible.clear();
            width += if character == '\t' {
                2
            } else {
                character.escape_default().count()
            };
        } else {
            visible.push(character);
        }
    }

    width + UnicodeWidthStr::width(visible.as_str())
}

fn escape_controls(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        if character.is_control() {
            escaped.extend(character.escape_default());
        } else {
            escaped.push(character);
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::{Diagnostic, DiagnosticCategory, DiagnosticSeverity};
    use crate::source::{SourceMap, SourceSpan};

    #[test]
    fn renders_source_location_unicode_excerpt_and_caret() {
        let mut sources = SourceMap::new();
        let source = sources.add_source("src/main.gfa", "PRINT \"héllo\"\r\n");
        let span = SourceSpan::new(source, 7, 13);

        let diagnostic = Diagnostic::new(
            "GFA-LEX-0001",
            DiagnosticSeverity::Error,
            DiagnosticCategory::Lexical,
            "invalid token",
        )
        .with_primary_span(span);

        assert_eq!(
            diagnostic.render(&sources),
            "src/main.gfa:1:8: error[GFA-LEX-0001]: invalid token\n  1 | PRINT \"héllo\"\n    |        ^^^^^"
        );
    }

    #[test]
    fn caret_width_matches_unicode_terminal_cells() {
        let mut sources = SourceMap::new();

        let wide_source = sources.add_source("wide.gfa", r#"PRINT "界""#);
        let wide = Diagnostic::new(
            "GFA-LEX-0002",
            DiagnosticSeverity::Error,
            DiagnosticCategory::Lexical,
            "invalid token",
        )
        .with_primary_span(SourceSpan::new(wide_source, 7, 10));
        let wide_rendered = wide.render(&sources);
        assert!(wide_rendered.ends_with(&format!("  1 | {}^^", " ".repeat(7))));

        let combining_source = sources.add_source("combining.gfa", "PRINT \"éx\"");
        let combining = Diagnostic::new(
            "GFA-LEX-0003",
            DiagnosticSeverity::Error,
            DiagnosticCategory::Lexical,
            "invalid token",
        )
        .with_primary_span(SourceSpan::new(combining_source, 7, 10));
        let combining_rendered = combining.render(&sources);
        assert!(combining_rendered.ends_with(&format!("  1 | {}^", " ".repeat(7))));

        let tab_source = sources.add_source("tab.gfa", "A\tB");
        let tab = Diagnostic::new(
            "GFA-LEX-0004",
            DiagnosticSeverity::Error,
            DiagnosticCategory::Lexical,
            "invalid token",
        )
        .with_primary_span(SourceSpan::new(tab_source, 2, 3));
        let tab_rendered = tab.render(&sources);
        assert!(tab_rendered.ends_with("  1 |    ^"));
    }

    #[test]
    fn renders_related_information_notes_help_and_references() {
        let mut sources = SourceMap::new();
        let source = sources.add_source("main.gfa", "A\nB");
        let diagnostic = Diagnostic::new(
            "GFA-SEM-0001",
            DiagnosticSeverity::Warning,
            DiagnosticCategory::Semantic,
            "possible compatibility issue",
        )
        .with_primary_span(SourceSpan::new(source, 0, 1))
        .with_related(SourceSpan::new(source, 2, 3), "related declaration")
        .with_note("selected dialect: example/1")
        .with_help("check the compatibility matrix")
        .with_reference("docs/architecture/COMPATIBILITY-MATRIX.md");

        let rendered = diagnostic.render(&sources);
        assert!(rendered.contains("main.gfa:1:1: warning[GFA-SEM-0001]"));
        assert!(rendered.contains("related main.gfa:2:1: related declaration"));
        assert!(rendered.contains("= note: selected dialect: example/1"));
        assert!(rendered.contains("= help: check the compatibility matrix"));
        assert!(rendered.contains("= reference: docs/architecture/COMPATIBILITY-MATRIX.md"));
    }

    #[test]
    fn invalid_spans_are_omitted_without_panicking() {
        let mut sources = SourceMap::new();
        let source = sources.add_source("main.gfa", "x");
        let diagnostic = Diagnostic::new(
            "GFA-SRC-0001",
            DiagnosticSeverity::Error,
            DiagnosticCategory::Source,
            "source location unavailable",
        )
        .with_primary_span(SourceSpan::new(source, 0, 2));

        assert_eq!(
            diagnostic.render(&sources),
            "error[GFA-SRC-0001]: source location unavailable"
        );
    }

    #[test]
    fn escapes_control_characters_in_names_messages_and_excerpts() {
        let mut sources = SourceMap::new();
        let source = sources.add_source("bad\nname.gfa", "A\u{0007}B");
        let diagnostic = Diagnostic::new(
            "GFA-SRC-0002",
            DiagnosticSeverity::Error,
            DiagnosticCategory::Source,
            "bad\u{0007}message",
        )
        .with_primary_span(SourceSpan::new(source, 1, 2));

        let rendered = diagnostic.render(&sources);
        assert!(rendered.starts_with("bad\\nname.gfa:1:2: error[GFA-SRC-0002]: bad\\u{7}message"));
        assert!(rendered.contains("A\\u{7}B"));
        assert!(rendered.ends_with("  1 |  ^^^^^"));
    }

    #[test]
    fn empty_span_still_has_a_caret_and_out_of_bounds_span_is_safe() {
        let mut sources = SourceMap::new();
        let source = sources.add_source("empty.gfa", "");
        let empty = Diagnostic::new(
            "GFA-PARSE-0001",
            DiagnosticSeverity::Error,
            DiagnosticCategory::Syntax,
            "expected statement",
        )
        .with_primary_span(SourceSpan::new(source, 0, 0));
        assert!(empty.render(&sources).ends_with("  1 | \n    | ^"));

        let invalid = Diagnostic::new(
            "GFA-PARSE-0002",
            DiagnosticSeverity::Error,
            DiagnosticCategory::Syntax,
            "invalid range",
        )
        .with_primary_span(SourceSpan::new(source, 1, 2));
        assert_eq!(
            invalid.render(&sources),
            "error[GFA-PARSE-0002]: invalid range"
        );
    }
}
