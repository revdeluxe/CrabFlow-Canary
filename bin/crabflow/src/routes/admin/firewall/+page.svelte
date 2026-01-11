<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let rules = []
  let loading = true
  let showModal = false
  
  // Edit State
  let editMode = false
  let currentId = ""

  let newRule = {
    name: "",
    port: 80,
    protocol: "TCP",
    action: "ALLOW",
    direction: "INBOUND"
  }

  async function refresh() {
    try {
      rules = await invoke("list_firewall_rules")
    } catch (e) {
      console.error("Failed to load rules:", e)
      alert("Failed to load rules: " + e)
    } finally {
      loading = false
    }
  }

  onMount(refresh)

  function openAddModal() {
    newRule = { name: "", port: 80, protocol: "TCP", action: "ALLOW", direction: "INBOUND" }
    editMode = false
    currentId = ""
    showModal = true
  }

  function openEditModal(rule) {
    newRule = {
      name: rule.name,
      port: rule.port,
      protocol: rule.protocol,
      action: rule.action,
      direction: rule.direction || "INBOUND"
    }
    currentId = rule.id
    editMode = true
    showModal = true
  }

  async function handleSave() {
    try {
      newRule.port = parseInt(newRule.port)
      
      if (editMode) {
        await invoke("update_firewall_rule", { id: currentId, input: newRule })
      } else {
        await invoke("add_firewall_rule", { input: newRule })
      }
      
      showModal = false
      refresh()
    } catch (e) {
      alert("Operation failed: " + e)
    }
  }

  async function deleteRule(id) {
    if(!confirm("Are you sure you want to delete this rule?")) return
    try {
      await invoke("delete_firewall_rule", { id })
      refresh()
    } catch (e) {
      alert("Failed to delete rule: " + e)
    }
  }
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>
          Firewall Management
          <a href="/admin/about/guides/firewall" class="btn btn-sm btn-outline-info ml-2" title="View Firewall Setup Guide">
            <i class="fas fa-question-circle"></i>
          </a>
        </h1>
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
          <button type="button" class="btn btn-primary btn-sm" on:click={openAddModal}>
            <i class="fas fa-plus"></i> Add Rule
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
              <th>Rule Name</th>
              <th>Direction</th>
              <th>Port</th>
              <th>Protocol</th>
              <th>Action</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#if loading}
                <tr><td colspan="6" class="text-center">Loading...</td></tr>
            {:else if rules.length === 0}
                <tr><td colspan="6" class="text-center">No firewall rules active.</td></tr>
            {:else}
              {#each rules as rule}
                <tr>
                  <td>{rule.name}</td>
                  <td>
                    <span class="badge" class:badge-info={rule.direction=='INBOUND'} class:badge-warning={rule.direction=='OUTBOUND'}>
                      {rule.direction}
                    </span>
                  </td>
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
                    <button class="btn btn-primary btn-xs mr-1" on:click={() => openEditModal(rule)}>
                        <i class="fas fa-edit"></i>
                    </button>
                    <button class="btn btn-danger btn-xs" on:click={() => deleteRule(rule.id)}>
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
        <h4 class="modal-title">{editMode ? 'Edit' : 'Add'} Firewall Rule</h4>
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
          <label>Direction</label>
          <select class="form-control" bind:value={newRule.direction}>
            <option value="INBOUND">INBOUND</option>
            <option value="OUTBOUND">OUTBOUND</option>
          </select>
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
            <option value="ANY">ANY</option>
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
        <button type="button" class="btn btn-primary" on:click={handleSave}>Save changes</button>
      </div>
    </div>
  </div>
</div>
{/if}
