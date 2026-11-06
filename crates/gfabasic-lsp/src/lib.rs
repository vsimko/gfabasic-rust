//! Minimal, dependency-free Language Server Protocol transport and document store.
//!
//! This first increment implements JSON-RPC over stdio, full-document
//! synchronization, versioned document state, and UTF-16/UTF-8 position
//! conversion. Diagnostics are produced by the compiler's shared supported-subset
//! parser; no separate language semantics are implemented here.

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use gfabasic_compiler::diagnostics::DiagnosticSeverity;
use gfabasic_compiler::frontend::parse_document;
use gfabasic_compiler::source::SourceMap;

const MAX_MESSAGE_BYTES: usize = 16 * 1024 * 1024;
const MAX_HEADER_BYTES: usize = 8 * 1024;
const MAX_JSON_DEPTH: usize = 128;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextDocument {
    pub uri: String,
    pub language_id: String,
    pub version: i64,
    pub text: String,
}

#[derive(Clone, Debug, Default)]
pub struct DocumentStore {
    documents: HashMap<String, TextDocument>,
}

impl DocumentStore {
    pub fn open(&mut self, uri: String, language_id: String, version: i64, text: String) -> bool {
        if self
            .documents
            .get(&uri)
            .is_some_and(|old| version <= old.version)
        {
            return false;
        }
        self.documents.insert(
            uri.clone(),
            TextDocument {
                uri,
                language_id,
                version,
                text,
            },
        );
        true
    }

    pub fn change(&mut self, uri: &str, version: i64, text: String) -> bool {
        let Some(document) = self.documents.get_mut(uri) else {
            return false;
        };
        if version <= document.version {
            return false;
        }
        document.version = version;
        document.text = text;
        true
    }

    pub fn close(&mut self, uri: &str) -> bool {
        self.documents.remove(uri).is_some()
    }

    pub fn get(&self, uri: &str) -> Option<&TextDocument> {
        self.documents.get(uri)
    }

