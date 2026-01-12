<script>
  import { onMount, onDestroy } from 'svelte'
  import { api } from '$lib/tauri'
  import { session } from '$lib/stores/session'

  let traffic = null
  let refreshInterval
  
  // Accordion state (expanded by default for connectivity test)
  let showConnectivityTest = true

  function formatSpeed(bps) {
    if (bps >= 1000000000) {
      return (bps / 1000000000).toFixed(2) + ' Gbps'
    } else if (bps >= 1000000) {
      return (bps / 1000000).toFixed(2) + ' Mbps'
    } else if (bps >= 1000) {
      return (bps / 1000).toFixed(2) + ' Kbps'
    }
    return bps.toFixed(0) + ' bps'
  }

  function formatBytes(bytes) {
    if (bytes >= 1073741824) {
      return (bytes / 1073741824).toFixed(2) + ' GB'
    } else if (bytes >= 1048576) {
      return (bytes / 1048576).toFixed(2) + ' MB'
    } else if (bytes >= 1024) {
      return (bytes / 1024).toFixed(2) + ' KB'
    }
    return bytes + ' B'
  }

  async function refresh() {
    try {
      traffic = await api.invokeCommand("get_traffic_summary")
    } catch (e) {
      console.error("Dashboard refresh failed:", e)
    }
  }

  onMount(() => {
    refresh()
    refreshInterval = setInterval(refresh, 2000)
  })

  onDestroy(() => {
    if (refreshInterval) clearInterval(refreshInterval)
  })
</script>

<div class="row mb-3">
  <div class="col-12">
    <h1 class="m-0"><i class="fas fa-tachometer-alt mr-2"></i>Portal Dashboard</h1>
  </div>
</div>

