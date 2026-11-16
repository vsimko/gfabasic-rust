# GFA BASIC language server

The `gfabasic-lsp` binary is a dependency-free LSP transport adapter around the shared `gfabasic-compiler` frontend.

## Run

```sh
cargo run -p gfabasic-lsp
```

The process reads and writes standard LSP JSON-RPC messages on stdin/stdout. Do not write logs to stdout; diagnostics and operational errors belong on stderr.

## Implemented capabilities

- JSON-RPC 2.0 framing with `Content-Length` headers.
- `initialize`, `initialized`, `shutdown`, and `exit`.
- Full-document synchronization: `didOpen`, `didChange`, and `didClose`.
- Monotonically increasing document-version checks; stale updates are ignored.
- UTF-16 LSP positions converted to UTF-8 byte offsets used by compiler spans.
- Push diagnostics from the shared compiler `parse_document` entry point, tagged with the document version.
- Diagnostics are cleared when a document closes.
- No protocol, editor SDK, LLVM, SDL3, or runtime dependency.

## Explicit limitations

- Only full-document changes are accepted. Ranged changes are ignored.
- Diagnostics cover only syntax accepted/rejected by the current compiler subset; they are not a claim of full Amiga GFA BASIC 3.5 validation.
- Formatting, completion, definition, references, rename, code actions, and build requests are not implemented or advertised.
- The internal JSON parser is a bootstrap implementation. Adversarial testing and fuzzing are still required before production use.
- Cancellation and concurrent request processing are not implemented.

See [ADR 0012](../../docs/architecture/adr/0012-language-server-and-editor-integration.md) and [issue #209](https://github.com/vsimko/gfabasic-rust/issues/209) for architecture and sequencing.
