<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let users = []
  let loading = true
  let error = null
  
  // Edit State
  let editingUser = null
  let editForm = {
    username: "",
    groups: "",
    password: "",
    is_active: false,
    is_approved: false
  }
  let showEdit = false

  // Add State
  let showAdd = false
  let addForm = {
    username: "",
    password: "",
    confirmPassword: ""
  }

  async function loadUsers() {
    loading = true
    try {
      users = await invoke("list_users")
    } catch (e) {
      error = "Failed to load users: " + e
    } finally {
      loading = false
    }
  }

  function startAdd() {
    addForm = { username: "", password: "", confirmPassword: "" }
    showAdd = true
  }

  function cancelAdd() {
    showAdd = false
  }

  async function createUser() {
    if (addForm.password !== addForm.confirmPassword) {
      alert("Passwords do not match")
      return
    }
    if (!addForm.username || !addForm.password) {
      alert("Username and password are required")
      return
    }

    try {
      await invoke("register_user", { username: addForm.username, password: addForm.password })
      alert("User created successfully")
      cancelAdd()
      loadUsers()
    } catch (e) {
      alert("Failed to create user: " + e)
    }
  }

  function startEdit(user) {
    editingUser = user
    editForm = {
      username: user.username,
      groups: user.groups.join(", "),
      password: "", // Don't show existing password
      is_active: user.is_active,
      is_approved: user.is_approved
    }
    showEdit = true
  }

  function cancelEdit() {
    editingUser = null
    showEdit = false
    editForm = { username: "", groups: "", password: "", is_active: false, is_approved: false }
  }

  async function saveUser() {
    if (!editingUser) return

    try {
      // Update Status
      if (editingUser.is_active !== editForm.is_active || editingUser.is_approved !== editForm.is_approved) {
        await invoke("update_user_status", { 
          username: editingUser.username, 
          active: editForm.is_active, 
          approved: editForm.is_approved 
        })
      }

      // Update Groups
      const groups = editForm.groups.split(",").map(g => g.trim()).filter(g => g)
      await invoke("update_user_groups", { username: editingUser.username, groups })

      // Update Password if provided
      if (editForm.password) {
        await invoke("change_password", { username: editingUser.username, newPassword: editForm.password })
      }

      alert("User updated successfully")
      cancelEdit()
      loadUsers()
    } catch (e) {
      alert("Failed to update user: " + e)
    }
  }

  async function deleteUser(username) {
    if (!confirm(`Are you sure you want to delete user ${username}?`)) return
    try {
      await invoke("remove_user", { username })
      loadUsers()
      if (editingUser && editingUser.username === username) {
        cancelEdit()
      }
    } catch (e) {
      alert("Failed to delete user: " + e)
    }
  }

  onMount(loadUsers)
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>User Management</h1>
      </div>
      <div class="col-sm-6">
        <button class="btn btn-primary float-sm-right ml-2" on:click={startAdd}>
          <i class="fas fa-plus"></i> Add User
        </button>
        <button class="btn btn-secondary float-sm-right" on:click={loadUsers}>
          <i class="fas fa-sync"></i> Refresh
        </button>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    
    {#if showAdd}
    <div class="modal fade show" style="display: block; background-color: rgba(0,0,0,0.5); z-index: 1050;" tabindex="-1" role="dialog" aria-modal="true">
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
            <h4 class="modal-title">Add New User</h4>
            <button type="button" class="close" on:click={cancelAdd} aria-label="Close">
              <span aria-hidden="true">&times;</span>
            </button>
          </div>
          <div class="modal-body">
            <div class="form-group">
              <label>Username</label>
              <input type="text" class="form-control" bind:value={addForm.username} placeholder="Enter username">
            </div>
            <div class="form-group">
              <label>Password</label>
              <input type="password" class="form-control" bind:value={addForm.password} placeholder="Password">
            </div>
            <div class="form-group">
              <label>Confirm Password</label>
              <input type="password" class="form-control" bind:value={addForm.confirmPassword} placeholder="Retype password">
            </div>
          </div>
          <div class="modal-footer justify-content-between">
            <button type="button" class="btn btn-default" on:click={cancelAdd}>Close</button>
            <button type="button" class="btn btn-primary" on:click={createUser}>Create User</button>
          </div>
        </div>
      </div>
    </div>
    {/if}

    <!-- Edit User Ribbon (Collapsible) -->
    {#if showEdit}
    <div class="card card-primary">
      <div class="card-header">
        <h3 class="card-title">Edit User: {editingUser.username}</h3>
        <div class="card-tools">
          <button type="button" class="btn btn-tool" on:click={cancelEdit}>
            <i class="fas fa-times"></i>
          </button>
        </div>
      </div>
      <div class="card-body">
        <div class="row">
          <div class="col-md-6">
            <div class="form-group">
              <label>Groups (comma separated)</label>
              <input type="text" class="form-control" bind:value={editForm.groups}>
            </div>
            <div class="form-group">
              <label>New Password (leave blank to keep current)</label>
              <input type="password" class="form-control" bind:value={editForm.password}>
            </div>
          </div>
          <div class="col-md-6">
            <div class="form-group">
              <label>Status</label>
              <div class="custom-control custom-switch">
                <input type="checkbox" class="custom-control-input" id="activeSwitch" bind:checked={editForm.is_active}>
                <label class="custom-control-label" for="activeSwitch">Active (Can Login)</label>
              </div>
              <div class="custom-control custom-switch mt-2">
                <input type="checkbox" class="custom-control-input" id="approvedSwitch" bind:checked={editForm.is_approved}>
                <label class="custom-control-label" for="approvedSwitch">Approved</label>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="card-footer">
        <button class="btn btn-primary" on:click={saveUser}>Save Changes</button>
        <button class="btn btn-default float-right" on:click={cancelEdit}>Cancel</button>
      </div>
    </div>
    {/if}

    <!-- Users Table -->
    <div class="card">
      <div class="card-header">
        <h3 class="card-title">Registered Users</h3>
      </div>
      <div class="card-body table-responsive p-0">
        <table class="table table-hover text-nowrap">
          <thead>
            <tr>
              <th>Username</th>
              <th>Role</th>
              <th>Groups</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#if loading}
              <tr><td colspan="5" class="text-center">Loading...</td></tr>
            {:else if error}
              <tr><td colspan="5" class="text-center text-danger">{error}</td></tr>
            {:else}
              {#each users as user}
                <tr class="{editingUser && editingUser.username === user.username ? 'bg-light' : ''}">
                  <td>{user.username}</td>
                  <td>{user.role}</td>
                  <td>{user.groups.join(", ")}</td>
                  <td>
                    {#if !user.is_approved}
                      <span class="badge badge-warning">Pending</span>
                    {:else if user.is_active}
                      <span class="badge badge-success">Active</span>
                    {:else}
                      <span class="badge badge-danger">Disabled</span>
                    {/if}
                  </td>
                  <td>
                    <button class="btn btn-info btn-xs" on:click={() => startEdit(user)}>
                      <i class="fas fa-edit"></i> Edit
                    </button>
                    <button class="btn btn-danger btn-xs" on:click={() => deleteUser(user.username)}>
                      <i class="fas fa-trash"></i> Delete
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