<div class="row">
  <div class="col-md-12">
    <div class="callout callout-info">
      <h5><i class="fas fa-user mr-2"></i>Welcome, {$session?.user?.username || 'Guest'}!</h5>
      <p class="mb-1">
        <strong>Status:</strong> 
        {#if $session?.user?.is_active}
          <span class="badge badge-success"><i class="fas fa-check mr-1"></i>Active</span>
        {:else}
          <span class="badge badge-warning"><i class="fas fa-clock mr-1"></i>Pending/Inactive</span>
        {/if}
      </p>
      <p class="mb-0 text-muted">Your network session is active. Monitor your traffic and connectivity below.</p>
    </div>
  </div>
</div>

<!-- Speed Cards -->
<div class="row">
  <div class="col-lg-3 col-md-6 col-sm-6">
    <div class="small-box bg-gradient-info">
      <div class="inner">
        {#if traffic}
          <h3>{formatSpeed(traffic.bps_rx || 0)}</h3>
        {:else}
          <h3><i class="fas fa-spinner fa-spin"></i></h3>
        {/if}
        <p>Download Speed</p>
      </div>
      <div class="icon">
        <i class="fas fa-download"></i>
      </div>
    </div>
  </div>
  
  <div class="col-lg-3 col-md-6 col-sm-6">
    <div class="small-box bg-gradient-success">
      <div class="inner">
        {#if traffic}
          <h3>{formatSpeed(traffic.bps_tx || 0)}</h3>
        {:else}
          <h3><i class="fas fa-spinner fa-spin"></i></h3>
        {/if}
        <p>Upload Speed</p>
      </div>
      <div class="icon">
        <i class="fas fa-upload"></i>
      </div>
    </div>
  </div>

  <div class="col-lg-3 col-md-6 col-sm-6">
    <div class="small-box bg-gradient-warning">
      <div class="inner">
        {#if traffic}
          <h3>{formatBytes(traffic.total_rx || 0)}</h3>
        {:else}
          <h3><i class="fas fa-spinner fa-spin"></i></h3>
        {/if}
        <p>Total Downloaded</p>
      </div>
      <div class="icon">
        <i class="fas fa-cloud-download-alt"></i>
      </div>
    </div>
  </div>

  <div class="col-lg-3 col-md-6 col-sm-6">
    <div class="small-box bg-gradient-danger">
      <div class="inner">
        {#if traffic}
          <h3>{formatBytes(traffic.total_tx || 0)}</h3>
        {:else}
          <h3><i class="fas fa-spinner fa-spin"></i></h3>
        {/if}
        <p>Total Uploaded</p>
      </div>
      <div class="icon">
        <i class="fas fa-cloud-upload-alt"></i>
      </div>
    </div>
  </div>
</div>

<!-- Traffic Distribution -->
<div class="row">
  <div class="col-lg-6 col-md-12">
    <div class="card card-outline card-primary">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-chart-pie mr-2"></i>Traffic Distribution</h3>
      </div>
      <div class="card-body">
        {#if traffic}
          <div class="row">
            <div class="col-6">
              <div class="info-box bg-light mb-0">
                <span class="info-box-icon bg-primary"><i class="fas fa-stream"></i></span>
                <div class="info-box-content">
                  <span class="info-box-text">TCP Traffic</span>
                  <span class="info-box-number">{traffic.tcp_pct || 0}%</span>
                  <div class="progress">
                    <div class="progress-bar bg-primary" style="width: {traffic.tcp_pct || 0}%"></div>
                  </div>
                </div>
              </div>
            </div>
            <div class="col-6">
              <div class="info-box bg-light mb-0">
                <span class="info-box-icon bg-success"><i class="fas fa-broadcast-tower"></i></span>
                <div class="info-box-content">
                  <span class="info-box-text">UDP Traffic</span>
                  <span class="info-box-number">{traffic.udp_pct || 0}%</span>
                  <div class="progress">
                    <div class="progress-bar bg-success" style="width: {traffic.udp_pct || 0}%"></div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        {:else}
          <div class="text-center text-muted py-3">
            <i class="fas fa-spinner fa-spin fa-2x"></i>
            <p class="mt-2">Loading traffic data...</p>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <div class="col-lg-6 col-md-12">
    <div class="card card-outline card-success">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-wifi mr-2"></i>Connection Status</h3>
      </div>
      <div class="card-body">
        <ul class="list-group list-group-flush">
          <li class="list-group-item d-flex justify-content-between align-items-center">
            <span><i class="fas fa-signal text-success mr-2"></i>Network Status</span>
            <span class="badge badge-success badge-pill">Connected</span>
          </li>
          <li class="list-group-item d-flex justify-content-between align-items-center">
            <span><i class="fas fa-shield-alt text-info mr-2"></i>Firewall</span>
            <span class="badge badge-info badge-pill">Active</span>
          </li>
          <li class="list-group-item d-flex justify-content-between align-items-center">
            <span><i class="fas fa-globe text-primary mr-2"></i>Internet Access</span>
            <span class="badge badge-primary badge-pill">Allowed</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</div>

<!-- Connectivity Test -->
<div class="row">
  <div class="col-12">
    <div class="card card-outline card-info">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-globe me-2"></i>Connectivity Test</h3>
        <div class="card-tools">
          <button type="button" class="btn btn-tool" on:click={() => showConnectivityTest = !showConnectivityTest}>
            <i class="fas {showConnectivityTest ? 'fa-minus' : 'fa-plus'}"></i>
          </button>
        </div>
      </div>
      <div class="card-body p-0" class:d-none={!showConnectivityTest}>
        <div class="connectivity-frame">
          <iframe 
            src="https://www.google.com/webhp?igu=1" 
            title="Google Connectivity Test"
            sandbox="allow-scripts allow-same-origin allow-forms"
          ></iframe>
        </div>
      </div>
      <div class="card-footer text-muted text-center" class:d-none={!showConnectivityTest}>
        <small><i class="fas fa-info-circle mr-1"></i>If Google loads, your internet connection is working properly.</small>
      </div>
    </div>
  </div>
</div>

<style>
  .connectivity-frame {
    position: relative;
    width: 100%;
    height: 400px;
    background: #f4f6f9;
  }
  .connectivity-frame iframe {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border: none;
  }
  .small-box .icon i {
    font-size: 70px;
  }
  .info-box-number {
    font-size: 1.5rem;
    font-weight: 700;
  }
  .progress {
    height: 5px;
    margin-top: 5px;
  }
</style>
