# Memory model and hardware access

- Status: Open. No GFA BASIC memory semantics have been accepted.

Research variable storage, lifetimes, arrays, strings, aggregates, address values, pointer/integer conversions, raw memory operations, alignment, aliasing, and original hardware layouts.

The compiler must not assume an original platform address has the same meaning in a Linux process. Classify hardware/OS-specific operations as equivalent implementation, software emulation, an explicit unsafe/privileged extension, or unsupported. Never silently reinterpret a physical address as a valid host pointer.

Required decisions include address representation and pointer-width conversions, lifetime/ownership, documented semantics of PEEK/POKE-like operations, bounds/alignment, runtime checks, and target-specific unsafe operation policy.

No candidate command names or semantics are normative until tied to an applicable manual.

Tracking: [issue #8](https://github.com/vsimko/gfabasic-rust/issues/8).
