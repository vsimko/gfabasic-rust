use gfabasic_compiler::source::SourceMap;

const MAX_INPUT_BYTES: usize = 1024 * 1024;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        if data.len() > MAX_INPUT_BYTES {
            return;
        }

        let mut sources = SourceMap::new();
        match sources.add_utf8_bytes("fuzz.gfa", data.to_vec()) {
            Ok(source_id) => {
                let file = sources.get(source_id).expect("added source is retrievable");
                assert_eq!(file.text().as_bytes(), data);
                assert!(file.line_count() >= 1);
                assert!(file.line_text(1).is_some());
                assert!(file.line_text(file.line_count()).is_some());

                for offset in [0, file.text().len(), file.text().len() / 2] {
                    if file.text().is_char_boundary(offset) {
                        assert!(file.line_column(offset).is_some());
                    }
                }
            }
            Err(_) => {
                assert!(
                    sources.is_empty(),
                    "a failed UTF-8 decode must not mutate the source map"
                );
            }
        }
    });
}
