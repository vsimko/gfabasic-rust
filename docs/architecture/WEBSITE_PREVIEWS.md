---
type: "Architecture"
title: "Website previews for pull requests"
---

# Website previews for pull requests

The Pages site uses the `gh-pages` branch as its publishing source. The source of documentation pages is the canonical OKF v0.2 bundle under `docs/`; CI validates it and generates safe, static HTML into `site/docs/` before publishing. Draft concepts are excluded and deprecated concepts carry a visible warning. Generated HTML is build output and must not be edited or committed as a second documentation source. The production workflow updates the site at the branch root while preserving `pr-preview/`. The PR preview workflow publishes each same-repository pull request below:

`https://vsimko.github.io/gfabasic-rust/pr-preview/pr-<number>/`

A sticky PR comment contains the preview URL and is refreshed on new commits. When a pull request is closed (including after merge), the preview action removes its directory from `gh-pages`. The workflow does not wait for a Pages build because GitHub Pages does not start a branch deployment while its source is set to GitHub Actions.

## One-time repository setup

After merging this change, configure **Settings → Pages → Build and deployment → Source** to **Deploy from a branch**, then select branch `gh-pages` and folder `/(root)`.

Also ensure **Settings → Actions → General → Workflow permissions** allows read and write access, so the workflows can update `gh-pages` and post preview comments.

Previews from forks are intentionally skipped because pull-request code from forks must not receive write permissions. Production publishing remains limited to pushes to `main`. The Pages workflow explicitly watches `docs/**` and the shared documentation generator/validator, so documentation-only changes trigger production deployment. After publishing, a production smoke test verifies the documentation index, a representative ADR, and its stylesheet. PR previews run the same source validation and page generation before deployment, then smoke-test both the main dashboard and generated documentation.
