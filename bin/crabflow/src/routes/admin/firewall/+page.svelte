<script>
  import { onMount } from 'svelte'
  
  // Mock Data for UI Demonstration
  let rules = [
    { id: 1, name: "Allow HTTP", port: 80, protocol: "TCP", action: "ALLOW" },
    { id: 2, name: "Allow HTTPS", port: 443, protocol: "TCP", action: "ALLOW" },
    { id: 3, name: "Block Telnet", port: 23, protocol: "TCP", action: "DENY" },
    { id: 4, name: "SSH Access", port: 22, protocol: "TCP", action: "ALLOW" }
  ]

  let showModal = false
  let newRule = {
    name: "",
    port: "",
    protocol: "TCP",
    action: "ALLOW"
  }

  function addRule() {
    if (!newRule.name || !newRule.port) {
      alert("Please fill in all fields")
      return
    }
    
    rules = [...rules, {
      id: Date.now(),
      name: newRule.name,
      port: parseInt(newRule.port),
      protocol: newRule.protocol,
      action: newRule.action
    }]
    
    newRule = { name: "", port: "", protocol: "TCP", action: "ALLOW" }
    showModal = false
  }

  function deleteRule(id) {
    if(confirm("Are you sure you want to delete this rule?")) {
      rules = rules.filter(r => r.id !== id)
    }
  }
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>Firewall Management</h1>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    <div class="card">
      <div class="card-header">
        <h3 class="card-title">Active Rules</h3>
        <div class="card-tools">
          <button type="button" class="btn btn-primary btn-sm" on:click={() => showModal = true}>
            <i class="fas fa-plus"></i> Add Rule
          </button>
        </div>
      </div>
      <div class="card-body table-responsive p-0">
        <table class="table table-hover text-nowrap">
          <thead>
            <tr>
              <th>Rule Name</th>
              <th>Port</th>
              <th>Protocol</th>
              <th>Action</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each rules as rule}
              <tr>
                <td>{rule.name}</td>
                <td>{rule.port}</td>
                <td><span class="badge badge-secondary">{rule.protocol}</span></td>
                <td>
                  {#if rule.action === 'ALLOW'}
                    <span class="badge badge-success">ALLOW</span>
                  {:else}
                    <span class="badge badge-danger">DENY</span>
                  {/if}
                </td>
                <td>
                  <button class="btn btn-danger btn-xs" on:click={() => deleteRule(rule.id)}>
                    <i class="fas fa-trash"></i>
                  </button>
                </td>
              </tr>
            {/each}
            {#if rules.length === 0}
              <tr><td colspan="5" class="text-center">No firewall rules active.</td></tr>
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
        <h4 class="modal-title">Add Firewall Rule</h4>
        <button type="button" class="close" on:click={() => showModal = false}>
          <span>&times;</span>
        </button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label>Rule Name</label>
          <input type="text" class="form-control" bind:value={newRule.name} placeholder="e.g. Web Server">
        </div>
        <div class="form-group">
          <label>Port</label>
          <input type="number" class="form-control" bind:value={newRule.port} placeholder="80">
        </div>
        <div class="form-group">
          <label>Protocol</label>
          <select class="form-control" bind:value={newRule.protocol}>
            <option value="TCP">TCP</option>
            <option value="UDP">UDP</option>
            <option value="ICMP">ICMP</option>
          </select>
        </div>
        <div class="form-group">
          <label>Action</label>
          <select class="form-control" bind:value={newRule.action}>
            <option value="ALLOW">ALLOW</option>
            <option value="DENY">DENY</option>
          </select>
        </div>
      </div>
      <div class="modal-footer justify-content-between">
        <button type="button" class="btn btn-default" on:click={() => showModal = false}>Close</button>
        <button type="button" class="btn btn-primary" on:click={addRule}>Add Rule</button>
      </div>
    </div>
  </div>
</div>
{/if}
