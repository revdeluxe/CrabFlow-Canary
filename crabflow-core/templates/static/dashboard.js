const themeToggle = document.getElementById("theme-toggle");
const body = document.body;

function applyTheme(theme) {
  body.setAttribute("data-theme", theme);
  localStorage.setItem("theme", theme);
}

themeToggle.addEventListener("click", () => {
  const currentTheme = body.getAttribute("data-theme");
  const newTheme = currentTheme === "light" ? "dark" : "light";
  applyTheme(newTheme);
});

// Apply saved theme on load
const savedTheme = localStorage.getItem("theme") || "light";
applyTheme(savedTheme);

// --- Button Functions ---

function goToSettings() {
  window.location.href = "/settings";
}

async function fetchStatus() {
  try {
    const res = await fetch("/status.json");
    if (!res.ok) throw new Error("status fetch failed");
    const data = await res.json();
    // update DOM
    document.querySelector(".card-system .kv dd:nth-of-type(1)").textContent =
      data.os || "";
    document.querySelector(".card-system .kv dd:nth-of-type(2)").textContent =
      data.cpu || "";
    document.querySelector(".card-system .kv dd:nth-of-type(3)").textContent =
      data.memory || "";
    const list = document.querySelector(".iflist");
    if (list) {
      list.innerHTML = data.interfaces
        .map((i) => `<li><code>${i}</code></li>`)
        .join("");
    }
  } catch (e) {
    console.error(e);
    alert("Failed to fetch status");
  }
}

async function runDiag() {
  try {
    const res = await fetch("/diag", { method: "POST" });
    if (!res.ok) throw new Error("diag failed");
    const txt = await res.text();
    alert("Not Implemented");
  } catch (e) {
    console.error(e);
    alert("Failed to run diagnostic");
  }
}

// --- Event Listeners ---

document.getElementById("settings")?.addEventListener("click", goToSettings);
document.getElementById("refresh")?.addEventListener("click", fetchStatus);
document.getElementById("run-diagnostic")?.addEventListener("click", runDiag);
document.getElementById("setup-wizard")?.addEventListener("click", runDiag);
document.getElementById("dhcp-server")?.addEventListener("click", runDiag);
document.getElementById("device-manager")?.addEventListener("click", runDiag);
document.getElementById("acl")?.addEventListener("click", runDiag);
document.getElementById("network-config")?.addEventListener("click", runDiag);
