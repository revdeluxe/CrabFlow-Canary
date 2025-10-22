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
function goToDhcpServer() {
  window.location.href = "/dhcp-server";
}

document
  .getElementById("dhcp-server")
  ?.addEventListener("click", goToDhcpServer);
function goToDeviceManager() {
  window.location.href = "/device-manager";
}

document
  .getElementById("device-manager")
  ?.addEventListener("click", goToDeviceManager);
document.getElementById("acl")?.addEventListener("click", runDiag);
document.getElementById("network-config")?.addEventListener("click", runDiag);

function showErrorDialog(errorCode, errorMessage) {
  const modalOverlay = document.createElement("div");
  modalOverlay.style.position = "fixed";
  modalOverlay.style.top = "0";
  modalOverlay.style.left = "0";
  modalOverlay.style.width = "100%";
  modalOverlay.style.height = "100%";
  modalOverlay.style.backgroundColor = "rgba(0, 0, 0, 0.5)";
  modalOverlay.style.display = "flex";
  modalOverlay.style.justifyContent = "center";
  modalOverlay.style.alignItems = "center";
  modalOverlay.style.zIndex = "1000";

  const modalContent = document.createElement("div");
  modalContent.className = "card";
  modalContent.style.maxWidth = "400px";
  modalContent.style.padding = "24px";

  const title = document.createElement("h3");
  title.textContent = `Error ${errorCode}`;
  title.style.borderBottom = "1px solid var(--border-color)";
  title.style.paddingBottom = "16px";
  title.style.marginBottom = "16px";

  const message = document.createElement("p");
  message.textContent = errorMessage;

  const closeButton = document.createElement("button");
  closeButton.textContent = "Close";
  closeButton.style.marginTop = "24px";
  closeButton.onclick = () => {
    window.history.back();
  };

  modalContent.appendChild(title);
  modalContent.appendChild(message);
  modalContent.appendChild(closeButton);
  modalOverlay.appendChild(modalContent);
  document.body.appendChild(modalOverlay);
}

// --- Traffic Chart ---
const ctx = document.getElementById("trafficChart")?.getContext("2d");
if (ctx) {
  const interfaceSelect = document.getElementById("interface-select");
  const intervalSlider = document.getElementById("interval-slider");
  const intervalValue = document.getElementById("interval-value");
  let trafficData = {};
  let selectedInterface = "";
  let fetchIntervalId;

  const trafficChart = new Chart(ctx, {
    type: "line",
    data: {
      labels: [],
      datasets: [
        {
          label: "Received",
          data: [],
          borderColor: "rgb(75, 192, 192)",
          tension: 0.1,
        },
        {
          label: "Transmitted",
          data: [],
          borderColor: "rgb(255, 99, 132)",
          tension: 0.1,
        },
      ],
    },
    options: {
      scales: {
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: "kbps",
          },
        },
      },
      plugins: {
        tooltip: {
          callbacks: {
            label: function (context) {
              let label = context.dataset.label || "";
              if (label) {
                label += ": ";
              }
              if (context.parsed.y !== null) {
                label += context.parsed.y.toFixed(2) + " kbps";
              }
              return label;
            },
          },
        },
      },
    },
  });

  function updateChart() {
    if (!selectedInterface || !trafficData[selectedInterface]) {
      return;
    }

    const interfaceData = trafficData[selectedInterface];
    trafficChart.data.labels = interfaceData.labels;
    trafficChart.data.datasets[0].data = interfaceData.rx_data;
    trafficChart.data.datasets[1].data = interfaceData.tx_data;
    trafficChart.update();
  }

  async function fetchTrafficData() {
    try {
      const res = await fetch("/api/traffic");
      if (!res.ok) {
        throw new Error("Failed to fetch traffic data");
      }
      const data = await res.json();

      const now = new Date();
      const label = `${now.getHours()}:${now.getMinutes()}:${now.getSeconds()}`;
      const current_time = now.getTime();

      if (interfaceSelect.options.length === 0 && data.length > 0) {
        data.forEach((iface) => {
          const option = document.createElement("option");
          option.value = iface.name;
          option.textContent = iface.name;
          interfaceSelect.appendChild(option);
        });
        selectedInterface = data[0].name;
      }

      data.forEach((iface) => {
        if (!trafficData[iface.name]) {
          trafficData[iface.name] = {
            labels: [],
            rx_data: [],
            tx_data: [],
            last_rx: 0,
            last_tx: 0,
            last_fetch_time: 0,
            rx_history: [],
            tx_history: [],
          };
        }

        const interfaceStore = trafficData[iface.name];

        if (interfaceStore.last_rx > 0) {
          // only calculate diff if we have a previous value
          const time_diff_s =
            (current_time - interfaceStore.last_fetch_time) / 1000;
          if (time_diff_s > 0) {
            const rx_diff = Math.max(0, iface.rx - interfaceStore.last_rx);
            const tx_diff = Math.max(0, iface.tx - interfaceStore.last_tx);

            const rx_kbps = (rx_diff * 8) / (time_diff_s * 1000);
            const tx_kbps = (tx_diff * 8) / (time_diff_s * 1000);

            interfaceStore.rx_history.push(rx_kbps);
            interfaceStore.tx_history.push(tx_kbps);

            if (interfaceStore.rx_history.length > 3) {
              interfaceStore.rx_history.shift();
              interfaceStore.tx_history.shift();
            }

            const avg_rx_kbps =
              interfaceStore.rx_history.reduce((a, b) => a + b, 0) /
              interfaceStore.rx_history.length;
            const avg_tx_kbps =
              interfaceStore.tx_history.reduce((a, b) => a + b, 0) /
              interfaceStore.tx_history.length;

            interfaceStore.labels.push(label);
            interfaceStore.rx_data.push(avg_rx_kbps);
            interfaceStore.tx_data.push(avg_tx_kbps);

            if (interfaceStore.labels.length > 30) {
              interfaceStore.labels.shift();
              interfaceStore.rx_data.shift();
              interfaceStore.tx_data.shift();
            }
          }
        }

        interfaceStore.last_rx = iface.rx;
        interfaceStore.last_tx = iface.tx;
        interfaceStore.last_fetch_time = current_time;
      });

      updateChart();
    } catch (e) {
      console.error(e);
    }
  }

  function startFetching(interval) {
    if (fetchIntervalId) {
      clearInterval(fetchIntervalId);
    }
    fetchIntervalId = setInterval(fetchTrafficData, interval);
  }

  intervalSlider.addEventListener("input", (e) => {
    const newInterval = parseInt(e.target.value, 10);
    intervalValue.textContent = `${newInterval} ms`;
    startFetching(newInterval);
  });

  interfaceSelect.addEventListener("change", (e) => {
    selectedInterface = e.target.value;
    updateChart();
  });

  // Initial fetch and start interval
  fetchTrafficData();
  startFetching(parseInt(intervalSlider.value, 10));
}