    pub fn len(&self) -> usize {
        self.documents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

/// Convert an LSP UTF-16 character offset on a zero-based line to a UTF-8 byte
/// offset in the complete document. Positions inside a surrogate pair,
/// beyond the line, or beyond the document are rejected.
pub fn position_to_byte_offset(text: &str, line: u32, character: u32) -> Option<usize> {
    let (line_start, line_end) = line_bounds(text, line)?;
    let line_text = &text[line_start..line_end];
    let mut utf16 = 0u32;
    for (byte, scalar) in line_text.char_indices() {
        if utf16 == character {
            return Some(line_start + byte);
        }
        let width = scalar.len_utf16() as u32;
        if character > utf16 && character < utf16 + width {
            return None;
        }
        utf16 += width;
    }
    (utf16 == character).then_some(line_end)
}

/// Convert a UTF-8 byte offset to an LSP zero-based line and UTF-16 character.
pub fn byte_offset_to_position(text: &str, offset: usize) -> Option<(u32, u32)> {
    if offset > text.len() || !text.is_char_boundary(offset) {
        return None;
    }
    let mut line = 0u32;
    let mut start = 0usize;
    loop {
        let (_, end) = line_bounds(text, line)?;
        if offset >= start && offset <= end {
            return Some((line, text[start..offset].encode_utf16().count() as u32));
        }
        let bytes = text.as_bytes();
        let mut next = end;
        if bytes.get(next) == Some(&b'\r') {
            next += 1;
            if bytes.get(next) == Some(&b'\n') {
                next += 1;
            }
        } else if bytes.get(next) == Some(&b'\n') {
            next += 1;
        } else {
            return None;
        }
        if offset < next {
            return None;
        }
        start = next;
        line += 1;
    }
}

fn line_bounds(text: &str, requested_line: u32) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut line = 0u32;
    let mut start = 0usize;
    let mut index = 0usize;
    loop {
        if line == requested_line {
            let mut end = start;
            while end < bytes.len() && bytes[end] != b'\r' && bytes[end] != b'\n' {
                end += 1;
            }
            return Some((start, end));
        }
        if index >= bytes.len() {
            return None;
        }
        match bytes[index] {
            b'\r' => {
                index += 1;
                if bytes.get(index) == Some(&b'\n') {
                    index += 1;
                }
                start = index;
                line += 1;
            }
            b'\n' => {
                index += 1;
                start = index;
                line += 1;
            }
            _ => index += 1,
        }
    }
}

/// Run the stdio JSON-RPC server until an \`exit\` notification or EOF.
pub fn run_stdio<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<()> {
    let mut documents = DocumentStore::default();
    loop {
        let Some(body) = read_message(reader)? else {
            return Ok(());
        };
        let mut outbound = Vec::new();
        let response = match parse_json(&body) {
            Ok(value) => dispatch_with_notifications(value, &mut documents, &mut outbound),
            Err(_) => Some(error_response(Json::Null, -32700, "Parse error")),
        };
        if let Some(response) = response {
            write_message(writer, &stringify(&response))?;
        }
        for notification in outbound {
            write_message(writer, &stringify(&notification))?;
        }
        if SERVER_EXITED.with(|flag| flag.replace(false)) {
            return Ok(());
        }
    }
}

thread_local! {
    static SERVER_EXITED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
fn dispatch(message: Json, documents: &mut DocumentStore) -> Option<Json> {
    dispatch_with_notifications(message, documents, &mut Vec::new())
}

fn dispatch_with_notifications(
    message: Json,
    documents: &mut DocumentStore,
    outbound: &mut Vec<Json>,
) -> Option<Json> {
    let Json::Object(object) = message else {
        return Some(error_response(Json::Null, -32600, "Invalid Request"));
    };
    let id = object.get("id").cloned();
    let method = match object.get("method") {
        Some(Json::String(method)) => method.as_str(),
        _ => return id.map(|id| error_response(id, -32600, "Invalid Request")),
    };
    let params = object.get("params").cloned().unwrap_or(Json::Null);

    match method {
        "initialize" => id.map(|id| {
            success_response(
                id,
                object_json([
                    (
                        "capabilities",
                        object_json([("textDocumentSync", Json::Number("1".into()))]),
                    ),
                    (
                        "serverInfo",
                        object_json([
                            ("name", Json::String("gfabasic-lsp".into())),
                            ("version", Json::String(env!("CARGO_PKG_VERSION").into())),
                        ]),
                    ),
                ]),
            )
        }),
        "initialized" => None,
        // There are no asynchronous requests yet, so cancellation has no
        // outstanding work to stop. Keep the standard notification harmless.
        "$/cancelRequest" => None,
        "shutdown" => id.map(|id| success_response(id, Json::Null)),
        "exit" => {
            SERVER_EXITED.with(|flag| flag.set(true));
            None
        }
        "textDocument/didOpen" => {
            if let Some(document) = params.get_path(&["textDocument"])
                && let (Some(uri), Some(language_id), Some(version), Some(text)) = (
                    document.string("uri"),
                    document.string("languageId"),
                    document.integer("version"),
                    document.string("text"),
                )
                && documents.open(uri.into(), language_id.into(), version, text.into())
                && let Some(document) = documents.get(uri)
            {
                outbound.push(diagnostic_notification(
                    &document.uri,
                    Some(document.version),
                    &document.text,
                ));
            }
            None
        }
        "textDocument/didChange" => {
            // Full synchronization is advertised. Reject ranged changes rather
            // than applying them with incomplete edit semantics.
            if let Some(document) = params.get_path(&["textDocument"])
                && let (Some(uri), Some(version)) =
                    (document.string("uri"), document.integer("version"))
                && let Some(Json::Array(changes)) = params.get("contentChanges")
                && changes.len() == 1
                && changes[0]
                    .get("range")
                    .is_none_or(|range| matches!(range, Json::Null))
                && let Some(text) = changes[0].string("text")
                && documents.change(uri, version, text.into())
                && let Some(document) = documents.get(uri)
            {
                outbound.push(diagnostic_notification(
                    &document.uri,
                    Some(document.version),
                    &document.text,
                ));
            }
            None
        }
        "textDocument/didClose" => {
            if let Some(uri) = params
                .get_path(&["textDocument"])
                .and_then(|document| document.string("uri"))
                && documents.close(uri)
            {
                outbound.push(empty_diagnostics_notification(uri));
            }
            None
        }
        _ => id.map(|id| error_response(id, -32601, "Method not found")),
    }
}

fn diagnostic_notification(uri: &str, version: Option<i64>, text: &str) -> Json {
    let mut sources = SourceMap::new();
    let source_id = sources.add_source(uri.to_owned(), text.to_owned());
    let diagnostics = match parse_document(source_id, text) {
        Ok(_) => Vec::new(),
        Err(diagnostics) => diagnostics
            .into_iter()
            .map(|diagnostic| {
                let (start, end) = match diagnostic.primary_span {
                    Some(span) => (
                        byte_offset_to_position(text, span.start).unwrap_or((0, 0)),
                        byte_offset_to_position(text, span.end).unwrap_or((0, 0)),
                    ),
                    None => ((0, 0), (0, 0)),
                };
                let severity = match diagnostic.severity {
                    DiagnosticSeverity::Error => 1,
                    DiagnosticSeverity::Warning => 2,
                    DiagnosticSeverity::Note => 3,
                };
                object_json([
                    (
                        "range",
                        object_json([("start", position_json(start)), ("end", position_json(end))]),
                    ),
                    ("severity", Json::Number(severity.to_string())),
                    ("code", Json::String(diagnostic.code)),
                    ("source", Json::String("gfabasic".into())),
                    ("message", Json::String(diagnostic.message)),
                ])
            })
            .collect(),
    };

    let mut params = HashMap::new();
    params.insert("uri".into(), Json::String(uri.into()));
    if let Some(version) = version {
        params.insert("version".into(), Json::Number(version.to_string()));
    }
    params.insert("diagnostics".into(), Json::Array(diagnostics));
    object_json([
        ("jsonrpc", Json::String("2.0".into())),
        (
            "method",
            Json::String("textDocument/publishDiagnostics".into()),
        ),
        ("params", Json::Object(params)),
    ])
}

fn empty_diagnostics_notification(uri: &str) -> Json {
    object_json([
        ("jsonrpc", Json::String("2.0".into())),
        (
            "method",
            Json::String("textDocument/publishDiagnostics".into()),
        ),
        (
            "params",
            object_json([
                ("uri", Json::String(uri.into())),
                ("diagnostics", Json::Array(Vec::new())),
            ]),
        ),
    ])
}

fn position_json((line, character): (u32, u32)) -> Json {
    object_json([
        ("line", Json::Number(line.to_string())),
        ("character", Json::Number(character.to_string())),
    ])
}

fn read_message<R: BufRead>(reader: &mut R) -> io::Result<Option<Vec<u8>>> {
    let mut content_length = None;
    let mut header_bytes = 0usize;
    loop {
        let mut line = Vec::new();
        loop {
            let available = reader.fill_buf()?;
            if available.is_empty() {
                return if header_bytes == 0 && line.is_empty() {
                    Ok(None)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "incomplete LSP headers",
                    ))
                };
            }
            let count = available
                .iter()
                .position(|byte| *byte == b'\n')
                .map_or(available.len(), |index| index + 1);
            if header_bytes
                .saturating_add(line.len())
                .saturating_add(count)
                > MAX_HEADER_BYTES
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "LSP headers too large",
                ));
            }
            let terminated = available.get(count - 1) == Some(&b'\n');
            line.extend_from_slice(&available[..count]);
            reader.consume(count);
            if terminated {
                break;
            }
        }
        header_bytes += line.len();
        let line = std::str::from_utf8(&line)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "LSP headers must be UTF-8"))?;
        if line == "\r\n" || line == "\n" {
            break;
        }
        let (name, value) = line
            .trim_end_matches(['\r', '\n'])
            .split_once(':')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid LSP header"))?;
        if name.eq_ignore_ascii_case("Content-Length") {
            if content_length.is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "duplicate Content-Length",
                ));
            }
            content_length = Some(value.trim().parse::<usize>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "invalid Content-Length")
            })?);
        }
    }
    let length = content_length
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing Content-Length"))?;
    if length > MAX_MESSAGE_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "LSP message too large",
        ));
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    Ok(Some(body))
}

