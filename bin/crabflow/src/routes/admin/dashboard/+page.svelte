<script>
  import { onMount, onDestroy } from 'svelte'
  import { api } from '$lib/tauri'

  import { invoke } from '@tauri-apps/api/core'

  let leasesCount = 0
  let recordsCount = 0
  let dnsStats = { total: 0, blocked: 0, percentage: 0 }
  let systemStatus = {
    cpu_usage: 0,
    memory_usage: 0,
    total_memory: 0,
    swap_total: 0,
    swap_used: 0,
    swap_percentage: 0,
    app_cpu_usage: 0,
    app_memory_usage: 0,
    internet_connected: false,
    active_interface: "Unknown"
  }
  let dhcpActive = false
  
  let hotspotSsid = ""
  let hotspotKey = ""
  let hotspotLoading = false

  let interval
  let cpuCanvas
  let memCanvas
  let cpuChart
  let memChart

  async function refresh() {
    try {
      const leases = await api.listLeases()
      leasesCount = leases.length

      const records = await api.listRecords()
      recordsCount = records.length

      systemStatus = await api.getSystemStatus()
      
      const config = await api.loadSetup()
      dhcpActive = config.dhcp && config.dhcp.enabled

      // DNS Stats
      const logs = await invoke("get_query_logs", { limit: 1000 })
      const total = logs.length
      const blocked = logs.filter(l => l.status === 'Blocked').length
      const percentage = total > 0 ? ((blocked / total) * 100).toFixed(1) : 0
      dnsStats = { total, blocked, percentage }

      updateCharts()
    } catch (e) {
      console.error("Failed to refresh dashboard:", e)
    }
  }

  function initCharts() {
    if (typeof Chart === 'undefined') return

    if (cpuCanvas) {
      cpuChart = new Chart(cpuCanvas.getContext('2d'), {
        type: 'doughnut',
        data: {
          labels: ['App', 'System', 'Free'],
          datasets: [{
            data: [0, 0, 100],
            backgroundColor: ['#ffc107', '#dc3545', '#d2d6de'],
          }]
        },
        options: {
          maintainAspectRatio: false,
          responsive: true,
          legend: {
            display: true,
            position: 'bottom'
          },
          cutoutPercentage: 70
        }
      })
    }

    if (memCanvas) {
      memChart = new Chart(memCanvas.getContext('2d'), {
        type: 'doughnut',
        data: {
          labels: ['App', 'System', 'Free'],
          datasets: [{
            data: [0, 0, 100],
            backgroundColor: ['#ffc107', '#007bff', '#d2d6de'],
          }]
        },
        options: {
          maintainAspectRatio: false,
          responsive: true,
          legend: {
            display: true,
            position: 'bottom'
          },
          cutoutPercentage: 70
        }
      })
    }
  }

  function updateCharts() {
    if (cpuChart) {
      const appCpu = systemStatus.app_cpu_usage || 0
      const totalCpu = systemStatus.cpu_usage || 0
      const sysCpu = Math.max(0, totalCpu - appCpu)
      const freeCpu = Math.max(0, 100 - totalCpu)
      
      cpuChart.data.datasets[0].data = [appCpu, sysCpu, freeCpu]
      cpuChart.update()
    }
    if (memChart) {
      const totalMem = systemStatus.total_memory || 1
      const appMemBytes = systemStatus.app_memory_usage || 0
      const appMemPct = (appMemBytes / totalMem) * 100
      
      const totalMemPct = systemStatus.memory_usage || 0
      const sysMemPct = Math.max(0, totalMemPct - appMemPct)
      const freeMemPct = Math.max(0, 100 - totalMemPct)

      memChart.data.datasets[0].data = [appMemPct, sysMemPct, freeMemPct]
      memChart.update()
    }
  }

  async function startHotspot() {
    hotspotLoading = true
    try {
      await api.createHotspot(hotspotSsid, hotspotKey)
      alert("Hotspot started!")
    } catch (e) {
      alert("Failed to start hotspot: " + e)
    } finally {
      hotspotLoading = false
    }
  }

  async function stopHotspot() {
    hotspotLoading = true
    try {
      await api.stopHotspot()
      alert("Hotspot stopped!")
    } catch (e) {
      alert("Failed to stop hotspot: " + e)
    } finally {
      hotspotLoading = false
    }
  }

  onMount(async () => {
    // Wait a bit for Chart.js to load if it's from CDN
    setTimeout(initCharts, 500)
    
    refresh()
    try {
      const config = await api.loadSetup()
      const updateFreq = config.monitor_interval || 5000
      interval = setInterval(refresh, updateFreq)
    } catch (e) {
      console.error("Failed to load config for interval:", e)
      interval = setInterval(refresh, 5000)
    }
  })

  onDestroy(() => {
    if (interval) clearInterval(interval)
    if (cpuChart) cpuChart.destroy()
    if (memChart) memChart.destroy()
  })
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0">Admin Dashboard</h1>
  </div>
</div>

