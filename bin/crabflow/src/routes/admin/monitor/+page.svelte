<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount, onDestroy } from 'svelte'

  let logs = []
  let systemStatus = {
    cpu_usage: 0,
    memory_usage: 0,
    internet_connected: false,
    active_interface: "Unknown",
    timestamp: 0
  }
  let interval
  let autoRefresh = true

  async function refreshStatus() {
    try {
      systemStatus = await invoke("get_system_status")
    } catch (e) {
      console.error("Failed to get system status:", e)
    }
  }

  async function refreshLogs() {
    try {
      logs = await invoke("get_logs", { limit: 100 })
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

  onMount(() => {
    refreshStatus()
    refreshLogs()
    interval = setInterval(() => {
      refreshStatus()
      refreshLogs()
    }, 2000)
  })

  onDestroy(() => {
    if (interval) clearInterval(interval)
  })
</script>

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
    <!-- System Info Cards -->
    <div class="row">
      <div class="col-md-6 col-sm-6 col-12">
        <div class="info-box">
          <span class="info-box-icon bg-info"><i class="fas fa-microchip"></i></span>
          <div class="info-box-content">
            <span class="info-box-text">CPU Usage</span>
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
            <span class="info-box-text">Memory Usage</span>
            <span class="info-box-number">{systemStatus.memory_usage.toFixed(1)}%</span>
            <div class="progress">
              <div class="progress-bar bg-success" style="width: {systemStatus.memory_usage}%"></div>
            </div>
          </div>
        </div>
      </div>
      <div class="col-md-6 col-sm-6 col-12">
        <div class="info-box">
          <span class="info-box-icon bg-warning"><i class="fas fa-network-wired"></i></span>
          <div class="info-box-content">
            <span class="info-box-text">Interface</span>
            <span class="info-box-number">{systemStatus.active_interface}</span>
          </div>
        </div>
      </div>
      <div class="col-md-6 col-sm-6 col-12">
        <div class="info-box">
          <span class="info-box-icon {systemStatus.internet_connected ? 'bg-primary' : 'bg-danger'}">
            <i class="fas {systemStatus.internet_connected ? 'fa-globe' : 'fa-globe-americas'}"></i>
          </span>
          <div class="info-box-content">
            <span class="info-box-text">Internet</span>
            <span class="info-box-number">{systemStatus.internet_connected ? 'Connected' : 'Offline'}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Logs -->
    <div class="card">
      <div class="card-header">
        <h3 class="card-title">System Logs</h3>
        <div class="card-tools">
          <button type="button" class="btn btn-tool" on:click={toggleAutoRefresh}>
            <i class="fas {autoRefresh ? 'fa-pause' : 'fa-play'}"></i>
          </button>
          <button type="button" class="btn btn-tool" on:click={downloadLogs}>
            <i class="fas fa-download"></i> Save Log
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