fn write_message<W: Write>(writer: &mut W, body: &str) -> io::Result<()> {
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
    writer.write_all(body.as_bytes())?;
    writer.flush()
}

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Self::Object(map) => map.get(key),
            _ => None,
        }
    }
    fn get_path(&self, path: &[&str]) -> Option<&Json> {
        path.iter().try_fold(self, |value, key| value.get(key))
    }
    fn string(&self, key: &str) -> Option<&str> {
        match self.get(key)? {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
    fn integer(&self, key: &str) -> Option<i64> {
        match self.get(key)? {
            Self::Number(value) => value.parse().ok(),
            _ => None,
        }
    }
}

fn object_json<const N: usize>(fields: [(&str, Json); N]) -> Json {
    Json::Object(
        fields
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect(),
    )
}

fn success_response(id: Json, result: Json) -> Json {
    object_json([
        ("jsonrpc", Json::String("2.0".into())),
        ("id", id),
        ("result", result),
    ])
}

fn error_response(id: Json, code: i64, message: &str) -> Json {
    object_json([
        ("jsonrpc", Json::String("2.0".into())),
        ("id", id),
        (
            "error",
            object_json([
                ("code", Json::Number(code.to_string())),
                ("message", Json::String(message.into())),
            ]),
        ),
    ])
}

fn stringify(value: &Json) -> String {
    match value {
        Json::Null => "null".into(),
        Json::Bool(value) => value.to_string(),
        Json::Number(value) => value.clone(),
        Json::String(value) => quote(value),
        Json::Array(values) => format!(
            "[{}]",
            values.iter().map(stringify).collect::<Vec<_>>().join(",")
        ),
        Json::Object(values) => {
            let mut entries: Vec<_> = values.iter().collect();
            entries.sort_by_key(|(key, _)| *key);
            format!(
                "{{{}}}",
                entries
                    .into_iter()
                    .map(|(key, value)| format!("{}:{}", quote(key), stringify(value)))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
}

fn quote(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{0008}' => output.push_str("\\b"),
            '\u{000c}' => output.push_str("\\f"),
            c if c <= '\u{001f}' => output.push_str(&format!("\\u{:04x}", c as u32)),
            c => output.push(c),
        }
    }
    output.push('"');
    output
}

#[derive(Clone, Copy, Debug)]
struct JsonParseError;

fn parse_json(input: &[u8]) -> Result<Json, JsonParseError> {
    let mut parser = JsonParser {
        input,
        offset: 0,
        depth: 0,
    };
    let value = parser.value()?;
    parser.whitespace();
    (parser.offset == input.len())
        .then_some(value)
        .ok_or(JsonParseError)
}

struct JsonParser<'a> {
    input: &'a [u8],
    offset: usize,
    depth: usize,
}

impl JsonParser<'_> {
    fn whitespace(&mut self) {
        while self
            .input
            .get(self.offset)
            .is_some_and(|byte| matches!(byte, b' ' | b'\\t' | b'\\n' | b'\\r'))
        {
            self.offset += 1;
        }
    }

    fn value(&mut self) -> Result<Json, JsonParseError> {
        if self.depth >= MAX_JSON_DEPTH {
            return Err(JsonParseError);
        }
        self.depth += 1;
        let result = self.value_inner();
        self.depth -= 1;
        result
    }

    fn value_inner(&mut self) -> Result<Json, JsonParseError> {
        self.whitespace();
        match self.input.get(self.offset).copied().ok_or(JsonParseError)? {
            b'n' => {
                self.literal(b"null")?;
                Ok(Json::Null)
            }
            b't' => {
                self.literal(b"true")?;
                Ok(Json::Bool(true))
            }
            b'f' => {
                self.literal(b"false")?;
                Ok(Json::Bool(false))
            }
            b'"' => self.string().map(Json::String),
            b'[' => self.array(),
            b'{' => self.object(),
            b'-' | b'0'..=b'9' => self.number(),
            _ => Err(JsonParseError),
        }
    }

    fn literal(&mut self, literal: &[u8]) -> Result<(), JsonParseError> {
        if self.input.get(self.offset..self.offset + literal.len()) == Some(literal) {
            self.offset += literal.len();
            Ok(())
        } else {
            Err(JsonParseError)
        }
    }

    fn string(&mut self) -> Result<String, JsonParseError> {
        self.offset += 1;
        let mut result = String::new();
        loop {
            let byte = *self.input.get(self.offset).ok_or(JsonParseError)?;
            match byte {
                b'"' => {
                    self.offset += 1;
                    return Ok(result);
                }
                b'\\' => {
                    self.offset += 1;
                    let escaped = *self.input.get(self.offset).ok_or(JsonParseError)?;
                    self.offset += 1;
                    match escaped {
                        b'"' => result.push('"'),
                        b'\\' => result.push('\\'),
                        b'/' => result.push('/'),
                        b'b' => result.push('\u{0008}'),
                        b'f' => result.push('\u{000c}'),
                        b'n' => result.push('\n'),
                        b'r' => result.push('\r'),
                        b't' => result.push('\t'),
                        b'u' => {
                            let first = self.hex_quad()?;
                            let scalar = if (0xd800..=0xdbff).contains(&first) {
                                if self.input.get(self.offset..self.offset + 2) != Some(b"\\u") {
                                    return Err(JsonParseError);
                                }
                                self.offset += 2;
                                let second = self.hex_quad()?;
                                if !(0xdc00..=0xdfff).contains(&second) {
                                    return Err(JsonParseError);
                                }
                                0x10000
                                    + (((first - 0xd800) as u32) << 10)
                                    + (second - 0xdc00) as u32
                            } else if (0xdc00..=0xdfff).contains(&first) {
                                return Err(JsonParseError);
                            } else {
                                first as u32
                            };
                            result.push(char::from_u32(scalar).ok_or(JsonParseError)?);
                        }
                        _ => return Err(JsonParseError),
                    }
                }
                0x00..=0x1f => return Err(JsonParseError),
                _ => {
                    let tail = std::str::from_utf8(&self.input[self.offset..])
                        .map_err(|_| JsonParseError)?;
                    let character = tail.chars().next().ok_or(JsonParseError)?;
                    result.push(character);
                    self.offset += character.len_utf8();
                }
            }
        }
    }

    fn hex_quad(&mut self) -> Result<u16, JsonParseError> {
        let bytes = self
            .input
            .get(self.offset..self.offset + 4)
            .ok_or(JsonParseError)?;
        let mut value = 0u16;
        for &byte in bytes {
            value = value.checked_mul(16).ok_or(JsonParseError)?
                + match byte {
                    b'0'..=b'9' => (byte - b'0') as u16,
                    b'a'..=b'f' => (byte - b'a' + 10) as u16,
                    b'A'..=b'F' => (byte - b'A' + 10) as u16,
                    _ => return Err(JsonParseError),
                };
        }
        self.offset += 4;
        Ok(value)
    }

    fn array(&mut self) -> Result<Json, JsonParseError> {
        self.offset += 1;
        let mut values = Vec::new();
        self.whitespace();
        if self.input.get(self.offset) == Some(&b']') {
            self.offset += 1;
            return Ok(Json::Array(values));
        }
        loop {
            values.push(self.value()?);
            self.whitespace();
            match self.input.get(self.offset) {
                Some(b',') => self.offset += 1,
                Some(b']') => {
                    self.offset += 1;
                    return Ok(Json::Array(values));
                }
                _ => return Err(JsonParseError),
            }
        }
    }

    fn object(&mut self) -> Result<Json, JsonParseError> {
        self.offset += 1;
        let mut values = HashMap::new();
        self.whitespace();
        if self.input.get(self.offset) == Some(&b'}') {
            self.offset += 1;
            return Ok(Json::Object(values));
        }
        loop {
            self.whitespace();
            if self.input.get(self.offset) != Some(&b'"') {
                return Err(JsonParseError);
            }
            let key = self.string()?;
            self.whitespace();
            if self.input.get(self.offset) != Some(&b':') {
                return Err(JsonParseError);
            }
            self.offset += 1;
            let value = self.value()?;
            values.insert(key, value);
            self.whitespace();
            match self.input.get(self.offset) {
                Some(b',') => self.offset += 1,
                Some(b'}') => {
                    self.offset += 1;
                    return Ok(Json::Object(values));
                }
                _ => return Err(JsonParseError),
            }
        }
    }

    fn number(&mut self) -> Result<Json, JsonParseError> {
        let start = self.offset;
        if self.input.get(self.offset) == Some(&b'-') {
            self.offset += 1;
        }
        match self.input.get(self.offset) {
            Some(b'0') => self.offset += 1,
            Some(b'1'..=b'9') => {
                self.offset += 1;
                while self.input.get(self.offset).is_some_and(u8::is_ascii_digit) {
                    self.offset += 1;
                }
            }
            _ => return Err(JsonParseError),
        }
        if self.input.get(self.offset) == Some(&b'.') {
            self.offset += 1;
            let start_fraction = self.offset;
            while self.input.get(self.offset).is_some_and(u8::is_ascii_digit) {
                self.offset += 1;
            }
            if self.offset == start_fraction {
                return Err(JsonParseError);
            }
        }
        if self
            .input
            .get(self.offset)
            .is_some_and(|b| matches!(b, b'e' | b'E'))
        {
            self.offset += 1;
            if self
                .input
                .get(self.offset)
                .is_some_and(|b| matches!(b, b'+' | b'-'))
            {
                self.offset += 1;
            }
            let start_exponent = self.offset;
            while self.input.get(self.offset).is_some_and(u8::is_ascii_digit) {
                self.offset += 1;
            }
            if self.offset == start_exponent {
                return Err(JsonParseError);
            }
        }
        let number =
            std::str::from_utf8(&self.input[start..self.offset]).map_err(|_| JsonParseError)?;
        Ok(Json::Number(number.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_lifecycle_and_stale_versions() {
        let mut store = DocumentStore::default();
        assert!(store.open(
            "file:///a.gfa".into(),
            "gfabasic".into(),
            1,
            "PRINT 1".into()
        ));
        assert!(!store.change("file:///a.gfa", 1, "PRINT 2".into()));
        assert!(!store.change("file:///a.gfa", 0, "PRINT 2".into()));
        assert!(store.change("file:///a.gfa", 2, "PRINT 2".into()));
        assert_eq!(store.get("file:///a.gfa").unwrap().text, "PRINT 2");
        assert!(store.close("file:///a.gfa"));
        assert!(store.is_empty());
    }

    #[test]
    fn utf16_positions_round_trip_non_bmp_and_combining_characters() {
        let text = "A😀e\u{301}\r\nB";
        for offset in [0, 1, 5, 6, 8, 10, 11] {
            if text.is_char_boundary(offset) {
                let (line, character) = byte_offset_to_position(text, offset).unwrap();
                assert_eq!(position_to_byte_offset(text, line, character), Some(offset));
            }
        }
        assert_eq!(position_to_byte_offset(text, 0, 2), None);
        assert_eq!(position_to_byte_offset(text, 0, 3), Some(5));
        assert_eq!(position_to_byte_offset(text, 1, 1), Some(11));
    }

    #[test]
    fn parses_and_serializes_json_with_unicode_escapes() {
        let parsed = parse_json(br#"{"s":"\ud83d\ude00","n":4,"a":[true,null]}"#).unwrap();
        assert_eq!(parsed.string("s"), Some("😀"));
        let encoded = stringify(&parsed);
        assert_eq!(parse_json(encoded.as_bytes()).unwrap(), parsed);
    }

    #[test]
    fn dispatch_tracks_full_document_open_change_and_close() {
        let mut store = DocumentStore::default();
        let opened = parse_json(br#"{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///x.gfa","languageId":"gfabasic","version":1,"text":"PRINT 1"}}}"#).unwrap();
        assert!(dispatch(opened, &mut store).is_none());
        assert_eq!(store.get("file:///x.gfa").unwrap().version, 1);

        let changed = parse_json(br#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":"file:///x.gfa","version":2},"contentChanges":[{"text":"PRINT 2"}]}}"#).unwrap();
        assert!(dispatch(changed, &mut store).is_none());
        assert_eq!(store.get("file:///x.gfa").unwrap().text, "PRINT 2");
        assert_eq!(store.get("file:///x.gfa").unwrap().version, 2);

        let stale = parse_json(br#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":"file:///x.gfa","version":1},"contentChanges":[{"text":"PRINT 0"}]}}"#).unwrap();
        dispatch(stale, &mut store);
        assert_eq!(store.get("file:///x.gfa").unwrap().text, "PRINT 2");

        let closed = parse_json(br#"{"jsonrpc":"2.0","method":"textDocument/didClose","params":{"textDocument":{"uri":"file:///x.gfa"}}}"#).unwrap();
        dispatch(closed, &mut store);
        assert!(store.is_empty());
    }

    #[test]
    fn rejects_ranged_changes_when_full_sync_is_negotiated() {
        let mut store = DocumentStore::default();
        store.open(
            "file:///x.gfa".into(),
            "gfabasic".into(),
            1,
            "PRINT 1".into(),
        );
        let changed = parse_json(br#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":"file:///x.gfa","version":2},"contentChanges":[{"range":{"start":{"line":0,"character":6},"end":{"line":0,"character":7}},"text":"2"}]}}"#).unwrap();
        dispatch(changed, &mut store);
        assert_eq!(store.get("file:///x.gfa").unwrap().version, 1);
        assert_eq!(store.get("file:///x.gfa").unwrap().text, "PRINT 1");
    }

    #[test]
    fn publishes_compiler_diagnostics_and_clears_them_on_close() {
        let mut store = DocumentStore::default();
        let mut outbound = Vec::new();
        let opened = parse_json(br#"{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///bad.gfa","languageId":"gfabasic","version":1,"text":"PRINT \"bad\""}}}"#).unwrap();
        dispatch_with_notifications(opened, &mut store, &mut outbound);
        assert_eq!(outbound.len(), 1);
        assert_eq!(
            outbound[0].string("method"),
            Some("textDocument/publishDiagnostics")
        );
        let diagnostics = outbound[0]
            .get("params")
            .and_then(|params| params.get("diagnostics"))
            .and_then(|value| match value {
                Json::Array(items) => Some(items),
                _ => None,
            })
            .unwrap();
        assert!(!diagnostics.is_empty());
        assert_eq!(
            outbound[0]
                .get("params")
                .and_then(|params| params.integer("version")),
            Some(1)
        );

        outbound.clear();
        let closed = parse_json(br#"{"jsonrpc":"2.0","method":"textDocument/didClose","params":{"textDocument":{"uri":"file:///bad.gfa"}}}"#).unwrap();
        dispatch_with_notifications(closed, &mut store, &mut outbound);
        let cleared = outbound[0]
            .get("params")
            .and_then(|params| params.get("diagnostics"))
            .and_then(|value| match value {
                Json::Array(items) => Some(items),
                _ => None,
            })
            .unwrap();
        assert!(cleared.is_empty());
    }

    #[test]
    fn json_parser_accepts_only_json_whitespace() {
        assert_eq!(parse_json(b" \\t\\r\\nnull ").unwrap(), Json::Null);
        assert!(parse_json(b"\\x0bnull").is_err());
        assert!(parse_json(b"\\x0cnull").is_err());
    }

    #[test]
    fn json_parser_rejects_malformed_and_excessively_nested_messages() {
        assert!(parse_json(b"{").is_err());
        let nested = format!(
            "{}null{}",
            "[".repeat(MAX_JSON_DEPTH + 1),
            "]".repeat(MAX_JSON_DEPTH + 1)
        );
        assert!(parse_json(nested.as_bytes()).is_err());
    }

    #[test]
    fn framing_rejects_messages_over_the_resource_limit() {
        let input = format!("Content-Length: {}\r\n\r\n", MAX_MESSAGE_BYTES + 1);
        let mut reader = io::Cursor::new(input.into_bytes());
        assert_eq!(
            read_message(&mut reader).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn framing_rejects_oversized_headers() {
        let input = format!("X-Padding: {}\r\n\r\n", "x".repeat(MAX_HEADER_BYTES));
        let mut reader = io::Cursor::new(input.into_bytes());
        assert_eq!(
            read_message(&mut reader).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn framing_rejects_duplicate_content_length_headers() {
        let input = b"Content-Length: 1\r\ncontent-length: 1\r\n\r\n{}";
        let mut reader = io::Cursor::new(input.as_slice());
        assert_eq!(
            read_message(&mut reader).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn store_isolates_multiple_documents() {
        let mut store = DocumentStore::default();
        store.open(
            "file:///one.gfa".into(),
            "gfabasic".into(),
            1,
            "PRINT 1".into(),
        );
        store.open(
            "file:///two.gfa".into(),
            "gfabasic".into(),
            1,
            "PRINT 2".into(),
        );
        assert!(store.change("file:///one.gfa", 2, "PRINT 3".into()));
        assert_eq!(store.get("file:///one.gfa").unwrap().text, "PRINT 3");
        assert_eq!(store.get("file:///two.gfa").unwrap().text, "PRINT 2");
    }

    #[test]
    fn malformed_jsonrpc_message_returns_parse_error_and_keeps_server_alive() {
        let input = "Content-Length: 1\r\n\r\n{";
        let mut reader = io::Cursor::new(input.as_bytes());
        let mut output = Vec::new();
        run_stdio(&mut reader, &mut output).unwrap();
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("\"code\":-32700"));
    }

    #[test]
    fn cancellation_notification_is_safe_without_async_requests() {
        let mut store = DocumentStore::default();
        let cancellation =
            parse_json(br#"{"jsonrpc":"2.0","method":"$/cancelRequest","params":{"id":99}}"#)
                .unwrap();
        assert!(dispatch(cancellation, &mut store).is_none());
    }

    #[test]
    fn handles_initialize_and_full_document_notifications() {
        let input = concat!(
            "Content-Length: 58\r\n\r\n",
            "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{}}",
        );
        let mut reader = io::Cursor::new(input.as_bytes());
        let mut output = Vec::new();
        run_stdio(&mut reader, &mut output).unwrap();
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("\"textDocumentSync\":1"));
    }

    #[test]
    fn stdio_recovers_after_malformed_json_and_processes_next_request() {
        fn frame(body: &str) -> String {
            format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
        }
        let malformed = "{";
        let initialize = r#"{"jsonrpc":"2.0","id":7,"method":"initialize","params":{}}"#;
        let exit = r#"{"jsonrpc":"2.0","method":"exit"}"#;
        let input = format!("{}{}{}", frame(malformed), frame(initialize), frame(exit));
        let mut reader = io::Cursor::new(input.into_bytes());
        let mut output = Vec::new();

        run_stdio(&mut reader, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("\"id\":null"));
        assert!(output.contains("\"code\":-32700"));
        assert!(output.contains("\"id\":7"));
        assert!(output.contains("\"textDocumentSync\":1"));
    }

    #[test]
    fn diagnostics_for_multiple_documents_keep_their_uris_and_versions() {
        let mut documents = DocumentStore::default();
        let mut outbound = Vec::new();
        for (uri, version) in [("file:///one.gfa", 3), ("file:///two.gfa", 8)] {
            let message = object_json([
                ("jsonrpc", Json::String("2.0".into())),
                ("method", Json::String("textDocument/didOpen".into())),
                (
                    "params",
                    object_json([(
                        "textDocument",
                        object_json([
                            ("uri", Json::String(uri.into())),
                            ("languageId", Json::String("gfabasic".into())),
                            ("version", Json::Number(version.to_string())),
                            ("text", Json::String("PRINT \"bad\"".into())),
                        ]),
                    )]),
                ),
            ]);
            assert!(dispatch_with_notifications(message, &mut documents, &mut outbound).is_none());
        }

        assert_eq!(outbound.len(), 2);
        let first = outbound[0].get("params").unwrap();
        let second = outbound[1].get("params").unwrap();
        assert_eq!(first.string("uri"), Some("file:///one.gfa"));
        assert_eq!(first.integer("version"), Some(3));
        assert_eq!(second.string("uri"), Some("file:///two.gfa"));
        assert_eq!(second.integer("version"), Some(8));
        assert_eq!(documents.len(), 2);
    }
}
