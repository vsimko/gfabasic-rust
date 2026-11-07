const menuButton = document.querySelector(".menu-toggle");
const navigation = document.querySelector("#main-nav");
menuButton?.addEventListener("click", () => {
  const open = menuButton.getAttribute("aria-expanded") !== "true";
  menuButton.setAttribute("aria-expanded", String(open));
  navigation?.classList.toggle("open", open);
});
navigation?.querySelectorAll("a").forEach((link) => {
  link.addEventListener("click", () => {
    navigation.classList.remove("open");
    menuButton?.setAttribute("aria-expanded", "false");
  });
});
const toast = document.querySelector(".copy-toast");
document.querySelector(".copy-button")?.addEventListener("click", async (event) => {
  const button = event.currentTarget;
  const text = button.getAttribute("data-copy") || "";
  try {
    await navigator.clipboard.writeText(text);
    toast.textContent = "Example copied";
    toast.classList.add("show");
    window.setTimeout(() => toast.classList.remove("show"), 1800);
  } catch {
    toast.textContent = "Clipboard access unavailable";
    toast.classList.add("show");
    window.setTimeout(() => toast.classList.remove("show"), 1800);
  }
});

const generatedStatus = document.querySelector("#generated-project-status");
if (generatedStatus) {
  const textNode = (tag, className, value) => {
    const node = document.createElement(tag);
    if (className) node.className = className;
    node.textContent = value;
    return node;
  };
  const sourceLink = (href, label) => {
    const link = document.createElement("a");
    link.href = href;
    link.target = "_blank";
    link.rel = "noreferrer";
    link.textContent = label;
    return link;
  };
  const statusLabel = (status) => ({
    implemented: "Implemented",
    partial: "Partial",
    prototype: "Prototype",
    research: "Research",
    planned: "Planned",
    blocked: "Blocked",
  }[status] || "Unknown");

  fetch("data/project.json")
    .then((response) => {
      if (!response.ok) throw new Error("Generated project data is unavailable");
      return response.json();
    })
    .then((data) => {
      if (data.schema_version !== 2 || !data.project || !Array.isArray(data.components) || !Array.isArray(data.workspace_packages)) {
        throw new Error("Generated project data has an unsupported or incomplete schema");
      }

      const overview = document.createElement("div");
      overview.className = "dashboard-overview";
      const maturity = document.createElement("article");
      maturity.className = "dashboard-stat";
      maturity.append(textNode("span", "dashboard-stat-label", "Project maturity"), textNode("strong", "", data.project.maturity || "Unknown"), textNode("small", "", "Experimental project"));
      const target = document.createElement("article");
      target.className = "dashboard-stat";
      target.append(textNode("span", "dashboard-stat-label", "Product target"), textNode("strong", "", data.project.target || "Unknown"), textNode("small", "", data.project.source_dialect || "Dialect unknown"));
      const packages = document.createElement("article");
      packages.className = "dashboard-stat";
      packages.append(textNode("span", "dashboard-stat-label", "Workspace"), textNode("strong", "", String(data.workspace_packages.length)), textNode("small", "", "Rust crates in Cargo workspace"));
      overview.append(maturity, target, packages);

      const heading = textNode("h3", "dashboard-subheading", "Implementation states");
      const grid = document.createElement("div");
      grid.className = "dashboard-components";
      for (const component of data.components) {
        const card = document.createElement("article");
        card.className = "dashboard-component";
        const top = document.createElement("div");
        top.className = "dashboard-component-top";
        top.append(textNode("h4", "", component.name), textNode("span", "dashboard-status", statusLabel(component.status)));
        card.append(top, textNode("p", "", component.rationale));
        const evidence = document.createElement("div");
        evidence.className = "dashboard-evidence";
        for (const item of component.evidence || []) {
          const [path, ...anchorParts] = item.split("#");
          const anchor = anchorParts.length ? `#${anchorParts.join("#")}` : "";
          evidence.append(sourceLink(`https://github.com/vsimko/gfabasic-rust/blob/main/${path}${anchor}`, `${item} ↗`));
        }
        for (const issue of component.related_issues || []) {
          evidence.append(sourceLink(`https://github.com/vsimko/gfabasic-rust/issues/${issue}`, `Issue #${issue} ↗`));
        }
        card.append(evidence);
        grid.append(card);
      }

      const footer = document.createElement("div");
      footer.className = "dashboard-footer";
      footer.append(
        sourceLink("https://github.com/vsimko/gfabasic-rust/blob/main/validation/project.json", "Canonical status model ↗"),
        sourceLink("https://github.com/vsimko/gfabasic-rust/blob/main/ROADMAP.md", "Delivery roadmap ↗"),
        sourceLink(`https://github.com/vsimko/gfabasic-rust/commit/${data.source_revision}`, `Source revision: ${data.source_revision.slice(0, 7)} ↗`),
        sourceLink("https://github.com/vsimko/gfabasic-rust/issues", "Open implementation issues ↗")
      );
      generatedStatus.replaceChildren(overview, heading, grid, footer);
      generatedStatus.setAttribute("aria-busy", "false");
    })
    .catch((error) => {
      generatedStatus.replaceChildren(textNode("p", "dashboard-message dashboard-error", `Dashboard data unavailable: ${error.message}. Check the generated site data or open the repository status model.`));
      generatedStatus.append(sourceLink("https://github.com/vsimko/gfabasic-rust/blob/main/validation/project.json", "Open canonical status model ↗"));
      generatedStatus.setAttribute("aria-busy", "false");
    });
}


