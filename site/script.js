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
  fetch("data/project.json")
    .then((response) => {
      if (!response.ok) throw new Error("Project data is unavailable");
      return response.json();
    })
    .then((data) => {
      const heading = document.createElement("h3");
      heading.textContent = data.project.name;
      const summary = document.createElement("p");
      summary.textContent = `${data.project.source_dialect} · ${data.project.target} · ${data.project.maturity}`;
      const prototype = document.createElement("p");
      prototype.textContent = data.prototype.description;
      const packages = document.createElement("p");
      packages.textContent = `Workspace crates: ${data.workspace_packages.map((item) => item.name).join(", ")}`;
      generatedStatus.replaceChildren(heading, summary, prototype, packages);
    })
    .catch(() => {
      generatedStatus.textContent = "Generated project data is not available. See the repository README and roadmap.";
    });
}


const apiCatalogueResults = document.querySelector("#api-catalogue-results");
if (apiCatalogueResults) {
  const searchInput = document.querySelector("#api-search");
  const domainFilter = document.querySelector("#api-domain-filter");
  const evidenceFilter = document.querySelector("#api-evidence-filter");
  const implementationFilter = document.querySelector("#api-implementation-filter");
  const supportFilter = document.querySelector("#api-support-filter");
  let catalogueEntries = [];

  const makeText = (tag, className, value) => {
    const element = document.createElement(tag);
    if (className) element.className = className;
    element.textContent = value;
    return element;
  };

  const renderSummary = (data) => {
    const summary = document.querySelector("#api-catalogue-summary");
    const linux = "x86_64-unknown-linux-gnu";
    const dimensions = [
      { title: "Declarations", counts: { total: data.counts.total } },
      { title: "Evidence", counts: data.counts.by_evidence_status },
      { title: "Implementation", counts: data.counts.by_implementation_status },
      { title: "Linux support", counts: data.counts.by_target_support[linux] || {} },
      { title: "Conformance", counts: data.counts.by_target_conformance[linux] || {} },
    ];
    const cards = dimensions.map((dimension) => {
      const card = document.createElement("article");
      card.className = "api-summary-card";
      card.append(makeText("h3", "", dimension.title));
      const lines = Object.entries(dimension.counts).map(([status, count]) => `${status}: ${count}`);
      card.append(makeText("p", "", lines.join(" · ") || "0"));
      return card;
    });
    summary.replaceChildren(...cards);
  };

  const renderEntries = () => {
    const query = (searchInput?.value || "").trim().toLowerCase();
    const domain = domainFilter?.value || "";
    const evidence = evidenceFilter?.value || "";
    const implementation = implementationFilter?.value || "";
    const support = supportFilter?.value || "";
    const filtered = catalogueEntries.filter((entry) => {
      const searchText = [
        entry.id,
        entry.name,
        entry.kind,
        entry.semantics?.summary || "",
        ...(entry.syntax || []).map((item) => item.form),
      ].join(" ").toLowerCase();
      return (!query || searchText.includes(query))
        && (!domain || entry.namespace === domain)
        && (!evidence || entry.evidence?.status === evidence)
        && (!implementation || entry.implementation?.status === implementation)
        && (!support || (entry.support || []).some((item) => item.classification === support));
    });

    if (!filtered.length) {
      apiCatalogueResults.replaceChildren(makeText("p", "api-empty", "No declarations match these filters."));
      return;
    }

    const cards = filtered.map((entry) => {
      const card = document.createElement("article");
      card.className = "api-entry-card";
      const top = document.createElement("div");
      top.className = "api-entry-top";
      const heading = makeText("h3", "", entry.name);
      top.append(heading, makeText("span", "api-kind", entry.kind));
      card.append(top);
      card.append(makeText("code", "api-entry-id", entry.id));

      const syntax = (entry.syntax || []).map((item) => item.form);
      if (syntax.length) card.append(makeText("p", "api-syntax", syntax.join("  ·  ")));
      if (entry.semantics?.summary) card.append(makeText("p", "api-summary-text", entry.semantics.summary));

      const statuses = document.createElement("div");
      statuses.className = "api-status-row";
      statuses.append(makeText("span", "api-status", `Evidence: ${entry.evidence?.status || "unknown"}`));
      statuses.append(makeText("span", "api-status", `Implementation: ${entry.implementation?.status || "unknown"}`));
      for (const item of entry.support || []) {
        statuses.append(makeText("span", "api-status", `${item.target}: ${item.classification} · ${item.conformance}`));
      }
      card.append(statuses);

      const evidenceParts = [
        ...(entry.evidence?.sources || []),
        ...(entry.evidence?.["printed-pages"] || []),
      ];
      if (evidenceParts.length) card.append(makeText("p", "api-evidence", evidenceParts.join(" · ")));

      const pending = (entry.support || []).map((item) => item["pending-reason"]).filter(Boolean);
      if (pending.length) card.append(makeText("p", "api-pending", pending.join(" ")));

      const anchor = entry.docs?.anchors?.[0] || "";
      if (entry.docs?.path) {
        const link = document.createElement("a");
        link.className = "api-doc-link";
        link.href = `https://github.com/vsimko/gfabasic-rust/blob/main/${entry.docs.path}${anchor ? `#${anchor}` : ""}`;
        link.target = "_blank";
        link.rel = "noreferrer";
        link.textContent = "Source notes ↗";
        card.append(link);
      }
      return card;
    });
    apiCatalogueResults.replaceChildren(...cards);
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
    })
    .catch(() => {
      apiCatalogueResults.textContent = "Generated API catalogue data is unavailable. See api/registry in the repository.";
    });

  [searchInput, domainFilter, evidenceFilter, implementationFilter, supportFilter]
    .filter(Boolean)
    .forEach((control) => control.addEventListener(control.tagName === "INPUT" ? "input" : "change", renderEntries));
}
