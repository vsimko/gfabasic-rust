import assert from "node:assert/strict";
import { chromium } from "playwright";

const baseURL = process.env.PREVIEW_URL;
assert.ok(baseURL, "PREVIEW_URL must point to the deployed PR preview");

const browser = await chromium.launch({ headless: true });
const failures = [];
const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
page.on("pageerror", error => failures.push(`pageerror: ${error.message}`));
page.on("requestfailed", request => {
  if (request.url().startsWith(baseURL)) failures.push(`request failed: ${request.url()}`);
});

try {
  let response;
  for (let attempt = 0; attempt < 12; attempt += 1) {
    try {
      response = await page.goto(baseURL, { waitUntil: "domcontentloaded", timeout: 15000 });
      if (response?.ok()) break;
    } catch {}
    await page.waitForTimeout(5000);
  }
  assert.ok(response?.ok(), `Preview did not return HTTP 200: ${response?.status()}`);
  await page.locator("#dashboard .dashboard-content").waitFor({ state: "visible" });
  await page.locator("#api-catalogue-results .api-category").first().waitFor({ state: "attached", timeout: 20000 });

  // Server-rendered dashboard and roadmap must be visible in the deployed HTML.
  const dashboard = page.locator("#dashboard");
  await assertText(dashboard, "Project maturity");
  await assertText(dashboard, "Product target");
  await assertText(dashboard, "Workspace");
  await assertText(dashboard, "Implementation states");
  assert.ok(await dashboard.locator(".dashboard-roadmap-progress .dashboard-progress-fill").count() >= 1,
    "Dashboard roadmap progress bars are missing");
  assert.ok(await dashboard.locator(".dashboard-component").count() >= 1,
    "Dashboard implementation states are missing");
  const roadmap = page.locator("#roadmap");
  assert.ok(await roadmap.locator(".roadmap-phase").count() >= 1, "Roadmap phases are missing");
  assert.ok(await roadmap.locator(".dashboard-progress-fill").count() >= 1, "Roadmap progress bars are missing");

  // API catalogue loads, starts collapsed, and loads details only after user interaction.
  const categories = page.locator("#api-catalogue-results .api-category");
  assert.ok(await categories.count() > 0, "API catalogue categories did not load");
  assert.equal(await categories.first().getAttribute("open"), null, "API categories should start collapsed");
  await categories.first().locator(":scope > summary").click();
  const functions = categories.first().locator(".api-function");
  await functions.first().waitFor({ state: "attached" });
  await functions.first().locator("summary").click();
  await functions.first().locator(".api-entry-detail").waitFor({ state: "visible", timeout: 15000 });

  // Search returns matching declarations and the mobile navigation works.
  const functionName = (await functions.first().locator("summary").innerText()).trim();
  const search = page.locator("#api-search");
  await search.fill(functionName);
  assert.ok(await page.locator("#api-catalogue-results .api-function").count() >= 1,
    "API search did not return the selected declaration");

  await page.setViewportSize({ width: 390, height: 844 });
  const menu = page.locator(".menu-toggle");
  await menu.click();
  assert.equal(await menu.getAttribute("aria-expanded"), "true", "Mobile menu did not open");
  await page.locator("#main-nav a[href='#dashboard']").click();
  assert.equal(await menu.getAttribute("aria-expanded"), "false", "Mobile menu did not close after navigation");
  const overflow = await page.evaluate(() =>
    document.documentElement.scrollWidth > document.documentElement.clientWidth
  );
  assert.equal(overflow, false, "Mobile page has horizontal overflow");

  assert.deepEqual(failures, [], "Preview emitted browser errors or same-origin failed requests");
  console.log("Preview QA passed: dashboard, roadmap, API catalogue/detail/search, mobile navigation and responsive layout.");
} finally {
  await browser.close();
}

async function assertText(locator, text) {
  assert.ok((await locator.innerText()).includes(text), `Missing expected preview text: ${text}`);
}
