# API registry

Canonical declarations are stored in TOML files under `language-core/`, `amiga/`, and `atari-st/`. The initial registry contains only research-status entries. Registry `support.tests` references use each manifest-listed fixture sidecar's stable `id`; manifest path stems are storage locations, not test IDs. Do not mark a declaration supported without primary evidence, a runtime implementation, and passing conformance tests.
