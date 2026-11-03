# Runtime architecture

- Status: Proposed baseline; contracts remain open under issue #7.

## Layers

1. **GFA BASIC core runtime**: language services that need runtime support, such as strings, arrays, file operations, numeric helpers, and error/status support.
2. **API compatibility runtime**: profile-specific Amiga and Atari ST behavior.
3. **Platform integration**: operating-system services for the selected target.
4. **Backend adapters**: SDL3 may provide Linux graphics, window, and input integration behind project-owned interfaces.

## Contract

Generated code must call project-owned runtime interfaces. It must not call SDL3 APIs directly. Runtime interfaces must define calling convention, data layout, ownership, error propagation, initialization, shutdown, and versioning.

## Capability reporting

Every runtime operation must have a support status for the selected target/profile. Distinguish native implementation, software emulation, partial support, and unsupported behavior. A successful link is not proof of semantic compatibility.

## Graphics and testing

Specify screen/window state, palettes, drawing state, planar/chunky image representations, pixel conversion, bitmap operations, refresh timing, and input mapping. Keep core runtime tests independent of SDL3. Platform tests should support headless CI where possible.

Tracking: [issue #7](https://github.com/vsimko/gfabasic-rust/issues/7).
