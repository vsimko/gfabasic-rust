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
