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

The first private-media inventory completed successfully on 2026-11-13. A follow-up extracted the candidate executables in the private workflow workspace, verified their AmigaOS loadseg headers, and recorded SHA-256 hashes. Only text metadata is retained as a workflow artifact; the extracted executable bytes are not published.

| Private asset | Filesystem and executable evidence | Intended research role |
|---|---|---|
| `GFA-BASIC v3.5U (1990)...(Disk 1 of 2)[b corrupt files][WB].zip` | ADF volume `GFA-DISK1`; contains `GFABasic` (135,956 bytes), `GFABasRo` (99,872 bytes), `GFA_BCOM` (89,480 bytes), and a `libs` directory. Readable filesystem, but the boot-block checksum is invalid. The archive name warns of corrupt files and this is only disk 1 of 2. The filesystem is readable, although its boot-block checksum is invalid. The extracted interpreter is a valid AmigaOS loadseg executable and embeds the exact string “GFA-BASIC Version 3.5 U”; the compiler embeds “GFA-BASIC 3.5E Compiler”. | Strong 3.5-era media candidate, but the interpreter's exact version label is unresolved. Keep it as a separate 3.5U/3.0U candidate profile pending runtime About/version identification. |
| `GFA-BASIC v3.52 (1990)...(Disk 1 of 2)[WB].zip` | ADF volume `GFABASIC3.52`; contains `GFABasic` (136,548 bytes) and `GFABasRo` (100,464 bytes). The `GFABasic` is the same 136,548-byte binary as the existing private `GFABasic` (SHA-256 `fd80f6093e8d0163856af5849121e945e37051a2ad2a9053aa1930021783d962`) and embeds “GFA-BASIC Version 3.52 D”. | Existing 3.52 interpreter baseline and strong later-version comparison profile for the 3.5 language target. |
| `GFA-BASIC v3.52 (1990)...(Disk 2 of 2).zip` | ADF volume `GFA_BCOM3.52`; contains `GFA_BCOM` (89,608 bytes; SHA-256 `ec31cf8b91e945f4eea90cf332b4fef4fe3246f59d7673733ec26a4d7fe49077`), libraries and linker support. The executable embeds “GFA-BASIC 3.52D Compiler”. | Separate compiler profile; do not conflate it with the interpreter version. |
| `GFA-BASIC v3.041 (1989-08-31)(GFA Systemtechnik).zip` | ADF volume `GFABASIC`; contains `gfabasic` (125,092 bytes; SHA-256 `f5185db2dfc99af11a15570120f0c40df9f8ad201c28b43cc0894bfaa9562759`) and `gfabasro` (91,060 bytes; SHA-256 `11be94fd0c1c5b7306afccaae2bd716716bb58b698f28efc7f5929dcb3ffa353`). The extracted interpreter is a valid AmigaOS loadseg executable and embeds “GFA-BASIC Version 3.041 U”. Its boot block is valid. | Identified Amiga GFA BASIC 3.041 interpreter candidate and practical 3.0-era baseline. |

## Version interpretation

The maintainer's pragmatic working model is supported in part by the disk contents:

- The extracted 3.5U interpreter embeds “GFA-BASIC Version 3.5 U”; this confirms the exact interpreter version identity from the disk image, not merely its archive filename.
- The existing 3.52 interpreter embeds “GFA-BASIC Version 3.52 D” and matches the binary already used by the project byte-for-byte. It remains a useful later-version comparison and pragmatic runtime target for the 3.5-era language specification.
- The v3.041 interpreter embeds “GFA-BASIC Version 3.041 U”, establishing a distinct 3.0-era baseline.
- The separate 3.52 disk 2 is the compiler distribution, with 3.52/3.52D strings. Interpreter and compiler version labels are not necessarily identical.
- The v3.041 image is an AmigaDOS disk with interpreter and run-only interpreter files; its README strings refer to version 3.041. It is a useful distinct 3.0-era reference.
- The 3.5U disk 1 contains the interpreter, run-only interpreter, and compiler; the extracted interpreter's own embedded version string confirms 3.5 U. The disk image is not bootable as supplied because its boot-block checksum is invalid, but its filesystem and executable files are readable.

Recommended test profiles:

1. **3.52 interpreter** — primary practical runtime profile for the 3.5-era language target; continue the existing workflow and label every result 3.52.
2. **3.5U interpreter** — direct 3.5-era profile; execute the extracted binary and obtain/inspect disk 2 before claiming complete distribution coverage.
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

- The 3.5U, 3.041, and 3.52 ADF filesystems have been enumerated. Interpreter/compiler candidates were extracted to the ephemeral workflow workspace and fingerprinted; all identified executables have the AmigaOS loadseg header `00 00 03 f3`.
- The workflow records byte size, SHA-256, file type, header bytes, and version strings. Extracted binaries are kept under the runner's temporary directory and are not included in the uploaded artifact.
- Boot the 3.5U candidate under the same Amiberry/Kickstart profile as the existing 3.52 baseline where possible. Its invalid boot-block checksum may require copying the extracted interpreter onto a fresh working boot volume.
- Run the same small probe corpus against 3.5U, 3.52, and 3.041 and retain separate result directories.
- Obtain and inspect disk 2 of the 3.5U set before making claims about a complete distribution.

## Evidence policy

- Archive filename and disk catalogue metadata are **candidate identity evidence**, not executable identity.
- A valid Amiga loadseg header and filesystem location help identify a binary, but do not by themselves prove its product version or whether it is interpreter/compiler/editor.
- A version banner/About screen plus matching disk provenance provides stronger component identity.
- Runtime output is observed evidence only for the exact executable hash, emulator, Kickstart, and disk setup recorded in that run.
- 3.52 results can pragmatically guide implementation, but remain labelled 3.52. 3.5U runs should become the direct 3.5 evidence stream.
