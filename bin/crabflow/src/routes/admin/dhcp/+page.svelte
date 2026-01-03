<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

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
      leases = await invoke("list_leases")
    } catch (e) {
      console.error("Failed to load leases:", e)
      error = e
    }
  }

  async function addStaticLease() {
    try {
      await invoke("add_static_lease", { input: newLease })
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
  })
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>DHCP Management</h1>
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
                    <th>Expires</th>
                    <th style="width: 40px">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {#each leases as l}
                    <tr>
                      <td>{l.ip}</td>
                      <td>{l.mac}</td>
                      <td>{l.hostname || '-'}</td>
                      <td>
                        {#if l.static_lease}
                          <span class="badge badge-info">Static</span>
                        {:else}
                          {l.expires_at}
                        {/if}
                      </td>
                      <td>
                        <button class="btn btn-danger btn-sm" on:click={() => removeLease(l.ip)}>
                          <i class="fas fa-trash"></i>
                        </button>
                      </td>
                    </tr>
                  {:else}
                    <tr>
                      <td colspan="5" class="text-center">No leases found</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
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
