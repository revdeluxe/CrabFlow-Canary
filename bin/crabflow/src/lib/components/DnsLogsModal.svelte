<script>
  import { onMount, createEventDispatcher } from 'svelte'
  import { api } from '$lib/tauri'

  export let show = false
  const dispatch = createEventDispatcher()

  let logs = []
  let filteredLogs = []
  let searchTerm = ""
  let loading = false

  async function loadLogs() {
    loading = true
    try {
      logs = await api.getQueryLogs(1000)
      filterLogs()
    } catch (e) {
      console.error(e)
    } finally {
      loading = false
    }
  }

  function filterLogs() {
    if (!searchTerm) {
      filteredLogs = logs
    } else {
      const lower = searchTerm.toLowerCase()
      filteredLogs = logs.filter(l => 
        l.domain.toLowerCase().includes(lower) || 
        l.client_ip.includes(lower) ||
        l.status.toLowerCase().includes(lower)
      )
    }
  }

  function close() {
    show = false
    dispatch('close')
  }

  // Reload when modal opens
  $: if (show) {
    loadLogs()
  }

  $: searchTerm, filterLogs()
</script>

{#if show}
<div class="modal show d-block" tabindex="-1" role="dialog" style="background: rgba(0,0,0,0.5)">
  <div class="modal-dialog modal-lg modal-dialog-scrollable" role="document">
    <div class="modal-content">
      <div class="modal-header">
        <h5 class="modal-title">DNS Activity Logs</h5>
        <button type="button" class="close" aria-label="Close" on:click={close}>
          <span aria-hidden="true">&times;</span>
        </button>
      </div>
      <div class="modal-body">
        <div class="form-group">
            <input type="text" class="form-control" placeholder="Search domain or IP..." bind:value={searchTerm}>
        </div>
        
        {#if loading}
            <div class="text-center p-3">
                <div class="spinner-border text-primary" role="status"></div>
            </div>
        {:else}
            <div class="table-responsive">
                <table class="table table-sm table-striped">
                    <thead>
                        <tr>
                            <th>Time</th>
                            <th>Client IP</th>
                            <th>Domain</th>
                            <th>Type</th>
                            <th>Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each filteredLogs as log}
                            <tr class={log.status === 'Blocked' ? 'table-danger' : (log.status === 'Portal' ? 'table-info' : '')}>
                                <td>{new Date(log.timestamp * 1000).toLocaleTimeString()}</td>
                                <td>{log.client_ip}</td>
                                <td>{log.domain}</td>
                                <td>{log.query_type}</td>
                                <td>
                                    {#if log.status === 'Blocked'}
                                        <span class="badge badge-danger">Blocked</span>
                                    {:else if log.status === 'Redirected'}
                                        <span class="badge badge-warning">Redirected</span>
                                    {:else if log.status === 'Portal'}
                                        <span class="badge badge-info">Portal</span>
                                    {:else}
                                        <span class="badge badge-success">Allowed</span>
                                    {/if}
                                </td>
                            </tr>
                        {/each}
                        {#if filteredLogs.length === 0}
                            <tr>
                                <td colspan="5" class="text-center">No logs found.</td>
                            </tr>
                        {/if}
                    </tbody>
                </table>
            </div>
        {/if}
      </div>
      <div class="modal-footer">
        <button type="button" class="btn btn-secondary" on:click={close}>Close</button>
        <button type="button" class="btn btn-primary" on:click={loadLogs}>Refresh</button>
      </div>
    </div>
  </div>
</div>
{/if}
