<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'
  import { api } from '$lib/tauri'

  let users = []
  let availableGroups = []
  let loading = true
  let error = null
  
  // Edit State
  let editingUser = null
  let editForm = {
    username: "",
    nickname: "",
    email: "",
    groups: [],
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

  async function loadData() {
    loading = true
    try {
      const [u, g] = await Promise.all([
        invoke("list_users"),
        api.listGroups()
      ])
      users = u
      availableGroups = g
    } catch (e) {
      error = "Failed to load data: " + e
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
      loadData()
    } catch (e) {
      alert("Failed to create user: " + e)
    }
  }

  function startEdit(user) {
    editingUser = user
    editForm = {
      username: user.username,
      nickname: user.nickname || "",
      email: user.email || "",
      groups: [...user.groups],
      password: "", // Don't show existing password
      is_active: user.is_active,
      is_approved: user.is_approved
    }
    showEdit = true
  }

  function cancelEdit() {
    editingUser = null
    showEdit = false
    editForm = { username: "", nickname: "", email: "", groups: [], password: "", is_active: false, is_approved: false }
  }

  function toggleGroup(groupName) {
    if (editForm.groups.includes(groupName)) {
      editForm.groups = editForm.groups.filter(g => g !== groupName)
    } else {
      editForm.groups = [...editForm.groups, groupName]
    }
  }

  async function saveUser() {
    if (!editingUser) return

    try {
      // Update Profile (Nickname, Email)
      await invoke("update_user_profile", {
        username: editingUser.username,
        nickname: editForm.nickname || null,
        email: editForm.email || null
      })

      // Update Status
      if (editingUser.is_active !== editForm.is_active || editingUser.is_approved !== editForm.is_approved) {
        await invoke("update_user_status", { 
          username: editingUser.username, 
          active: editForm.is_active, 
          approved: editForm.is_approved 
        })
      }

      // Update Groups
      await invoke("update_user_groups", { username: editingUser.username, groups: editForm.groups })

      // Update Password if provided
      if (editForm.password) {
        await invoke("change_password", { username: editingUser.username, newPassword: editForm.password })
      }

      alert("User updated successfully")
      cancelEdit()
      loadData()
    } catch (e) {
      alert("Failed to update user: " + e)
    }
  }

  async function getTrafficHistory(user) {
    if (!user) return [];
    // If user has no login history, we can't match IPs easily unless we have a static IP field.
    // For now, we rely on login history.
    const userIps = user.login_history ? [...new Set(user.login_history.map(l => l.ip))] : [];
    
    if (userIps.length === 0) return [];

    try {
      const logs = await invoke("get_query_logs", { limit: 500 });
      return logs.filter(log => userIps.includes(log.client_ip));
    } catch (e) {
      console.error("Failed to fetch traffic history:", e);
      return [];
    }
  }

  async function blockDomain(domain) {
    if (!confirm(`Block domain ${domain}?`)) return;
    try {
      await invoke("block_domain", { domain });
      editingUser = {...editingUser}; // Trigger refresh
    } catch (e) {
      alert("Failed to block: " + e);
    }
  }

  async function unblockDomain(domain) {
    if (!confirm(`Unblock domain ${domain}?`)) return;
    try {
      await invoke("unblock_domain", { domain });
      editingUser = {...editingUser}; // Trigger refresh
    } catch (e) {
      alert("Failed to unblock: " + e);
    }
  }

  async function deleteUser(username) {
    if (username === 'admin') {
      alert("Cannot delete admin user")
      return
    }
    if (!confirm(`Are you sure you want to delete user ${username}?`)) return
    try {
      await invoke("remove_user", { username })
      loadData()
      if (editingUser && editingUser.username === username) {
        cancelEdit()
      }
    } catch (e) {
      alert("Failed to delete user: " + e)
    }
  }

  onMount(loadData)
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
        <button class="btn btn-secondary float-sm-right" on:click={loadData}>
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
    <div class="card card-primary card-tabs">
      <div class="card-header p-0 pt-1">
        <ul class="nav nav-tabs" id="custom-tabs-one-tab" role="tablist">
          <li class="nav-item">
            <a class="nav-link active" id="custom-tabs-one-settings-tab" data-toggle="pill" href="#custom-tabs-one-settings" role="tab" aria-controls="custom-tabs-one-settings" aria-selected="true">Settings</a>
          </li>
          <li class="nav-item">
            <a class="nav-link" id="custom-tabs-one-history-tab" data-toggle="pill" href="#custom-tabs-one-history" role="tab" aria-controls="custom-tabs-one-history" aria-selected="false">Login History</a>
          </li>
          <li class="nav-item">
            <a class="nav-link" id="custom-tabs-one-traffic-tab" data-toggle="pill" href="#custom-tabs-one-traffic" role="tab" aria-controls="custom-tabs-one-traffic" aria-selected="false">Traffic History</a>
          </li>
        </ul>
      </div>
      <div class="card-body">
        <div class="tab-content" id="custom-tabs-one-tabContent">
          
          <!-- Settings Tab -->
          <div class="tab-pane fade show active" id="custom-tabs-one-settings" role="tabpanel" aria-labelledby="custom-tabs-one-settings-tab">
            <div class="row">
              <div class="col-md-6">
                <div class="form-group">
                  <label>Nickname</label>
                  <input type="text" class="form-control" bind:value={editForm.nickname} placeholder="Nickname">
                </div>
                <div class="form-group">
                  <label>Email</label>
                  <input type="email" class="form-control" bind:value={editForm.email} placeholder="Email">
                </div>
                <div class="form-group">
                  <label>Groups</label>
                  <div class="row">
                    {#each availableGroups as group}
                      <div class="col-md-6">
                        <div class="custom-control custom-checkbox">
                          <input type="checkbox" class="custom-control-input" id="group_{group.name}" 
                                 checked={editForm.groups.includes(group.name)} 
                                 on:change={() => toggleGroup(group.name)}
                                 disabled={editingUser.username === 'admin' && group.name === 'admin'}>
                          <label class="custom-control-label" for="group_{group.name}">{group.name}</label>
                        </div>
                      </div>
                    {/each}
                  </div>
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
            <div class="mt-3">
              <button class="btn btn-primary" on:click={saveUser}>Save Changes</button>
              <button class="btn btn-default float-right" on:click={cancelEdit}>Cancel</button>
            </div>
          </div>

          <!-- Login History Tab -->
          <div class="tab-pane fade" id="custom-tabs-one-history" role="tabpanel" aria-labelledby="custom-tabs-one-history-tab">
            <table class="table table-hover text-nowrap">
              <thead>
                <tr>
                  <th>Time</th>
                  <th>IP</th>
                  <th>MAC</th>
                  <th>Device</th>
                </tr>
              </thead>
              <tbody>
                {#if editingUser.login_history && editingUser.login_history.length > 0}
                  {#each editingUser.login_history as record}
                    <tr>
                      <td>{new Date(record.timestamp).toLocaleString()}</td>
                      <td>{record.ip}</td>
                      <td>{record.mac}</td>
                      <td>{record.device_name || '-'}</td>
                    </tr>
                  {/each}
                {:else}
                  <tr><td colspan="4" class="text-center">No login history found.</td></tr>
                {/if}
              </tbody>
            </table>
          </div>

          <!-- Traffic History Tab -->
          <div class="tab-pane fade" id="custom-tabs-one-traffic" role="tabpanel" aria-labelledby="custom-tabs-one-traffic-tab">
            <div class="alert alert-info">
              <i class="icon fas fa-info"></i> Showing recent DNS queries from IPs associated with this user.
            </div>
            <table class="table table-sm table-hover">
              <thead>
                <tr>
                  <th>Time</th>
                  <th>Domain</th>
                  <th>Type</th>
                  <th>Status</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                {#await getTrafficHistory(editingUser) then logs}
                  {#each logs as log}
                    <tr>
                      <td>{new Date(log.timestamp * 1000).toLocaleTimeString()}</td>
                      <td>{log.domain}</td>
                      <td>{log.query_type}</td>
                      <td>
                        <span class="badge {log.status === 'Blocked' ? 'badge-danger' : 'badge-success'}">
                          {log.status}
                        </span>
                      </td>
                      <td>
                        {#if log.status !== 'Blocked'}
                          <button class="btn btn-xs btn-danger" on:click={() => blockDomain(log.domain)}>
                            Block
                          </button>
                        {:else}
                          <button class="btn btn-xs btn-success" on:click={() => unblockDomain(log.domain)}>
                            Unblock
                          </button>
                        {/if}
                      </td>
                    </tr>
                  {/each}
                  {#if logs.length === 0}
                    <tr><td colspan="5" class="text-center">No recent traffic found.</td></tr>
                  {/if}
                {:catch error}
                  <tr><td colspan="5" class="text-center text-danger">{error}</td></tr>
                {/await}
              </tbody>
            </table>
          </div>

        </div>
      </div>
      <!-- Footer removed as buttons are now in Settings tab -->
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
                    {#if user.username !== 'admin'}
                      <button class="btn btn-danger btn-xs" on:click={() => deleteUser(user.username)}>
                        <i class="fas fa-trash"></i> Delete
                      </button>
                    {:else}
                      <button class="btn btn-danger btn-xs disabled" disabled>
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
