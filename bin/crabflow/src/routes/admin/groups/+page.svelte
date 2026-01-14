<script>
  import { api } from '$lib/tauri'
  import { onMount } from 'svelte'

  let groups = []
  let permissions = []
  let loading = true
  let error = null

  // Edit/Add State
  let showModal = false
  let isEditing = false
  let form = {
    name: "",
    description: "",
    permissions: []
  }

  async function loadData() {
    loading = true
    try {
      const [g, p] = await Promise.all([
        api.listGroups(),
        api.listPermissions()
      ])
      groups = g
      permissions = p
    } catch (e) {
      error = "Failed to load data: " + e
    } finally {
      loading = false
    }
  }

  function startAdd() {
    isEditing = false
    form = { name: "", description: "", permissions: [] }
    showModal = true
  }

  function startEdit(group) {
    isEditing = true
    form = {
      name: group.name,
      description: group.description,
      permissions: [...group.permissions]
    }
    showModal = true
  }

  function closeModal() {
    showModal = false
  }

  function togglePermission(perm) {
    if (form.permissions.includes(perm)) {
      form.permissions = form.permissions.filter(p => p !== perm)
    } else {
      form.permissions = [...form.permissions, perm]
    }
  }

  async function saveGroup() {
    if (!form.name) return alert("Name is required")
    
    try {
      if (isEditing) {
        await api.updateGroup(form.name, form.description, form.permissions)
      } else {
        await api.addGroup(form.name, form.description, form.permissions)
      }
      closeModal()
      loadData()
    } catch (e) {
      alert("Failed to save group: " + e)
    }
  }

  async function deleteGroup(name) {
    if (!confirm(`Are you sure you want to delete group ${name}?`)) return
    try {
      await api.deleteGroup(name)
      loadData()
    } catch (e) {
      alert("Failed to delete group: " + e)
    }
  }

  onMount(loadData)
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>
          Group Management
          <a href="/admin/about/guides/users-groups" class="btn btn-sm btn-outline-info ml-2" title="View Users & Groups Guide">
            <i class="fas fa-question-circle"></i>
          </a>
        </h1>
      </div>
      <div class="col-sm-6">
        <button class="btn btn-primary float-sm-right" on:click={startAdd}>
          <i class="fas fa-plus"></i> Add Group
        </button>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    
    {#if showModal}
    <div class="modal fade show" style="display: block; background-color: rgba(0,0,0,0.5); z-index: 1050;" tabindex="-1" role="dialog" aria-modal="true">
      <div class="modal-dialog modal-lg">
        <div class="modal-content">
          <div class="modal-header">
            <h4 class="modal-title">{isEditing ? 'Edit Group' : 'Add Group'}</h4>
            <button type="button" class="close" on:click={closeModal} aria-label="Close">
              <span aria-hidden="true">&times;</span>
            </button>
          </div>
          <div class="modal-body">
            <div class="form-group">
              <label>Group Name</label>
              <input type="text" class="form-control" bind:value={form.name} readonly={isEditing} placeholder="Enter group name">
            </div>
            <div class="form-group">
              <label>Description</label>
              <input type="text" class="form-control" bind:value={form.description} placeholder="Description">
            </div>
            <div class="form-group">
              <label>Permissions</label>
              <div class="row">
                {#each permissions as perm}
                  <div class="col-md-4">
                    <div class="custom-control custom-checkbox">
                      <input type="checkbox" class="custom-control-input" id="perm_{perm}" 
                             checked={form.permissions.includes(perm)} 
                             on:change={() => togglePermission(perm)}>
                      <label class="custom-control-label" for="perm_{perm}">{perm}</label>
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          </div>
          <div class="modal-footer justify-content-between">
            <button type="button" class="btn btn-default" on:click={closeModal}>Close</button>
            <button type="button" class="btn btn-primary" on:click={saveGroup}>Save Changes</button>
          </div>
        </div>
      </div>
    </div>
    {/if}

    <div class="card">
      <div class="card-header">
        <h3 class="card-title">User Groups</h3>
      </div>
      <div class="card-body table-responsive p-0">
        <table class="table table-hover text-nowrap">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
              <th>Permissions</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#if loading}
              <tr><td colspan="4" class="text-center">Loading...</td></tr>
            {:else if error}
              <tr><td colspan="4" class="text-center text-danger">{error}</td></tr>
            {:else}
              {#each groups as group}
                <tr>
                  <td>{group.name}</td>
                  <td>{group.description}</td>
                  <td>
                    {#each group.permissions as perm}
                      <span class="perm-chip" title={perm}>{perm}</span>
                    {/each}
                  </td>
                  <td>
                    <button class="btn btn-info btn-xs" on:click={() => startEdit(group)}>
                      <i class="fas fa-edit"></i> Edit
                    </button>
                    {#if group.name !== 'admin'}
                      <button class="btn btn-danger btn-xs" on:click={() => deleteGroup(group.name)}>
                        <i class="fas fa-trash"></i> Delete
                      </button>
                    {/if}
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
