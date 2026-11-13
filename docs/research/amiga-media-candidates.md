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

| Private asset | What the repository currently establishes | Intended research role |
|---|---|---|
| `GFA-BASIC v3.5U (1990)...(Disk 1 of 2)[b corrupt files][WB].zip` | Archive label says 3.5U, 1990, German, disk 1 of 2; archive filename warns of corrupt files. Executable identity has not yet been inspected. | Primary candidate for the 3.5U interpreter. Incomplete until disk 2 is available or the interpreter is found on disk 1. |
| `GFA-BASIC v3.52 (1990)...(Disk 1 of 2)[WB].zip` and disk 2 ZIP | Two disk archives are present. | Media-level comparison candidate for the existing 3.52 interpreter. |
| `GFABasic` | Existing AmigaOS loadseg executable, 136,548 bytes, SHA-256 `fd80f6093e8d0163856af5849121e945e37051a2ad2a9053aa1930021783d962`, as recorded in the private repository README. This is the executable already used by the Amiberry workflow. | Existing 3.52 runtime baseline and comparison target. |
| `GFA-BASIC v3.041 (1989-08-31)(GFA Systemtechnik).zip` | Archive filename identifies a 3.041-labelled release dated 1989-08-31. Platform and executable component/version have not yet been inspected. | Possible 3.0-era reference. Do not call it an Amiga 3.0.41 interpreter until its disk contents establish that. |

## Version interpretation

The project maintainer's working hypothesis is:

- GFA BASIC 3.5U (1990) is a direct interpreter candidate for the Amiga 3.5 language target.
- GFA BASIC 3.52 (1990), already used by the project, implements the 3.5-era specification and is a useful later-version comparison.
- GFA BASIC 3.041 (1989) is a possible distinct 3.0-era baseline.

This is a useful pragmatic test plan, but the archive names alone do not prove these mappings. In particular, keep these labels distinct in test results:

1. **3.5U candidate** — pending extraction and executable identity.
2. **3.52 observed baseline** — already executed; observations remain 3.52.
3. **3.041 archive candidate** — platform and component identity pending.

Do not silently relabel a 3.52 observation as a 3.5U observation. If 3.52 behavior agrees with the 3.5 manuals, record that as supporting compatibility evidence, not proof that all 3.5 and 3.52 behavior is identical.

## Inventory and extraction plan

The metadata-only GitHub Actions workflow:

1. Downloads the four ZIP archives from the private assets repository using `GFA_ASSETS_TOKEN`.
2. Records SHA-256 hashes of the original ZIPs.
3. Lists archive members and extracted files.
4. Identifies ADF files and records filesystem metadata, root directory entries, boot block information, and likely embedded version strings.
5. Publishes only text inventory reports as a short-lived Actions artifact; it does not publish the copyrighted disk images or extracted executable.

After the inventory:

- Identify whether the 3.5U disk image contains the interpreter executable, editor, compiler, or only supporting files.
- Extract candidate loadseg files into the private workflow workspace, never into the public repository.
- Record executable byte size, SHA-256, Amiga Hunk/loadseg header, and version strings.
- Boot the 3.5U candidate under the same Amiberry/Kickstart profile as the existing 3.52 baseline where possible.
- Run the same small probe corpus against both versions and retain separate result directories.
- Inspect the v3.041 disk contents before deciding whether it belongs in the Amiga interpreter comparison matrix.

## Evidence policy

- Archive filename and disk catalogue metadata are **candidate identity evidence**, not executable identity.
- A valid Amiga loadseg header and filesystem location help identify a binary, but do not by themselves prove its product version or whether it is interpreter/compiler/editor.
- A version banner/About screen plus matching disk provenance provides stronger component identity.
- Runtime output is observed evidence only for the exact executable hash, emulator, Kickstart, and disk setup recorded in that run.
- 3.52 results can pragmatically guide implementation, but remain labelled 3.52. 3.5U runs should become the direct 3.5 evidence stream.
