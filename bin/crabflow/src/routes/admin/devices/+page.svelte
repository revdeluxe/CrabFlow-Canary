<script>
  import { onMount } from 'svelte'
  import { api } from '$lib/tauri'

  let leases = []
  let loading = true
  let error = null
  let showModal = false

  let newLease = {
    ip: "",
    mac: "",
    hostname: ""
  }

  async function refresh() {
    try {
      leases = await api.invokeCommand("list_leases")
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
      await api.invokeCommand("remove_lease", { ip })
      refresh()
    } catch (e) {
      alert("Failed to remove lease: " + e)
    }
  }

  onMount(async () => {
    await refresh()
    loading = false
  })
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>Connected Devices</h1>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    <div class="card">
      <div class="card-header">
        <h3 class="card-title">DHCP Leases</h3>
        <div class="card-tools">
          <button type="button" class="btn btn-primary btn-sm" on:click={() => showModal = true}>
            <i class="fas fa-plus"></i> Add Static Lease
          </button>
          <button type="button" class="btn btn-tool" on:click={refresh}>
            <i class="fas fa-sync"></i>
          </button>
        </div>
      </div>
      <div class="card-body table-responsive p-0">
        <table class="table table-hover text-nowrap">
          <thead>
            <tr>
              <th>IP Address</th>
              <th>MAC Address</th>
              <th>Hostname</th>
              <th>Expires</th>
              <th>Type</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#if loading}
              <tr><td colspan="6" class="text-center">Loading...</td></tr>
            {:else if leases.length === 0}
              <tr><td colspan="6" class="text-center">No active leases found.</td></tr>
            {:else}
              {#each leases as lease}
                <tr>
                  <td>{lease.ip}</td>
                  <td>{lease.mac}</td>
                  <td>{lease.hostname}</td>
                  <td>{lease.expires_at}</td>
                  <td>
                    {#if lease.static_lease}
                      <span class="badge badge-success">Static</span>
                    {:else}
                      <span class="badge badge-info">Dynamic</span>
                    {/if}
                  </td>
                  <td>
                    <button class="btn btn-danger btn-xs" on:click={() => removeLease(lease.ip)}>
                      <i class="fas fa-trash"></i>
                    </button>
                  </td>
                </tr>
              {/each}
            {/if}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</section>

{#if showModal}
<div class="modal fade show" style="display: block; background: rgba(0,0,0,0.5)">
  <div class="modal-dialog">
    <div class="modal-content">
      <div class="modal-header">
        <h4 class="modal-title">Add Static Lease</h4>
        <button type="button" class="close" on:click={() => showModal = false}>
          <span>&times;</span>
        </button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label>IP Address</label>
          <input type="text" class="form-control" bind:value={newLease.ip} placeholder="192.168.1.x">
        </div>
        <div class="form-group">
          <label>MAC Address</label>
          <input type="text" class="form-control" bind:value={newLease.mac} placeholder="AA:BB:CC:DD:EE:FF">
        </div>
        <div class="form-group">
          <label>Hostname</label>
          <input type="text" class="form-control" bind:value={newLease.hostname} placeholder="Device Name">
        </div>
      </div>
      <div class="modal-footer justify-content-between">
        <button type="button" class="btn btn-default" on:click={() => showModal = false}>Close</button>
        <button type="button" class="btn btn-primary" on:click={addStaticLease}>Save changes</button>
      </div>
    </div>
  </div>
</div>
{/if}
