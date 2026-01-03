<script>
  import { onMount, onDestroy } from 'svelte'
  import { api } from '$lib/tauri'

  let leasesCount = 0
  let recordsCount = 0
  let systemStatus = {
    cpu_usage: 0,
    memory_usage: 0,
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
          labels: ['Used', 'Free'],
          datasets: [{
            data: [0, 100],
            backgroundColor: ['#dc3545', '#d2d6de'],
          }]
        },
        options: {
          maintainAspectRatio: false,
          responsive: true,
          legend: {
            display: false
          },
          cutoutPercentage: 70 // For Chart.js 2.x, use cutout: '70%' for 3.x
        }
      })
    }

    if (memCanvas) {
      memChart = new Chart(memCanvas.getContext('2d'), {
        type: 'doughnut',
        data: {
          labels: ['Used', 'Free'],
          datasets: [{
            data: [0, 100],
            backgroundColor: ['#007bff', '#d2d6de'],
          }]
        },
        options: {
          maintainAspectRatio: false,
          responsive: true,
          legend: {
            display: false
          },
          cutoutPercentage: 70
        }
      })
    }
  }

  function updateCharts() {
    if (cpuChart) {
      cpuChart.data.datasets[0].data = [systemStatus.cpu_usage, 100 - systemStatus.cpu_usage]
      cpuChart.update()
    }
    if (memChart) {
      memChart.data.datasets[0].data = [systemStatus.memory_usage, 100 - systemStatus.memory_usage]
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
    <h1 class="m-0">
        Admin Dashboard 
        <small class="text-muted" style="font-size: 0.5em; vertical-align: middle;">
            CrabFlow: <span class="text-success">Online</span> | 
            DHCP: <span class={dhcpActive ? "text-success" : "text-danger"}>{dhcpActive ? "Active" : "Inactive"}</span>
        </small>
    </h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item">
        {#if systemStatus.internet_connected}
          <span class="badge badge-success">Online</span>
        {:else}
          <span class="badge badge-danger">Offline</span>
        {/if}
      </li>
    </ol>
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
          <span class="text-danger font-weight-bold">{systemStatus.cpu_usage}%</span> Used
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
          <span class="text-primary font-weight-bold">{systemStatus.memory_usage}%</span> Used
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


