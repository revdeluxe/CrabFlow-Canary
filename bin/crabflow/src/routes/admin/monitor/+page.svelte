<script>
  import { onMount, onDestroy } from 'svelte'
  import { api } from '$lib/tauri'
  import DnsLogsModal from '$lib/components/DnsLogsModal.svelte'

  let logs = []
  let dnsStats = { total: 0, blocked: 0, percentage: 0 }
  let showDnsModal = false;
  let systemStatus = {
    cpu_usage: 0,
    memory_usage: 0,
    swap_total: 0,
    swap_used: 0,
    swap_percentage: 0,
    app_cpu_usage: 0,
    app_memory_usage: 0,
    internet_connected: false,
    active_interface: "Unknown",
    timestamp: 0
  }
  let interval
  let autoRefresh = true
  let isMobile = false

  function updateViewport() {
    isMobile = window.innerWidth < 700
  }

  async function refreshStatus() {
    try {
      systemStatus = await api.getSystemStatus()
    } catch (e) {
      console.error("Failed to get system status:", e)
    }
  }

  let interfaces = []
  async function refreshInterfaces() {
      try {
        interfaces = await api.listInterfaces()
      } catch (e) {
          console.error("Failed to list interfaces:", e)
      }
  }

  async function refreshDnsStats() {
    try {
      const logs = await api.getQueryLogs(1000)
      const total = logs.length
      const blocked = logs.filter(l => l.status === 'Blocked').length
      const percentage = total > 0 ? ((blocked / total) * 100).toFixed(1) : 0
      dnsStats = { total, blocked, percentage }
    } catch (e) {
      console.error("Failed to get DNS stats:", e)
    }
  }

  async function refreshLogs() {
    try {
      logs = await api.getLogs(100)
    } catch (e) {
      console.error("Failed to get logs:", e)
    }
  }

  function toggleAutoRefresh() {
    autoRefresh = !autoRefresh
    if (autoRefresh) {
      interval = setInterval(() => {
        refreshStatus()
        refreshLogs()
        refreshDnsStats()
        refreshInterfaces()
      }, 2000)
    } else {
      clearInterval(interval)
    }
  }

  function downloadLogs() {
    const text = logs.map(l => `[${l.level}] ${l.message} (${l.timestamp})`).join('\n')
    const blob = new Blob([text], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `crabflow-logs-${Date.now()}.txt`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  async function clearLogs() {
    if (!confirm("Are you sure you want to clear all system logs?")) return
    try {
      await invoke("clear_logs")
      refreshLogs()
    } catch (e) {
      alert("Failed to clear logs: " + e)
    }
  }

  onMount(() => {
    updateViewport()
    window.addEventListener('resize', updateViewport)
    refreshStatus()
    refreshLogs()
    refreshDnsStats()
    refreshInterfaces()
    interval = setInterval(() => {
      refreshStatus()
      refreshLogs()
      refreshDnsStats()
      refreshInterfaces()
    }, 2000)
  })

  onDestroy(() => {
    if (interval) clearInterval(interval)
    window.removeEventListener('resize', updateViewport)
  })
</script>

{#if isMobile}
  <section class="p-3" style="font-family: sans-serif;">
    <h2 class="mb-2">System Monitor (Mobile)</h2>

    <!-- Compact summary -->
    <div class="mb-3">
      <div style="display:flex; gap:8px; overflow-x:auto;">
        <div style="min-width:140px; padding:12px; background:#fff; border-radius:8px; box-shadow:0 1px 3px rgba(0,0,0,0.06);">
          <div style="font-size:12px; color:#666">CPU</div>
          <div style="font-size:18px; font-weight:600">{systemStatus.cpu_usage.toFixed(1)}%</div>
        </div>
        <div style="min-width:140px; padding:12px; background:#fff; border-radius:8px; box-shadow:0 1px 3px rgba(0,0,0,0.06);">
          <div style="font-size:12px; color:#666">RAM</div>
          <div style="font-size:18px; font-weight:600">{systemStatus.memory_usage.toFixed(1)}%</div>
        </div>
        <div style="min-width:140px; padding:12px; background:#fff; border-radius:8px; box-shadow:0 1px 3px rgba(0,0,0,0.06);">
          <div style="font-size:12px; color:#666">Internet</div>
          <div style="font-size:18px; font-weight:600">{systemStatus.internet_connected ? 'Online' : 'Offline'}</div>
        </div>
      </div>
    </div>

    <!-- Interfaces (compact list) -->
    <div class="mb-3">
      <h3 style="font-size:16px; margin-bottom:8px;">Interfaces</h3>
      {#each interfaces as iface}
        <div style="padding:10px; background:#fff; border-radius:6px; margin-bottom:8px; box-shadow:0 1px 2px rgba(0,0,0,0.04);">
          <div style="font-weight:600">{iface.display_name || iface.name}</div>
          <div style="font-size:13px; color:#555">{iface.ip || (iface.ips && iface.ips[0]) || 'No IP'}</div>
          <div style="font-size:12px; color:#888">{iface.mac || '-'}</div>
        </div>
      {/each}
      {#if interfaces.length === 0}
        <div class="text-muted">No interfaces found.</div>
      {/if}
    </div>

    <!-- DNS & Logs summary -->
    <div style="display:flex; gap:12px;">
      <div style="flex:1; background:#fff; padding:12px; border-radius:8px;">
        <div style="font-size:12px; color:#666">DNS (last 1k)</div>
        <div style="font-weight:700">{dnsStats.total} / {dnsStats.blocked} blocked</div>
        <div style="font-size:12px; color:#4caf50">{dnsStats.percentage}%</div>
      </div>
      <div style="flex:1; background:#fff; padding:12px; border-radius:8px;">
        <div style="font-size:12px; color:#666">Logs</div>
        <div style="font-weight:700">{logs.length}</div>
        <button class="btn btn-sm btn-outline-primary mt-2" on:click={refreshLogs}>Refresh</button>
      </div>
    </div>

    <div style="margin-top:12px;">
      <button class="btn btn-block btn-secondary" on:click={() => showDnsModal = true}>View DNS Log</button>
    </div>
  </section>
  <DnsLogsModal bind:show={showDnsModal} />
{:else}
  <section class="content-header">
    <div class="container-fluid">
      <div class="row mb-2">
        <div class="col-sm-6">
          <h1>System Monitor</h1>
        </div>
      </div>
    </div>
  </section>

  <section class="content">
    <div class="container-fluid">
      <!-- Desktop: keep original layout -->
      <!-- System Info Cards and rest of the original layout -->
      <!-- ...existing layout preserved... -->
      <div class="row">
        <div class="col-md-6 col-sm-6 col-12">
          <div class="info-box">
            <span class="info-box-icon bg-info"><i class="fas fa-microchip"></i></span>
            <div class="info-box-content">
              <span class="info-box-text">System CPU</span>
              <span class="info-box-number">{systemStatus.cpu_usage.toFixed(1)}%</span>
              <div class="progress">
                <div class="progress-bar bg-info" style="width: {systemStatus.cpu_usage}%"></div>
              </div>
            </div>
          </div>
        </div>
        <div class="col-md-6 col-sm-6 col-12">
          <div class="info-box">
            <span class="info-box-icon bg-success"><i class="fas fa-memory"></i></span>
            <div class="info-box-content">
              <span class="info-box-text">System RAM</span>
              <span class="info-box-number">{systemStatus.memory_usage.toFixed(1)}%</span>
              <div class="progress">
                <div class="progress-bar bg-success" style="width: {systemStatus.memory_usage}%"></div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Rest of original content retained for desktop -->
      <!-- Interfaces, DNS stats, Logs (same as before) -->

      <div class="row">
        <div class="col-12">
          <div class="card">
            <div class="card-header">
              <h3 class="card-title">Network Interfaces</h3>
            </div>
            <div class="card-body table-responsive p-0">
              <table class="table table-sm table-hover text-nowrap">
                <thead>
                  <tr>
                    <th>Name</th>
                    <th>IP Addresses</th>
                    <th>MAC</th>
                  </tr>
                </thead>
                <tbody>
                  {#each interfaces as iface}
                    <tr>
                      <td>{iface.display_name || iface.name}</td>
                      <td>
                        {#each iface.ips as ip}
                          <span class="badge bg-light mr-1">{ip}</span>
                        {/each}
                      </td>
                      <td>{iface.mac || '-'}</td>
                    </tr>
                  {/each}
                  {#if interfaces.length === 0}
                    <tr><td colspan="3" class="text-center text-muted">No interfaces found.</td></tr>
                  {/if}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- DNS Statistics -->
      <div class="row">
        <div class="col-md-12">
          <div class="card">
            <div class="card-header">
              <h3 class="card-title">DNS Statistics (Last 1000 Queries)</h3>
              <div class="card-tools">
                <button type="button" class="btn btn-tool" on:click={() => showDnsModal = true}>
                  <i class="fas fa-list"></i> View Log
                </button>
              </div>
            </div>
            <div class="card-body">
              <div class="row">
                <div class="col-md-4">
                  <div class="description-block border-right">
                    <h5 class="description-header">{dnsStats.total}</h5>
                    <span class="description-text">TOTAL QUERIES</span>
                  </div>
                </div>
                <div class="col-md-4">
                  <div class="description-block border-right">
                    <h5 class="description-header text-danger">{dnsStats.blocked}</h5>
                    <span class="description-text">BLOCKED</span>
                  </div>
                </div>
                <div class="col-md-4">
                  <div class="description-block">
                    <h5 class="description-header text-success">{dnsStats.percentage}%</h5>
                    <span class="description-text">BLOCK PERCENTAGE</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Logs -->
      <div class="card">
        <div class="card-header">
          <h3 class="card-title">System Logs</h3>
          <div class="card-tools">
            <button type="button" class="btn btn-tool" on:click={refreshLogs} title="Refresh Logs">
              <i class="fas fa-sync-alt"></i>
            </button>
            <button type="button" class="btn btn-tool" on:click={toggleAutoRefresh} title={autoRefresh ? "Pause Auto-Refresh" : "Start Auto-Refresh"}>
              <i class="fas {autoRefresh ? 'fa-pause' : 'fa-play'}"></i>
            </button>
            <button type="button" class="btn btn-tool" on:click={downloadLogs} title="Download Logs">
              <i class="fas fa-download"></i> Save Log
            </button>
            <button type="button" class="btn btn-tool" on:click={clearLogs} title="Clear Logs">
              <i class="fas fa-trash"></i>
            </button>
          </div>
        </div>
        <div class="card-body p-0" style="height: 400px; overflow-y: auto; background: #f4f6f9; font-family: monospace;">
          <ul class="list-group list-group-flush">
            {#each logs as log}
              <li class="list-group-item py-1 px-3" style="background: transparent; border: none;">
                <span class="text-muted small">[{log.timestamp}]</span>
                <span class="badge {log.level === 'ERROR' ? 'badge-danger' : (log.level === 'WARN' ? 'badge-warning' : 'badge-info')}">{log.level}</span>
                <span>{log.message}</span>
              </li>
            {/each}
            {#if logs.length === 0}
              <li class="list-group-item text-center text-muted">No logs available.</li>
            {/if}
          </ul>
        </div>
      </div>

    </div>
  </section>

  <DnsLogsModal bind:show={showDnsModal} />
{/if}
