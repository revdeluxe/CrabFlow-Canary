<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let records = []
  let loading = true
  let showModal = false
  
  let newRecord = {
    name: "",
    rtype: "A",
    value: "",
    ttl: 3600
  }

  async function refresh() {
    try {
      records = await invoke("list_records")
    } catch (e) {
      console.error("Failed to load records:", e)
    } finally {
      loading = false
    }
  }

  async function addRecord() {
    try {
      // Convert TTL to integer if it's a string
      newRecord.ttl = parseInt(newRecord.ttl)
      await invoke("add_record", { input: newRecord })
      newRecord = { name: "", rtype: "A", value: "", ttl: 3600 }
      showModal = false
      refresh()
      alert("Record added successfully")
    } catch (e) {
      alert("Failed to add record: " + e)
    }
  }

  // --- Edit Logic ---
  let editMode = false
  let oldRecordName = ""
  let oldRecordType = ""

  function openEditModal(record) {
    editMode = true
    oldRecordName = record.name
    oldRecordType = record.rtype
    
    // Copy into newRecord for binding
    newRecord = {
      name: record.name,
      rtype: record.rtype,
      value: record.value,
      ttl: record.ttl
    }
    showModal = true
  }

  async function updateRecord() {
    try {
      newRecord.ttl = parseInt(newRecord.ttl)
      await invoke("update_record", { 
        oldName: oldRecordName, 
        oldRtype: oldRecordType, 
        input: newRecord 
      })
      
      newRecord = { name: "", rtype: "A", value: "", ttl: 3600 }
      showModal = false
      editMode = false
      refresh()
      alert("Record updated successfully")
    } catch(e) {
      alert("Failed to update record: " + e)
    }
  }

  function handleSave() {
    if (editMode) {
      updateRecord()
    } else {
      addRecord()
    }
  }

  function closeModal() {
      showModal = false
      editMode = false
      newRecord = { name: "", rtype: "A", value: "", ttl: 3600 }
  }

  async function removeRecord(name, rtype) {
    if (!confirm(`Are you sure you want to remove ${rtype} record for ${name}?`)) return
    try {
      await invoke("remove_record", { name, rtype })
      refresh()
    } catch (e) {
      alert("Failed to remove record: " + e)
    }
  }

  onMount(refresh)
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>DNS Management</h1>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    <div class="card">
      <div class="card-header">
        <h3 class="card-title">DNS Records</h3>
        <div class="card-tools">
          <button type="button" class="btn btn-primary btn-sm" on:click={() => { editMode = false; showModal = true; }}>
            <i class="fas fa-plus"></i> Add Record
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
              <th>Name</th>
              <th>Type</th>
              <th>Value</th>
              <th>TTL</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#if loading}
              <tr><td colspan="5" class="text-center">Loading...</td></tr>
            {:else if records.length === 0}
              <tr><td colspan="5" class="text-center">No DNS records found.</td></tr>
            {:else}
              {#each records as r}
                <tr>
                  <td>{r.name}</td>
                  <td><span class="badge badge-info">{r.rtype}</span></td>
                  <td>{r.value}</td>
                  <td>{r.ttl}</td>
                  <td>
                    <button class="btn btn-primary btn-xs mr-1" on:click={() => openEditModal(r)}>
                      <i class="fas fa-edit"></i>
                    </button>
                    <button class="btn btn-danger btn-xs" on:click={() => removeRecord(r.name, r.rtype)}>
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
        <h4 class="modal-title">{editMode ? 'Edit' : 'Add'} DNS Record</h4>
        <button type="button" class="close" on:click={closeModal}>
          <span>&times;</span>
        </button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label>Domain Name</label>
          <input type="text" class="form-control" bind:value={newRecord.name} placeholder="example.local">
        </div>
        <div class="form-group">
          <label>Record Type</label>
          <select class="form-control" bind:value={newRecord.rtype}>
            <option value="A">A (IPv4)</option>
            <option value="AAAA">AAAA (IPv6)</option>
            <option value="CNAME">CNAME (Alias)</option>
            <option value="TXT">TXT (Text)</option>
          </select>
        </div>
        <div class="form-group">
          <label>Value</label>
          <input type="text" class="form-control" bind:value={newRecord.value} placeholder="192.168.1.x or domain">
        </div>
        <div class="form-group">
          <label>TTL (Seconds)</label>
          <input type="number" class="form-control" bind:value={newRecord.ttl}>
        </div>
      </div>
      <div class="modal-footer justify-content-between">
        <button type="button" class="btn btn-default" on:click={closeModal}>Close</button>
        <button type="button" class="btn btn-primary" on:click={handleSave}>Save changes</button>
      </div>
    </div>
  </div>
</div>
{/if}
