---
type: "Research Note"
title: "Amiga GFA BASIC media candidates"
---

# Amiga GFA BASIC media candidates

- Tracking: [issue #66](https://github.com/vsimko/gfabasic-rust/issues/66).
- Private source repository: `vsimko/gfabasic-private-assets`.
- Inventory workflow: [Amiga media inventory](../../.github/workflows/amiga-media-inventory.yml).
- Last reviewed: 2026-11-13.

## Candidate register

The first private-media inventory completed successfully on 2026-11-13. It read the ADF filesystems and identified the following contents:

| Private asset | Filesystem and executable evidence | Intended research role |
|---|---|---|
| `GFA-BASIC v3.5U (1990)...(Disk 1 of 2)[b corrupt files][WB].zip` | ADF volume `GFA-DISK1`; contains `GFABasic` (135,956 bytes), `GFABasRo` (99,872 bytes), `GFA_BCOM` (89,480 bytes), and a `libs` directory. Readable filesystem, but the boot-block checksum is invalid. The archive name warns of corrupt files and this is only disk 1 of 2. Identity strings are mixed: README text calls it “3.5 with Compiler” and “3.01/3.5-Compiler and Interpreter”, while the interpreter itself contains the string “GFA-BASIC Version 3.0 U”. | Strong 3.5-era media candidate, but the interpreter's exact version label is unresolved. Keep it as a separate 3.5U/3.0U candidate profile pending runtime About/version identification. |
| `GFA-BASIC v3.52 (1990)...(Disk 1 of 2)[WB].zip` | ADF volume `GFABASIC3.52`; contains `GFABasic` (136,548 bytes) and `GFABasRo` (100,464 bytes). The `GFABasic` is the same size as the existing private `GFABasic` binary. Embedded strings include “GFA-BASIC Interpreter Version 3.5”. | Existing 3.52 interpreter baseline and strong later-version comparison profile for the 3.5 language target. |
| `GFA-BASIC v3.52 (1990)...(Disk 2 of 2).zip` | ADF volume `GFA_BCOM3.52`; contains `GFA_BCOM` (89,608 bytes), libraries and linker support. Embedded strings include “GFA-BASIC 3.52 Compiler” and “GFA-BASIC 3.52D Linker/Compiler”; a README string refers to compiler version 3.51. | Separate compiler profile; do not conflate it with the interpreter version. |
| `GFA-BASIC v3.041 (1989-08-31)(GFA Systemtechnik).zip` | ADF volume `GFABASIC`; contains `gfabasic` (125,092 bytes) and `gfabasro` (91,060 bytes). README strings explicitly refer to “version 3.041” and Amiga interpreter features. Its boot block is valid. | Identified Amiga GFA BASIC 3.041 interpreter candidate and practical 3.0-era baseline. |

## Version interpretation

The maintainer's pragmatic working model is supported in part by the disk contents:

- GFA BASIC 3.52's interpreter binary contains the explicit string “GFA-BASIC Interpreter Version 3.5”, while its disk volume and media are labelled 3.52. This is strong evidence that the existing 3.52 runtime is a suitable practical implementation target for the 3.5-era language specification.
- The separate 3.52 disk 2 is the compiler distribution, with 3.52/3.52D strings. Interpreter and compiler version labels are not necessarily identical.
- The v3.041 image is an AmigaDOS disk with interpreter and run-only interpreter files; its README strings refer to version 3.041. It is a useful distinct 3.0-era reference.
- The 3.5U disk 1 contains interpreter/compiler files and documentation referring to 3.5, but its interpreter's embedded string says “3.0 U”. Until the About/version screen and disk 2 are checked, do not silently resolve this conflict in favour of either “3.5U” or “3.0U”.

Recommended test profiles:

1. **3.52 interpreter** — primary practical runtime profile for the 3.5-era language target; continue the existing workflow and label every result 3.52.
2. **3.5U / 3.0U disk set** — independent candidate profile; verify its runtime version string and obtain/inspect disk 2 before claiming exact 3.5U conformance.
3. **3.041 interpreter** — 3.0-era comparison profile.

Do not silently relabel a 3.52 observation as a 3.5U observation. Agreement between 3.52 and the 3.5 manuals is supporting compatibility evidence, not proof that every 3.5 and 3.52 behaviour is identical.

## Inventory and extraction plan

The metadata-only GitHub Actions workflow:

1. Downloads the four ZIP archives from the private assets repository using `GFA_ASSETS_TOKEN`.
2. Records SHA-256 hashes of the original ZIPs.
3. Lists archive members and extracted files.
4. Identifies ADF files and records filesystem metadata, root directory entries, boot block information, and likely embedded version strings.
5. Publishes only text inventory reports as a short-lived Actions artifact; it does not publish the copyrighted disk images or extracted executable.

After the initial inventory:

- The 3.5U, 3.041, and 3.52 ADF filesystems have been enumerated. Interpreter/compiler candidates are identified by path and byte size.
- The follow-up inventory workflow extracts the candidate loadseg files only into the private workflow workspace and records byte size, SHA-256, file type, header bytes, and version strings. It publishes hashes and text metadata only, never the binaries.
- Boot the 3.5U candidate under the same Amiberry/Kickstart profile as the existing 3.52 baseline where possible. Its invalid boot-block checksum may require copying the extracted interpreter onto a fresh working boot volume.
- Run the same small probe corpus against 3.5U, 3.52, and 3.041 and retain separate result directories.
- Obtain and inspect disk 2 of the 3.5U set before making claims about a complete distribution.

## Evidence policy

- Archive filename and disk catalogue metadata are **candidate identity evidence**, not executable identity.
- A valid Amiga loadseg header and filesystem location help identify a binary, but do not by themselves prove its product version or whether it is interpreter/compiler/editor.
- A version banner/About screen plus matching disk provenance provides stronger component identity.
- Runtime output is observed evidence only for the exact executable hash, emulator, Kickstart, and disk setup recorded in that run.
- 3.52 results can pragmatically guide implementation, but remain labelled 3.52. 3.5U runs should become the direct 3.5 evidence stream.
