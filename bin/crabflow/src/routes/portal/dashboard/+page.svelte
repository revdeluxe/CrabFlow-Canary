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

<style>
  .dashboard {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    padding: 2rem;
  }
  .card {
    background: #fff;
    border-radius: 8px;
    padding: 1rem;
    box-shadow: 0 2px 6px rgba(0,0,0,0.1);
  }
  .title {
    font-size: 1.2rem;
    margin-bottom: 0.5rem;
  }
  .value {
    font-size: 2rem;
    font-weight: bold;
  }
</style>

<h1>CrabFlow Dashboard</h1>

<h2>Portal Dashboard</h2>
<p>Welcome to your network session. View traffic, device status, and profile info here.</p>

<div class="card">
  <div class="title">Traffic</div>
  {#if traffic}
    <div class="value">{traffic.bps_rx}↓ / {traffic.bps_tx}↑</div>
    <small>TCP {traffic.tcp_pct}% | UDP {traffic.udp_pct}% | ICMP {traffic.icmp_pct}%</small>
  {:else}
    <div class="value">Loading…</div>
  {/if}
</div>
