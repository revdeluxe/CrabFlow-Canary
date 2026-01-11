<script>
  import { onMount, onDestroy } from 'svelte'
  import { api } from '$lib/tauri'

  import { invoke } from '@tauri-apps/api/core'

  let leasesCount = 0
  let recordsCount = 0
  let usersCount = 0
  let dnsStats = { total: 0, blocked: 0, percentage: 0 }
  
  let liveStats = {
      dhcp_clients: 0,
      dns_queries_total: 0,
      active_users: 0,
      services_status: { dhcp: false, dns: false }
  };

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
  let interfaces = []
  let recentDnsLogs = []
  let uptimeSeconds = 0
  let startTime = Date.now()

  let interval
  let cpuCanvas
  let memCanvas
  let cpuChart
  let memChart

  // Utility: format uptime
  function formatUptime(seconds) {
    const h = Math.floor(seconds / 3600)
    const m = Math.floor((seconds % 3600) / 60)
    const s = Math.floor(seconds % 60)
    return `${h}h ${m}m ${s}s`
  }

  // Utility: format bytes
  function formatBytes(bytes) {
    if (bytes < 1024) return bytes + ' B'
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
    if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB'
    return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB'
  }

  async function refresh() {
    try {
      liveStats = await invoke('get_live_stats');
      leasesCount = liveStats.dhcp_clients;
      usersCount = liveStats.active_users || 0;

      const records = await api.listRecords()
      recordsCount = records.length

      systemStatus = await api.getSystemStatus()
      interfaces = await api.listInterfaces()
      
      const config = await api.loadSetup()
      dhcpActive = liveStats.services_status.dhcp;

      // DNS Stats from query logs
      try {
        const logs = await api.getQueryLogs(100)
        recentDnsLogs = logs.slice(0, 5)
        const blocked = logs.filter(l => l.status === 'Blocked').length
        dnsStats = { 
            total: liveStats.dns_queries_total, 
            blocked: blocked,
            percentage: logs.length > 0 ? ((blocked / logs.length) * 100).toFixed(1) : 0
        };
      } catch (e) {
        console.error("Failed to get DNS logs:", e)
      }

      // Update uptime
      uptimeSeconds = Math.floor((Date.now() - startTime) / 1000)

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
    interval = setInterval(refresh, 2000)
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
  <div class="col-sm-6">
    <div class="float-sm-right">
      <span class="badge badge-secondary mr-2">
        <i class="fas fa-clock"></i> Uptime: {formatUptime(uptimeSeconds)}
      </span>
      <span class="badge {systemStatus.internet_connected ? 'badge-success' : 'badge-danger'}">
        <i class="fas fa-globe"></i> {systemStatus.internet_connected ? 'Online' : 'Offline'}
      </span>
    </div>
  </div>
</div>

<!-- Quick Stats Row -->
<div class="row">
  <div class="col-lg-3 col-6">
    <div class="small-box bg-info">
      <div class="inner">
        <h3>{leasesCount}</h3>
        <p>DHCP Clients</p>
      </div>
      <div class="icon"><i class="fas fa-laptop"></i></div>
      <a href="/admin/dhcp" class="small-box-footer">Manage <i class="fas fa-arrow-circle-right"></i></a>
    </div>
  </div>
  <div class="col-lg-3 col-6">
    <div class="small-box bg-success">
      <div class="inner">
        <h3>{recordsCount}</h3>
        <p>DNS Records</p>
      </div>
      <div class="icon"><i class="fas fa-dns"></i></div>
      <a href="/admin/dns" class="small-box-footer">Manage <i class="fas fa-arrow-circle-right"></i></a>
    </div>
  </div>
  <div class="col-lg-3 col-6">
    <div class="small-box bg-warning">
      <div class="inner">
        <h3>{dnsStats.total}</h3>
        <p>DNS Queries</p>
      </div>
      <div class="icon"><i class="fas fa-search"></i></div>
      <a href="/admin/monitor" class="small-box-footer">View Logs <i class="fas fa-arrow-circle-right"></i></a>
    </div>
  </div>
  <div class="col-lg-3 col-6">
    <div class="small-box bg-danger">
      <div class="inner">
        <h3>{dnsStats.blocked} <small>({dnsStats.percentage}%)</small></h3>
        <p>Ads Blocked</p>
      </div>
      <div class="icon"><i class="fas fa-ban"></i></div>
      <a href="/admin/adblock" class="small-box-footer">Configure <i class="fas fa-arrow-circle-right"></i></a>
    </div>
  </div>
</div>

<!-- Services Status Row -->
<div class="row">
  <div class="col-12">
    <div class="card card-outline card-primary">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-server mr-2"></i>Services Status</h3>
      </div>
      <div class="card-body p-0">
        <table class="table table-striped mb-0">
          <tbody>
            <tr>
              <td style="width: 50px;">
                <span class="badge {liveStats.services_status.dhcp ? 'badge-success' : 'badge-danger'} badge-lg p-2">
                  <i class="fas fa-network-wired"></i>
                </span>
              </td>
              <td>
                <strong>DHCP Server</strong>
                <br><small class="text-muted">{leasesCount} active leases</small>
              </td>
              <td class="text-right">
                <span class="badge {liveStats.services_status.dhcp ? 'badge-success' : 'badge-danger'}">
                  {liveStats.services_status.dhcp ? 'Running' : 'Stopped'}
                </span>
              </td>
            </tr>
            <tr>
              <td>
                <span class="badge {liveStats.services_status.dns ? 'badge-success' : 'badge-danger'} badge-lg p-2">
                  <i class="fas fa-globe"></i>
                </span>
              </td>
              <td>
                <strong>DNS Server</strong>
                <br><small class="text-muted">{dnsStats.total} queries handled</small>
              </td>
              <td class="text-right">
                <span class="badge {liveStats.services_status.dns ? 'badge-success' : 'badge-danger'}">
                  {liveStats.services_status.dns ? 'Running' : 'Stopped'}
                </span>
              </td>
            </tr>
            <tr>
              <td>
                <span class="badge {systemStatus.internet_connected ? 'badge-success' : 'badge-warning'} badge-lg p-2">
                  <i class="fas fa-wifi"></i>
                </span>
              </td>
              <td>
                <strong>Internet</strong>
                <br><small class="text-muted">{systemStatus.active_interface}</small>
              </td>
              <td class="text-right">
                <span class="badge {systemStatus.internet_connected ? 'badge-success' : 'badge-warning'}">
                  {systemStatus.internet_connected ? 'Connected' : 'Disconnected'}
                </span>
              </td>
            </tr>
            <tr>
              <td>
                <span class="badge badge-info badge-lg p-2">
                  <i class="fas fa-users"></i>
                </span>
              </td>
              <td>
                <strong>Portal Users</strong>
                <br><small class="text-muted">Authenticated clients</small>
              </td>
              <td class="text-right">
                <span class="badge badge-info">{usersCount} active</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>

<div class="row">
  <div class="col-md-6">
    <div class="card card-danger card-outline">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-microchip mr-2"></i>CPU Usage</h3>
      </div>
      <div class="card-body">
        <canvas bind:this={cpuCanvas} style="min-height: 200px; height: 200px; max-height: 200px; max-width: 100%;"></canvas>
        <div class="text-center mt-2">
          <span class="text-danger font-weight-bold">{systemStatus.cpu_usage.toFixed(1)}%</span> System
          <span class="mx-2">|</span>
          <span class="text-warning font-weight-bold">{systemStatus.app_cpu_usage.toFixed(2)}%</span> CrabFlow
        </div>
      </div>
    </div>
  </div>
  <div class="col-md-6">
    <div class="card card-primary card-outline">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-memory mr-2"></i>Memory Usage</h3>
      </div>
      <div class="card-body">
        <canvas bind:this={memCanvas} style="min-height: 200px; height: 200px; max-height: 200px; max-width: 100%;"></canvas>
        <div class="text-center mt-2">
          <span class="text-primary font-weight-bold">{systemStatus.memory_usage.toFixed(1)}%</span> System
          <span class="mx-2">|</span>
          <span class="text-warning font-weight-bold">{formatBytes(systemStatus.app_memory_usage)}</span> CrabFlow
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Recent DNS Activity -->
<div class="row">
  <div class="col-md-6">
    <div class="card card-outline card-info">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-history mr-2"></i>Recent DNS Activity</h3>
        <div class="card-tools">
          <a href="/admin/monitor" class="btn btn-tool"><i class="fas fa-external-link-alt"></i></a>
        </div>
      </div>
      <div class="card-body p-0">
        <table class="table table-sm table-striped mb-0">
          <tbody>
            {#if recentDnsLogs.length === 0}
              <tr><td class="text-center text-muted">No recent queries</td></tr>
            {:else}
              {#each recentDnsLogs as log}
                <tr class={log.status === 'Blocked' ? 'table-danger' : ''}>
                  <td class="text-truncate" style="max-width: 200px;">{log.domain}</td>
                  <td>
                    {#if log.status === 'Blocked'}
                      <span class="badge badge-danger badge-sm">Blocked</span>
                    {:else if log.status === 'Portal'}
                      <span class="badge badge-info badge-sm">Portal</span>
                    {:else}
                      <span class="badge badge-success badge-sm">OK</span>
                    {/if}
                  </td>
                  <td class="text-muted"><small>{log.client_ip}</small></td>
                </tr>
              {/each}
            {/if}
          </tbody>
        </table>
      </div>
    </div>
  </div>
  
  <!-- Quick Actions -->
  <div class="col-md-6">
    <div class="card card-outline card-secondary">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-bolt mr-2"></i>Quick Actions</h3>
      </div>
      <div class="card-body">
        <div class="row">
          <div class="col-6 mb-2">
            <a href="/admin/dns" class="btn btn-outline-primary btn-block">
              <i class="fas fa-plus-circle"></i> Add DNS Record
            </a>
          </div>
          <div class="col-6 mb-2">
            <a href="/admin/dhcp" class="btn btn-outline-info btn-block">
              <i class="fas fa-desktop"></i> Manage Leases
            </a>
          </div>
          <div class="col-6 mb-2">
            <a href="/admin/acl" class="btn btn-outline-warning btn-block">
              <i class="fas fa-shield-alt"></i> ACL Rules
            </a>
          </div>
          <div class="col-6 mb-2">
            <a href="/admin/users" class="btn btn-outline-success btn-block">
              <i class="fas fa-users"></i> Manage Users
            </a>
          </div>
          <div class="col-6 mb-2">
            <a href="/admin/firewall" class="btn btn-outline-danger btn-block">
              <i class="fas fa-fire"></i> Firewall
            </a>
          </div>
          <div class="col-6 mb-2">
            <a href="/admin/monitor" class="btn btn-outline-dark btn-block">
              <i class="fas fa-chart-line"></i> Monitor
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Network Interfaces (Collapsed) -->
<div class="row">
    <div class="col-12">
        <div class="card card-outline card-secondary collapsed-card">
            <div class="card-header">
                <h3 class="card-title"><i class="fas fa-ethernet mr-2"></i>Network Interfaces</h3>
                <div class="card-tools">
                    <button type="button" class="btn btn-tool" data-card-widget="collapse">
                        <i class="fas fa-plus"></i>
                    </button>
                </div>
            </div>
            <div class="card-body p-0">
                <table class="table table-sm table-striped">
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
                                <td>
                                  {iface.name}
                                  {#if iface.name === systemStatus.active_interface}
                                    <span class="badge badge-success ml-1">Active</span>
                                  {/if}
                                </td>
                                <td>
                                    {#each iface.ips as ip}
                                        <span class="badge bg-light mr-1 text-dark border">{ip}</span>
                                    {/each}
                                </td>
                                <td><small class="text-muted">{iface.mac || '-'}</small></td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</div>

<!-- Hotspot Control (Collapsed by default) -->
<div class="row">
  <div class="col-md-12">
    <div class="card card-primary collapsed-card">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-wifi mr-2"></i>Hotspot Control</h3>
        <div class="card-tools">
          <button type="button" class="btn btn-tool" data-card-widget="collapse">
            <i class="fas fa-plus"></i>
          </button>
        </div>
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