// --- Device Manager ---
if (document.getElementById("dhcp-leases-grid")) {
  const dhcpLeasesGrid = document.getElementById("dhcp-leases-grid");

  async function fetchDhcpLeases() {
    try {
      const res = await fetch("/api/dhcp/leases");
      if (!res.ok) {
        throw new Error("Failed to fetch DHCP leases");
      }
      const leases = await res.json();

      dhcpLeasesGrid.innerHTML = ""; // Clear existing leases

      leases.forEach((lease) => {
        const card = document.createElement("div");
        card.className = "card";

        const ip = document.createElement("h3");
        ip.textContent = lease.ip_address;

        const mac = document.createElement("p");
        mac.innerHTML = `<strong>MAC:</strong> ${lease.mac_address}`;

        const hostname = document.createElement("p");
        hostname.innerHTML = `<strong>Hostname:</strong> ${lease.hostname}`;

        const leaseTime = document.createElement("p");
        leaseTime.innerHTML = `<strong>Lease Time:</strong> ${lease.lease_time}s`;

        const editButton = document.createElement("button");
        editButton.textContent = "Edit";
        editButton.onclick = () => {
          window.location.href = "/dhcp-server";
        };

        card.appendChild(ip);
        card.appendChild(mac);
        card.appendChild(hostname);
        card.appendChild(leaseTime);
        card.appendChild(editButton);

        dhcpLeasesGrid.appendChild(card);
      });
    } catch (e) {
      console.error(e);
    }
  }

  fetchDhcpLeases();
}

// --- Settings Form ---
const settingsForm = document.getElementById("settings-form");
if (settingsForm) {
  // Fetch and display current settings
  fetch("/api/settings")
    .then((res) => res.json())
    .then((settings) => {
      document.getElementById("app_name").value = settings.app_name;
      document.getElementById("port").value = settings.port;
      document.getElementById("enable_ui").checked = settings.enable_ui;
      document.getElementById("sdnc_mode").value = settings.sdnc_mode;
      document.getElementById("log_level").value = settings.log_level;
      document.getElementById("default_theme").value = settings.default_theme;
    });

  settingsForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    const formData = new FormData(settingsForm);
    const newSettings = {};
    for (const [key, value] of formData.entries()) {
      if (key === "enable_ui") {
        newSettings[key] = value === "on";
      } else if (key === "port") {
        newSettings[key] = parseInt(value, 10);
      } else {
        newSettings[key] = value;
      }
    }
    newSettings["enable_ui"] = document.getElementById("enable_ui").checked;

    try {
      const res = await fetch("/api/settings", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(newSettings),
      });

      if (res.ok) {
        alert("Settings saved successfully!");
      } else {
        throw new Error("Failed to save settings");
      }
    } catch (err) {
      console.error(err);
      alert("Failed to save settings");
    }
  });
}
