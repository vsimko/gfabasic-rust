---
type: "Documentation Overview"
title: "Documentation overview"
---

# Documentation overview

The canonical navigation entry point is [the OKF v0.2 documentation index](index.md). This page provides a short audience-oriented overview and remains at its historical path for existing links.

- **New visitors and users:** start with the [project README](../README.md), then use the [user guide index](user/index.md) for current support boundaries and user-facing guidance.
- **Contributors:** read the [contributor guide](contributor/index.md) and [documentation policy](DOCUMENTATION.md) before changing source or docs.
- **Architects and implementers:** use the [architecture index](architecture/index.md), including the numbered [architecture decision records](architecture/adr/index.md).
- **Language and API researchers:** use the [reference index](reference/index.md) and the [reference source register](reference-sources.md). Keep dialect and version evidence separate.
- **Testing and verification:** start with the [testing index](testing/index.md) and [corpus instructions](../tests/corpus/README.md).
- **Source reviewers:** use the [research index](research/index.md) for page-addressable evidence, extraction audits, and open questions.
- **Documentation authors:** use the [template index](templates/index.md) for reusable guide and reference structures.

The Markdown concepts under `docs/` are the canonical editable source. Static website pages are generated from these concepts during CI and deployment; do not edit generated HTML or maintain a second copy of a concept in `site/`.
