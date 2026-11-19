use gfabasic_compiler::source::SourceMap;

fn expected_line_column(text: &str, offset: usize) -> Option<(usize, usize)> {
    if offset > text.len() || !text.is_char_boundary(offset) {
        return None;
    }

    let bytes = text.as_bytes();
    let mut index = 0;
    let mut line = 1;
    let mut column = 1;
    while index < offset {
        match bytes[index] {
            b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                if offset < index + 2 {
                    break;
                }
                index += 2;
                line += 1;
                column = 1;
            }
            b'\r' | b'\n' => {
                index += 1;
                line += 1;
                column = 1;
            }
            _ => {
                let character = text[index..].chars().next().unwrap();
                index += character.len_utf8();
                column += 1;
            }
        }
    }
    Some((line, column))
}

fn expected_lines(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut lines = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\r' => {
                lines.push(&text[start..index]);
                index += 1;
                if bytes.get(index) == Some(&b'\n') {
                    index += 1;
                }
                start = index;
            }
            b'\n' => {
                lines.push(&text[start..index]);
                index += 1;
                start = index;
            }
            _ => index += 1,
        }
    }
    lines.push(&text[start..]);
    lines
}

#[test]
fn generated_source_text_has_consistent_lines_and_columns() {
    // Fixed xorshift seed and bounded alphabet keep this property test reproducible.
    const SEED: u64 = 0x5EED_35A1_2026;
    const ALPHABET: [&str; 7] = ["A", "z", "é", "界", "🙂", "\r", "\n"];

    let mut state = SEED;
    for case_index in 0..128 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let character_count = (state as usize) % 49;

        let mut text = String::new();
        for _ in 0..character_count {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            text.push_str(ALPHABET[(state as usize) % ALPHABET.len()]);
        }

        let mut map = SourceMap::new();
        let source = map.add_source(format!("generated-{case_index}.gfa"), text.clone());
        let file = map.get(source).unwrap();
        let lines = expected_lines(&text);

        assert_eq!(
            file.line_count(),
            lines.len(),
            "seed {SEED:#x}, case {case_index}, text {text:?}"
        );
        for (index, expected) in lines.iter().enumerate() {
            assert_eq!(
                file.line_text(index + 1),
                Some(*expected),
                "seed {SEED:#x}, case {case_index}, line {}, text {text:?}",
                index + 1
            );
        }

        for offset in 0..=text.len() {
            assert_eq!(
                file.line_column(offset),
                expected_line_column(&text, offset),
                "seed {SEED:#x}, case {case_index}, byte offset {offset}, text {text:?}"
            );
        }
    }
}
