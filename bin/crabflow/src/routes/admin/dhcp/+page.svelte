<script>
  import { onMount, onDestroy } from 'svelte'
  import { api } from '$lib/tauri'

  let leases = []
  let loading = true
  let error = null
  let showModal = false
  let serverStatus = {
    running: false,
    mode: 'server', // 'server' or 'client'
    pool_start: '',
    pool_end: '',
    gateway: '',
    dns: ''
  }

  let newLease = {
    ip: "",
    mac: "",
    hostname: ""
  }
  
  let refreshInterval
  
  // Format expiry timestamp to readable date/time
  function formatExpiry(timestamp) {
    if (!timestamp || timestamp === 'never') return 'Never'
    try {
      // Handle both Unix timestamp and ISO string
      const ts = typeof timestamp === 'number' ? timestamp : parseInt(timestamp)
      if (isNaN(ts)) return timestamp
      const date = new Date(ts * 1000)
      return date.toLocaleString()
    } catch (e) {
      return timestamp
    }
  }

  async function refresh() {
    try {
      leases = await api.invokeCommand("list_leases")
      
      // Get live stats for DHCP server status
      const liveStats = await api.invokeCommand('get_live_stats')
      serverStatus.running = liveStats.services_status.dhcp
      
      // Try to get DHCP config for pool info
      try {
        const config = await api.invokeCommand("load_setup")
        if (config && config.dhcp) {
          serverStatus.pool_start = config.dhcp.range_start || ''
          serverStatus.pool_end = config.dhcp.range_end || ''
          serverStatus.gateway = config.dhcp.gateway || ''
          serverStatus.dns = config.dhcp.dns || ''
          serverStatus.mode = config.dhcp.enabled ? 'server' : 'client'
        }
      } catch (e) {
        console.error("Failed to load DHCP config:", e)
      }
    } catch (e) {
      console.error("Failed to load leases:", e)
      error = e
    }
  }

  async function addStaticLease() {
    try {
      await api.invokeCommand("add_static_lease", { input: newLease })
      newLease = { ip: "", mac: "", hostname: "" }
      refresh()
      showModal = false
      alert("Static lease added successfully")
    } catch (e) {
      alert("Failed to add lease: " + e)
    }
  }

  async function removeLease(ip) {
    if (!confirm(`Are you sure you want to remove lease for ${ip}?`)) return
    try {
      await invoke("remove_lease", { ip })
      refresh()
    } catch (e) {
      alert("Failed to remove lease: " + e)
    }
  }

  onMount(async () => {
    await refresh()
    loading = false
    // Auto-refresh every 5 seconds
    refreshInterval = setInterval(refresh, 5000)
  })
  
  onDestroy(() => {
    if (refreshInterval) clearInterval(refreshInterval)
  })
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>
          DHCP Management
          <a href="/admin/about/guides/dhcp" class="btn btn-sm btn-outline-info ml-2" title="View DHCP Setup Guide">
            <i class="fas fa-question-circle"></i>
          </a>
        </h1>
      </div>
      <div class="col-sm-6">
        <div class="float-sm-right">
          <a href="/admin/settings#network-settings" class="btn btn-primary">
            <i class="fas fa-cogs mr-1"></i> Configure DHCP
          </a>
        </div>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    <!-- DHCP Server Status Card -->
    <div class="row mb-3">
      <div class="col-12">
        <div class="card card-outline {serverStatus.running ? 'card-success' : 'card-danger'}">
          <div class="card-header">
            <h3 class="card-title">
              <i class="fas fa-server mr-2"></i>
              DHCP Server Status
            </h3>
            <div class="card-tools">
              <span class="badge {serverStatus.running ? 'badge-success' : 'badge-danger'} badge-lg">
                <i class="fas fa-circle mr-1"></i>
                {serverStatus.running ? 'Running' : 'Stopped'}
              </span>
            </div>
          </div>
          <div class="card-body p-0">
            <table class="table table-striped mb-0">
              <tbody>
                <tr>
                  <td style="width: 50px;">
                    <span class="badge {serverStatus.mode === 'server' ? 'badge-primary' : 'badge-secondary'} p-2">
                      <i class="fas {serverStatus.mode === 'server' ? 'fa-server' : 'fa-laptop'}"></i>
                    </span>
                  </td>
                  <td>
                    <strong>Mode</strong><br>
                    <small class="text-muted">{serverStatus.mode === 'server' ? 'Assigning IPs' : 'Receiving IP'}</small>
                  </td>
                  <td class="text-right">
                    <span class="badge {serverStatus.mode === 'server' ? 'badge-primary' : 'badge-secondary'}">
                      {serverStatus.mode === 'server' ? 'DHCP Server' : 'Client Only'}
                    </span>
                  </td>
                </tr>
                <tr>
                  <td>
                    <span class="badge badge-info p-2"><i class="fas fa-network-wired"></i></span>
                  </td>
                  <td>
                    <strong>IP Pool</strong><br>
                    <small class="text-muted">{serverStatus.pool_end ? 'to ' + serverStatus.pool_end : 'Range end'}</small>
                  </td>
                  <td class="text-right">
                    <code>{serverStatus.pool_start || 'Not configured'}</code>
                  </td>
                </tr>
                <tr>
                  <td>
                    <span class="badge badge-warning p-2"><i class="fas fa-door-open"></i></span>
                  </td>
                  <td>
                    <strong>Gateway</strong><br>
                    <small class="text-muted">Default route</small>
                  </td>
                  <td class="text-right">
                    <code>{serverStatus.gateway || 'Not set'}</code>
                  </td>
                </tr>
                <tr>
                  <td>
                    <span class="badge badge-success p-2"><i class="fas fa-laptop"></i></span>
                  </td>
                  <td>
                    <strong>Active Leases</strong><br>
                    <small class="text-muted">Connected clients</small>
                  </td>
                  <td class="text-right">
                    <span class="badge badge-success">{leases.length}</span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
    
    {#if loading}
      <div class="d-flex justify-content-center">
        <div class="spinner-border text-primary" role="status">
          <span class="sr-only">Loading...</span>
        </div>
      </div>
    {:else if error}
      <div class="alert alert-danger">
        <i class="icon fas fa-ban"></i> {error}
      </div>
    {:else}
      <div class="row">
        <div class="col-md-12">
          <div class="card card-primary card-outline">
            <div class="card-header">
              <h3 class="card-title">Active & Static Leases</h3>
              <div class="card-tools">
                <button type="button" class="btn btn-success btn-sm mr-2" on:click={() => showModal = true}>
                  <i class="fas fa-plus"></i> Add Static Lease
                </button>
                <button type="button" class="btn btn-tool" on:click={refresh}>
                  <i class="fas fa-sync"></i>
                </button>
              </div>
            </div>
            <div class="card-body p-0">
              <table class="table table-striped">
                <thead>
                  <tr>
                    <th>IP Address</th>
                    <th>MAC Address</th>
                    <th>Hostname</th>
                    <th>Type</th>
                    <th>Expires</th>
                    <th style="width: 120px">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {#each leases as l}
                    <tr>
                      <td>
                        <i class="fas fa-circle text-success mr-1" style="font-size: 8px;"></i>
                        {l.ip}
                      </td>
                      <td><code>{l.mac}</code></td>
                      <td>{l.hostname || '-'}</td>
                      <td>
                        {#if l.static_lease}
                          <span class="badge badge-info">Static</span>
                        {:else}
                          <span class="badge badge-secondary">Dynamic</span>
                        {/if}
                      </td>
                      <td>
                        {#if l.static_lease}
                          <span class="text-muted">Never</span>
                        {:else}
                          {formatExpiry(l.expires_at)}
                        {/if}
                      </td>
                      <td>
                        <a href="/admin/dns" class="btn btn-info btn-xs mr-1" title="Create DNS Record">
                          <i class="fas fa-globe"></i>
                        </a>
                        <button class="btn btn-danger btn-xs" on:click={() => removeLease(l.ip)} title="Remove">
                          <i class="fas fa-trash"></i>
                        </button>
                      </td>
                    </tr>
                  {:else}
                    <tr>
                      <td colspan="6" class="text-center text-muted">
                        <i class="fas fa-info-circle mr-1"></i> No leases found
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
            <div class="card-footer text-muted">
              <small><i class="fas fa-sync-alt mr-1"></i> Auto-refreshes every 5 seconds</small>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</section>

{#if showModal}
  <div class="modal fade show" style="display: block; background: rgba(0,0,0,0.5);">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title">Add Static Lease</h4>
          <button type="button" class="close" on:click={() => showModal = false}>
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <form on:submit|preventDefault={addStaticLease}>
          <div class="modal-body">
            <div class="form-group">
              <label>IP Address</label>
              <input type="text" class="form-control" bind:value={newLease.ip} placeholder="192.168.1.x" required />
            </div>
            <div class="form-group">
              <label>MAC Address</label>
              <input type="text" class="form-control" bind:value={newLease.mac} placeholder="00:11:22:33:44:55" required />
            </div>
            <div class="form-group">
              <label>Hostname</label>
              <input type="text" class="form-control" bind:value={newLease.hostname} placeholder="Optional" />
            </div>
          </div>
          <div class="modal-footer justify-content-between">
            <button type="button" class="btn btn-default" on:click={() => showModal = false}>Cancel</button>
            <button type="submit" class="btn btn-success">Add Lease</button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}