<div class="row">
  <!-- At a Glance Card -->
  <div class="col-md-12">
    <div class="card card-outline card-primary">
      <div class="card-body">
        <div class="row">
          <div class="col-md-6 col-sm-6 col-12">
            <div class="info-box shadow-none">
              <span class="info-box-icon bg-info"><i class="fas fa-network-wired"></i></span>
              <div class="info-box-content">
                <span class="info-box-text">DHCP Server</span>
                <span class="info-box-number">
                  <span class={dhcpActive ? "text-success" : "text-danger"}>{dhcpActive ? "Active" : "Inactive"}</span>
                </span>
              </div>
            </div>
          </div>
          <div class="col-md-6 col-sm-6 col-12">
            <div class="info-box shadow-none">
              <span class="info-box-icon {systemStatus.internet_connected ? 'bg-success' : 'bg-danger'}">
                <i class="fas {systemStatus.internet_connected ? 'fa-globe' : 'fa-globe-americas'}"></i>
              </span>
              <div class="info-box-content">
                <span class="info-box-text">Internet Status</span>
                <span class="info-box-number">{systemStatus.internet_connected ? 'Online' : 'Offline'}</span>
              </div>
            </div>
          </div>
        </div>
        <div class="row">
          <div class="col-md-6 col-sm-6 col-12">
            <div class="info-box shadow-none">
              <span class="info-box-icon bg-warning"><i class="fas fa-shield-alt"></i></span>
              <div class="info-box-content">
                <span class="info-box-text">DNS Queries</span>
                <span class="info-box-number">{dnsStats.total}</span>
              </div>
            </div>
          </div>
          <div class="col-md-6 col-sm-6 col-12">
            <div class="info-box shadow-none">
              <span class="info-box-icon bg-danger"><i class="fas fa-ban"></i></span>
              <div class="info-box-content">
                <span class="info-box-text">Ads Blocked</span>
                <span class="info-box-number">{dnsStats.blocked} ({dnsStats.percentage}%)</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<div class="row">
  <div class="col-lg-6 col-6">
    <!-- small box -->
    <div class="small-box bg-info">
      <div class="inner">
        <h3>{leasesCount}</h3>
        <p>DHCP Clients</p>
      </div>
      <div class="icon">
        <i class="fas fa-laptop"></i>
      </div>
      <a href="/admin/devices" class="small-box-footer">More info <i class="fas fa-arrow-circle-right"></i></a>
    </div>
  </div>
  <!-- ./col -->
  <div class="col-lg-6 col-6">
    <!-- small box -->
    <div class="small-box bg-success">
      <div class="inner">
        <h3>{recordsCount}</h3>
        <p>DNS Records</p>
      </div>
      <div class="icon">
        <i class="fas fa-globe"></i>
      </div>
      <a href="/admin/dns" class="small-box-footer">More info <i class="fas fa-arrow-circle-right"></i></a>
    </div>
  </div>
</div>

<div class="row">
  <div class="col-md-6">
    <div class="card card-danger card-outline">
      <div class="card-header">
        <h3 class="card-title">CPU Usage</h3>
      </div>
      <div class="card-body">
        <canvas bind:this={cpuCanvas} style="min-height: 250px; height: 250px; max-height: 250px; max-width: 100%;"></canvas>
        <div class="text-center mt-3">
          <span class="text-danger font-weight-bold">{systemStatus.cpu_usage.toFixed(1)}%</span> Used
          <br>
          <small class="text-muted">App: {systemStatus.app_cpu_usage.toFixed(2)}%</small>
        </div>
      </div>
    </div>
  </div>
  <div class="col-md-6">
    <div class="card card-primary card-outline">
      <div class="card-header">
        <h3 class="card-title">Memory Usage</h3>
      </div>
      <div class="card-body">
        <canvas bind:this={memCanvas} style="min-height: 250px; height: 250px; max-height: 250px; max-width: 100%;"></canvas>
        <div class="text-center mt-3">
          <span class="text-primary font-weight-bold">{systemStatus.memory_usage.toFixed(1)}%</span> Used
          <br>
          <small class="text-muted">
            App: {(systemStatus.app_memory_usage / 1024 / 1024).toFixed(1)} MB | 
            Swap: {systemStatus.swap_percentage.toFixed(1)}%
          </small>
        </div>
      </div>
    </div>
  </div>
</div>

<div class="row">
  <div class="col-md-12">
    <div class="card card-primary">
      <div class="card-header">
        <h3 class="card-title">Hotspot Control</h3>
      </div>
      <div class="card-body">
        <div class="row">
          <div class="col-md-6">
            <div class="form-group">
              <label>SSID</label>
              <input type="text" class="form-control" bind:value={hotspotSsid} placeholder="Enter SSID">
            </div>
          </div>
          <div class="col-md-6">
            <div class="form-group">
              <label>Password</label>
              <input type="password" class="form-control" bind:value={hotspotKey} placeholder="Enter Password">
            </div>
          </div>
        </div>
      </div>
      <div class="card-footer">
        <button class="btn btn-primary" on:click={startHotspot} disabled={hotspotLoading}>
          <i class="fas fa-wifi"></i> Start Hotspot
        </button>
        <button class="btn btn-danger float-right" on:click={stopHotspot} disabled={hotspotLoading}>
          <i class="fas fa-stop"></i> Stop
        </button>
      </div>
    </div>
  </div>
</div>


