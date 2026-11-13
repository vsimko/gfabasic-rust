---
type: "Research Note"
title: "Amiga emulator experiment lab"
---

# Amiga emulator experiment lab

- Tracking: [issue #66](https://github.com/vsimko/gfabasic-rust/issues/66).
- Status: **3.5U and 3.52 media candidates are now available in the private assets repository; automated disk-image inventory and executable identification are pending**.
- Purpose: define a repeatable and evidence-safe way to compare Amiga GFA BASIC interpreter/compiler behavior with the applicable manuals.
- Non-goal: this document does not establish that an archive-labelled release is a particular interpreter or compiler version.

## 1. Emulator baseline

Use [FS-UAE](https://fs-uae.net/) as the initial Linux-hosted emulator because it provides Linux x86-64 builds, a separate Launcher for configuration, and support for Amiga floppy-image workflows.

The official site currently lists FS-UAE 3.2.35 Linux x86-64 and ARM64 downloads. Its release notes date 3.2.35 to 2025-09-07 and describe it as the stable release at that time. Pin the exact emulator and Launcher versions in every experiment record; do not write only “FS-UAE latest”.

For a Linux x86-64 host:

1. Obtain FS-UAE and FS-UAE Launcher from the [official Linux download page](https://fs-uae.net/download/linux/), or use the official package repository for the host distribution.
2. If using the portable archives, extract both directories and keep them alongside each other as the FS-UAE instructions require.
3. Start FS-UAE Launcher and create a named configuration for the specific experiment.
4. Record the exact emulator and Launcher versions shown by the installed builds.

The Launcher is preferred for the initial setup because it exposes machine, Kickstart, and floppy configuration without requiring contributors to invent configuration keys. Preserve the resulting configuration file privately with the experiment record.

## 2. ROM and software provenance

### Kickstart ROM

Use only a Kickstart ROM that is licensed for the intended emulator use. FS-UAE documents Amiga Forever Plus/Premium as a way to obtain licensed ROMs for supported Amiga models. The FS-UAE documentation also says that its built-in AROS replacement is less compatible with Amiga software. Do not use the replacement ROM for a conformance run unless the experiment is explicitly labelled as an AROS compatibility probe.

The Amiga Forever licensing FAQ describes the EULA scope for ROM and operating-system files and separately warns that third-party files may have different licensing conditions. A Kickstart/AmigaOS licence does **not** grant rights to copy or redistribute GFA BASIC. Check the applicable terms for the GFA BASIC interpreter/compiler and every sample program independently.

References:

- [FS-UAE Kickstart ROM guidance](https://fs-uae.net/docs/kickstart-roms/)
- [Amiga Forever: use of ROM and operating-system files](https://www.amigaforever.com/kb/13-145)

### GFA BASIC disks and samples

- Use a copy obtained from a rights holder or another source for which the right to make and use the working copy is clear.
- Do not download or use an archive image merely because its catalogue entry is public.
- Do not commit ADF/IPF images, Kickstart ROMs, Workbench disks, or copyrighted sample collections to this repository.
- Keep the original media/image read-only. Run experiments on a separate working copy if the emulator or interpreter can write to a disk.
- Record the source, acquisition date, disk label, file name, image format, and a local SHA-256 digest.
- Keep disk 1 and disk 2 as separate inputs. Do not infer a component version from an archive filename alone.

The current research register lists archive-catalogue entries labelled “GFA-BASIC v3.52” but also records contemporary evidence that interpreter 3.51 and compiler 3.52 were marketed separately. Therefore, record the interpreter and compiler identities independently. The archive label is not proof that the interpreter executable is 3.52.

## 3. Machine configuration

Do not hard-code an Amiga model as the project-wide baseline yet. Select the model, chipset, CPU, Kickstart, and memory configuration from the documentation or verified requirements of the exact software component under test.

For each configuration, record:

- Amiga model selected in FS-UAE;
- CPU model/speed and chipset options, if exposed;
- Kickstart version/revision and local ROM SHA-256;
- Workbench/AmigaOS version, if used;
- floppy drive assignments and disk labels;
- display mode, input/keyboard mapping, and locale settings;
- FS-UAE and Launcher versions;
- the exported configuration file, with private absolute paths removed before any publication.

If the software requirements are not known, mark the configuration as exploratory. Do not combine observations from different machine profiles into one result.

## 4. Experiment procedure

1. Start from a cold emulator boot for each independent probe. Do not rely on an undocumented saved state.
2. Insert the legally obtained GFA BASIC disks in the documented order.
3. Record the product identity visible in the application (for example, an About/version screen) and identify interpreter and compiler components separately. Do not assume a version command exists.
4. Record any startup error, missing dependency, disk request, or hardware requirement before running a language probe.
5. Run one small, independently authored probe that tests one behavior.
6. Record the exact steps, output, error messages, side effects, and any required user interaction.
7. Repeat the probe from a fresh start when repeatability matters. Record inconsistent results rather than selecting the preferred output.
8. Compare the observation with the cited manual passage. Record agreement, discrepancy, or unresolved status; do not silently replace a manual claim with one observation.

Do not use a compiler result as evidence for interpreter behavior, or vice versa. Do not transfer an observation from Atari ST to Amiga or from one Amiga version to another.

## 5. Observation record

Create one record per probe using this minimum template:

```text
probe_id:
date_utc:
host_os_and_version:
emulator_version:
launcher_version:
amiga_model:
cpu_and_chipset:
kickstart_version_and_sha256:
workbench_version:
gfa_component: interpreter | compiler | editor | unknown
gfa_version_string:
disk_labels_and_order:
disk_source_and_permissions:
disk_image_sha256:
manual_source_and_printed_pages:
probe_source_path_and_sha256:
setup_steps:
expected_result_from_manual:
observed_result:
errors_and_side_effects:
repeatability:
evidence_location:
status: candidate | observed | disputed | blocked
limitations:
```

Use the following distinctions:

- **Manual-backed**: the applicable manual passage supports the expected behavior.
- **Observed**: the behavior was reproduced on the exact recorded interpreter/compiler build and emulator configuration.
- **Inferred**: a conclusion follows from evidence but is not stated or directly observed.
- **Disputed**: manual and implementation evidence disagree.
- **Blocked**: the required licensed software, exact component identity, or reproducible environment is unavailable.

An “observed” result is not automatically a normative language rule. Keep the source citation and the experiment record side by side.

## 6. Initial status and blockers

Historical initial status (2026-11-03): no GFA BASIC disk image had been obtained or executed at that point. This status was superseded by the automated workflow integration on `main` (2026-11-09).

Current status (2026-11-13):

- The native Amiberry Merge + Run workflow is integrated on `main` through PR #382.
- The private assets repository now contains the previously used GFA BASIC 3.52 loadseg executable (136,548 bytes, SHA-256 recorded in that repository's README), two ZIP archives for 3.52 disks 1 and 2, a ZIP labelled GFA BASIC 3.5U (1990), disk 1 of 2, and a ZIP labelled GFA BASIC v3.041 (1989-08-31).
- The 3.5U archive filename itself marks it as having corrupt files; only disk 1 of 2 is currently present. The v3.041 archive name does not, by itself, establish its target platform or prove that its executable is an Amiga GFA BASIC 3.0.41 interpreter.
- The 3.52 executable has already been exercised by the native Amiberry Merge + Run probe: `FACT(0..2)` produced `1 1 2`. These observations remain explicitly versioned as 3.52.
- A new metadata-only workflow, `.github/workflows/amiga-media-inventory.yml`, downloads the four ZIP archives from private storage, records their SHA-256 hashes, enumerates archive contents, and inspects any contained ADF filesystems. It does not publish disk images or extracted commercial binaries.
- Until that inventory identifies the executable files and their component/version banners, treat 3.5U as the primary 3.5 interpreter candidate, 3.52 as the existing later-version comparison candidate, and v3.041 as a possible 3.0-era candidate. These are working roles, not yet verified identities.
- Once the 3.5U executable is identified and can be run, add a separate 3.5U probe profile. Keep 3.52 as a comparison profile and use v3.041 only if the disk contents establish that it is the relevant Amiga interpreter.

See [the Amiga media candidate register](amiga-media-candidates.md) and [issue #66](https://github.com/vsimko/gfabasic-rust/issues/66).
