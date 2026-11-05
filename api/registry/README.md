# API registry

Canonical declarations use TOML. The planned registry namespaces are `language-core/`, `amiga/`, and `atari-st/`; this describes the intended layout, not the current set of populated directories.

The current checked-in registry contains only `atari-st/time.toml`: one research-status Atari ST `TIMER` declaration. The `language-core/` and `amiga/` namespaces do not yet contain declarations. Do not create placeholder entries or infer implementation support from the planned layout.

Registry `support.tests` references use each manifest-listed fixture sidecar's stable `id`; manifest path stems are storage locations, not test IDs. Do not mark a declaration supported without primary evidence, a runtime implementation, and passing conformance tests.
