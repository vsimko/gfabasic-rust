# Runtime handler registration

The TOML files under `api/registry/` are the canonical source of API declaration IDs, implementation status, signatures, and `implementation.handler` mappings. The runtime crate consumes a generated Rust requirement table; regenerate it with:

```sh
python3 scripts/generate_runtime_handler_requirements.py
python3 scripts/generate_runtime_handler_requirements.py --check
```

Language-core IDs use `gfa.language.<category>.<name>`; Amiga IDs use
`gfa.amiga.<category>.<name>`; Atari ST IDs use `gfa.atari-st.<category>.<name>`.
IDs are case-sensitive. Handler IDs are explicit registry values and are not
Rust symbol names.

Each runtime registration carries an API profile and a typed Rust function
signature. Resolution requires exactly one registration with the expected
profile and signature. Missing, duplicate, stale, wrong-namespace, wrong-profile,
and wrong-signature mappings are errors; there is no generic fallback handler.

CI checks that every declaration marked `implemented` or `partial` has a
supported typed signature and exactly one registered runtime handler. It also
rejects handler IDs on non-callable declarations and registrations that are not
referenced by the canonical registry. The runtime crate tests the complete
generated declaration set, including unsupported declarations, without SDL3.

The compiler's semantic API resolver uses the same runtime lookup by stable
declaration ID. The narrow combinatorics IR lowering now dispatches FACT,
VARIAT, and COMBIN through those typed registrations rather than maintaining
duplicate arithmetic implementations.

The three combinatorics declarations are marked `partial`, not complete:
the helpers use u32 inputs and u128 results, while the original interpreter's
overflow/error behavior remains unverified and the compiler frontend remains
restricted to its documented literal subset. Their Linux support remains
`unknown` and conformance remains `not-tested`. A Rust registration does not
promote evidence, target support, or historical conformance.
