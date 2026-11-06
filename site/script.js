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
