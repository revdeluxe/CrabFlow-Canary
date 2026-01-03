<script>
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let leasesCount = 0
  let recordsCount = 0
  let traffic = null
  let devicesCount = 0

  async function refresh() {
    const leases = await invoke("list_leases")
    leasesCount = leases.length

    const records = await invoke("list_records")
    recordsCount = records.length

    traffic = await invoke("get_traffic_summary")
    const devices = await invoke("list_devices")
    devicesCount = devices.length
  }

  onMount(refresh)
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0">Portal Dashboard</h1>
  </div>
</div>

<div class="row">
  <div class="col-md-12">
    <div class="callout callout-info">
      <h5>Welcome!</h5>
      <p>Welcome to your network session. View traffic, device status, and profile info here.</p>
    </div>
  </div>
</div>

<div class="row">
  <div class="col-lg-4 col-6">
    <!-- small box -->
    <div class="small-box bg-info">
      <div class="inner">
        {#if traffic}
          <h3>{traffic.bps_rx} <small>bps</small></h3>
          <p>Download Speed</p>
        {:else}
          <h3>Loading...</h3>
        {/if}
      </div>
      <div class="icon">
        <i class="fas fa-download"></i>
      </div>
    </div>
  </div>
  
  <div class="col-lg-4 col-6">
    <!-- small box -->
    <div class="small-box bg-success">
      <div class="inner">
        {#if traffic}
          <h3>{traffic.bps_tx} <small>bps</small></h3>
          <p>Upload Speed</p>
        {:else}
          <h3>Loading...</h3>
        {/if}
      </div>
      <div class="icon">
        <i class="fas fa-upload"></i>
      </div>
    </div>
  </div>

  <div class="col-lg-4 col-6">
    <div class="info-box">
      <span class="info-box-icon bg-warning"><i class="fas fa-chart-pie"></i></span>
      <div class="info-box-content">
        <span class="info-box-text">Traffic Distribution</span>
        {#if traffic}
          <span class="info-box-number">TCP: {traffic.tcp_pct}%</span>
          <span class="info-box-number">UDP: {traffic.udp_pct}%</span>
        {:else}
          <span class="info-box-number">Loading...</span>
        {/if}
      </div>
    </div>
  </div>
</div>

<div class="row">
  <div class="col-md-12">
    <div class="card card-primary card-outline">
        <div class="card-header">
            <h3 class="card-title">Connectivity Test</h3>
        </div>
        <div class="card-body p-0">
            <div class="embed-responsive embed-responsive-16by9">
                <iframe class="embed-responsive-item" src="https://www.google.com/webhp?igu=1" allowfullscreen title="Google Connectivity Test"></iframe>
            </div>
        </div>
    </div>
  </div>
</div>