const apiCatalogueResults = document.querySelector("#api-catalogue-results");
if (apiCatalogueResults) {
  const searchInput = document.querySelector("#api-search");
  const domainFilter = document.querySelector("#api-domain-filter");
  const evidenceFilter = document.querySelector("#api-evidence-filter");
  const implementationFilter = document.querySelector("#api-implementation-filter");
  const supportFilter = document.querySelector("#api-support-filter");
  const summary = document.querySelector("#api-catalogue-summary");
  let catalogueEntries = [];

  const makeText = (tag, className, value) => {
    const element = document.createElement(tag);
    if (className) element.className = className;
    element.textContent = value;
    return element;
  };

  const renderSummary = (data) => {
    const linux = "x86_64-unknown-linux-gnu";
    const dimensions = [
      { title: "Declarations", counts: { total: data.counts.total } },
      { title: "Evidence", counts: data.counts.by_evidence_status },
      { title: "Implementation", counts: data.counts.by_implementation_status },
      { title: "Linux support", counts: data.counts.by_target_support[linux] || {} },
      { title: "Conformance", counts: data.counts.by_target_conformance[linux] || {} },
    ];
    summary.replaceChildren(...dimensions.map((dimension) => {
      const card = document.createElement("article");
      card.className = "api-summary-card";
      card.append(makeText("h3", "", dimension.title));
      const lines = Object.entries(dimension.counts).map(([status, count]) => `${status}: ${count}`);
      card.append(makeText("p", "", lines.join(" · ") || "0"));
      return card;
    }));
  };

  const renderDetail = (container, detail) => {
    const content = document.createElement("div");
    content.className = "api-entry-detail";
    content.append(makeText("code", "api-entry-id", detail.id));
    if (detail.syntax?.length) {
      content.append(makeText("h4", "", "Syntax"));
      content.append(makeText("pre", "api-syntax", detail.syntax.map((item) => item.form).join("\n")));
    }
    if (detail.semantics?.summary) content.append(makeText("p", "api-summary-text", detail.semantics.summary));
    if (detail.signature && Object.keys(detail.signature).length) {
      content.append(makeText("h4", "", "Signature"));
      content.append(makeText("pre", "api-detail-code", JSON.stringify(detail.signature, null, 2)));
    }
    const statuses = document.createElement("div");
    statuses.className = "api-status-row";
    statuses.append(makeText("span", "api-status", `Evidence: ${detail.evidence?.status || "unknown"}`));
    statuses.append(makeText("span", "api-status", `Implementation: ${detail.implementation?.status || "unknown"}`));
    for (const item of detail.support || []) {
      statuses.append(makeText("span", "api-status", `${item.target}: ${item.classification} · ${item.conformance}`));
      if (item["pending-reason"]) content.append(makeText("p", "api-pending", item["pending-reason"]));
    }
    content.append(statuses);
    const evidenceParts = [
      ...(detail.evidence?.sources || []),
      ...(detail.evidence?.["printed-pages"] || []),
    ];
    if (evidenceParts.length) content.append(makeText("p", "api-evidence", evidenceParts.join(" · ")));
    const anchor = detail.docs?.anchors?.[0] || "";
    if (detail.docs?.path) {
      const link = document.createElement("a");
      link.className = "api-doc-link";
      link.href = `https://github.com/vsimko/gfabasic-rust/blob/main/${detail.docs.path}${anchor ? `#${anchor}` : ""}`;
      link.target = "_blank";
      link.rel = "noreferrer";
      link.textContent = "Source notes ↗";
      content.append(link);
    }
    container.replaceChildren(content);
  };

  const loadDetail = async (entry, panel) => {
    if (panel.dataset.loaded === "true" || panel.dataset.loading === "true") return;
    panel.dataset.loading = "true";
    const status = makeText("p", "api-detail-loading", "Loading static declaration detail…");
    panel.replaceChildren(status);
    try {
      const response = await fetch(`data/${entry.detail}`);
      if (!response.ok) throw new Error("Static declaration detail is unavailable");
      renderDetail(panel, await response.json());
      panel.dataset.loaded = "true";
    } catch {
      panel.replaceChildren(makeText("p", "api-empty", "Declaration detail is unavailable."));
    } finally {
      delete panel.dataset.loading;
    }
  };

  const renderEntries = () => {
    const query = (searchInput?.value || "").trim().toLowerCase();
    const domain = domainFilter?.value || "";
    const evidence = evidenceFilter?.value || "";
    const implementation = implementationFilter?.value || "";
    const support = supportFilter?.value || "";
    const filtered = catalogueEntries.filter((entry) => {
      const searchText = [entry.id, entry.name, entry.kind, entry.summary, ...(entry.syntax || [])]
        .join(" ").toLowerCase();
      return (!query || searchText.includes(query))
        && (!domain || entry.namespace === domain)
        && (!evidence || entry.evidence_status === evidence)
        && (!implementation || entry.implementation_status === implementation)
        && (!support || (entry.support || []).some((item) => item.classification === support));
    });

    if (!filtered.length) {
      apiCatalogueResults.replaceChildren(makeText("p", "api-empty", "No declarations match these filters."));
      return;
    }

    const groups = new Map();
    for (const entry of filtered) {
      const key = `${entry.namespace}/${entry.category}`;
      if (!groups.has(key)) groups.set(key, []);
      groups.get(key).push(entry);
    }

    const categoryNodes = [];
    for (const [key, entries] of groups) {
      const category = document.createElement("details");
      category.className = "api-category";
      category.dataset.category = key;
      const heading = document.createElement("summary");
      heading.append(makeText("span", "api-category-name", key.replace("/", " / ")));
      heading.append(makeText("span", "api-category-count", String(entries.length)));
      category.append(heading);

      const populateNames = () => {
        if (category.dataset.namesLoaded === "true") return;
        const names = document.createElement("div");
        names.className = "api-function-list";
        for (const entry of entries) {
          const item = document.createElement("details");
          item.className = "api-function";
          item.id = `api-entry-${entry.id}`;
          const name = document.createElement("summary");
          name.textContent = entry.name;
          const panel = document.createElement("div");
          panel.className = "api-function-panel";
          item.append(name, panel);
          item.addEventListener("toggle", () => {
            if (item.open) loadDetail(entry, panel);
          });
          names.append(item);
        }
        category.append(names);
        category.dataset.namesLoaded = "true";
      };
      category.addEventListener("toggle", () => {
        if (category.open) populateNames();
      });
      const shouldExpand = Boolean(query || domain || evidence || implementation || support);
      if (shouldExpand) {
        category.open = true;
        populateNames();
      }
      categoryNodes.push(category);
    }
    apiCatalogueResults.replaceChildren(...categoryNodes);
  };

  fetch("data/api-catalogue.json")
    .then((response) => {
      if (!response.ok) throw new Error("Generated API catalogue is unavailable");
      return response.json();
    })
    .then((data) => {
      catalogueEntries = data.entries || [];
      renderSummary(data);
      renderEntries();
      if (window.location.hash.startsWith("#api-entry-")) {
        const id = decodeURIComponent(window.location.hash.slice("#api-entry-".length));
        const entry = catalogueEntries.find((item) => item.id === id);
        if (entry) {
          const category = [...apiCatalogueResults.querySelectorAll(".api-category")]
            .find((item) => item.dataset.category === `${entry.namespace}/${entry.category}`);
          if (category) {
            category.open = true;
            category.dispatchEvent(new Event("toggle"));
            const target = document.getElementById(`api-entry-${id}`);
            if (target) {
              target.open = true;
              target.scrollIntoView();
            }
          }
        }
      }
    })
    .catch(() => {
      apiCatalogueResults.textContent = "Generated API catalogue data is unavailable. See api/registry in the repository.";
    });

  [searchInput, domainFilter, evidenceFilter, implementationFilter, supportFilter]
    .filter(Boolean)
    .forEach((control) => control.addEventListener(control.tagName === "INPUT" ? "input" : "change", renderEntries));
}
