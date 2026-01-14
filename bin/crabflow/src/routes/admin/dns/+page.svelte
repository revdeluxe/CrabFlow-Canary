<script>
  import { onMount, onDestroy } from 'svelte'
  import { api } from '$lib/tauri'

  let records = []
  let loading = true
  let showModal = false
  let activeTab = 'records' // 'records' or 'logs'
  
  // DNS Logs
  let dnsLogs = []
  let logsLoading = false
  let logSearchTerm = ''
  let filteredLogs = []
  
  // Homelab Templates
  let showTemplateModal = false
  let selectedTemplate = null
  const homelabTemplates = [
    { name: 'Plex Media Server', domain: 'plex.local', port: 32400, icon: 'fa-play-circle' },
    { name: 'Home Assistant', domain: 'hass.local', port: 8123, icon: 'fa-home' },
    { name: 'Nextcloud', domain: 'cloud.local', port: 443, icon: 'fa-cloud' },
    { name: 'Pi-hole', domain: 'pihole.local', port: 80, icon: 'fa-shield-alt' },
    { name: 'Jellyfin', domain: 'jellyfin.local', port: 8096, icon: 'fa-film' },
    { name: 'Portainer', domain: 'portainer.local', port: 9000, icon: 'fa-docker' },
    { name: 'Grafana', domain: 'grafana.local', port: 3000, icon: 'fa-chart-area' },
    { name: 'Nginx Proxy', domain: 'proxy.local', port: 80, icon: 'fa-server' }
  ]
  
  let newRecord = {
    name: "",
    rtype: "A",
    value: "",
    ttl: 3600
  }

  async function refresh() {
    try {
      records = await api.invokeCommand("list_records")
    } catch (e) {
      console.error("Failed to load records:", e)
    } finally {
      loading = false
    }
  }
  
  async function loadLogs() {
    logsLoading = true
    try {
      dnsLogs = await api.invokeCommand("get_query_logs", { limit: 500 })
      filterLogs()
    } catch (e) {
      console.error("Failed to load DNS logs:", e)
    } finally {
      logsLoading = false
    }
  }
  
  function filterLogs() {
    if (!logSearchTerm) {
      filteredLogs = dnsLogs
    } else {
      const term = logSearchTerm.toLowerCase()
      filteredLogs = dnsLogs.filter(l => 
        l.domain.toLowerCase().includes(term) || 
        l.client_ip.includes(term) ||
        l.status.toLowerCase().includes(term)
      )
    }
  }
  
  $: logSearchTerm, filterLogs()

  async function addRecord() {
    try {
      // Convert TTL to integer if it's a string
      newRecord.ttl = parseInt(newRecord.ttl)
      await api.invokeCommand("add_record", { input: newRecord })
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
      await api.invokeCommand("update_record", { 
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
      await api.invokeCommand("remove_record", { name, rtype })
      refresh()
    } catch (e) {
      alert("Failed to remove record: " + e)
    }
  }
  
  // Homelab template functions
  function openTemplateModal(template) {
    selectedTemplate = template
    newRecord = {
      name: template.domain,
      rtype: "A",
      value: "",
      ttl: 3600
    }
    showTemplateModal = true
  }
  
  async function applyTemplate() {
    try {
      newRecord.ttl = parseInt(newRecord.ttl)
      await api.invokeCommand("add_record", { input: newRecord })
      showTemplateModal = false
      selectedTemplate = null
      newRecord = { name: "", rtype: "A", value: "", ttl: 3600 }
      refresh()
      alert(`DNS record for ${selectedTemplate?.name} created! You can now configure ACL rules.`)
    } catch (e) {
      alert("Failed to create record: " + e)
    }
  }
  
  // Create ACL/Firewall quick link
  function getAclLink(record) {
    return `/admin/acl?domain=${encodeURIComponent(record.name)}&ip=${encodeURIComponent(record.value)}`
  }
  
  let refreshInterval

  onMount(() => {
    refresh()
    loadLogs()
    refreshInterval = setInterval(() => {
      if (activeTab === 'logs') loadLogs()
    }, 10000)
  })
  
  onDestroy(() => {
    if (refreshInterval) clearInterval(refreshInterval)
  })
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>
          DNS Management
          <a href="/admin/about/guides/dns" class="btn btn-sm btn-outline-info ml-2" title="View DNS Setup Guide">
            <i class="fas fa-question-circle"></i>
          </a>
        </h1>
      </div>
      <div class="col-sm-6">
        <div class="float-sm-right">
          <a href="/admin/acl" class="btn btn-warning mr-2">
            <i class="fas fa-shield-alt"></i> ACL Rules
          </a>
        </div>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    <!-- Tabs -->
    <ul class="nav nav-tabs mb-3">
      <li class="nav-item">
        <button class="nav-link {activeTab === 'records' ? 'active' : ''}" on:click={() => activeTab = 'records'}>
          <i class="fas fa-list mr-1"></i> DNS Records
        </button>
      </li>
      <li class="nav-item">
        <button class="nav-link {activeTab === 'logs' ? 'active' : ''}" on:click={() => { activeTab = 'logs'; loadLogs(); }}>
          <i class="fas fa-history mr-1"></i> Query Logs
        </button>
      </li>
      <li class="nav-item">
        <button class="nav-link {activeTab === 'homelab' ? 'active' : ''}" on:click={() => activeTab = 'homelab'}>
          <i class="fas fa-server mr-1"></i> Homelab Quick Setup
        </button>
      </li>
    </ul>
    
    <!-- Records Tab -->
    {#if activeTab === 'records'}
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
              <tr><td colspan="5" class="text-center text-muted">No DNS records found.</td></tr>
            {:else}
              {#each records as r}
                <tr>
                  <td>{r.name}</td>
                  <td><span class="badge badge-info">{r.rtype}</span></td>
                  <td><code>{r.value}</code></td>
                  <td>{r.ttl}</td>
                  <td>
                    <a href={getAclLink(r)} class="btn btn-warning btn-xs mr-1" title="Create ACL Rule">
                      <i class="fas fa-shield-alt"></i>
                    </a>
                    <button class="btn btn-primary btn-xs mr-1" on:click={() => openEditModal(r)} title="Edit">
                      <i class="fas fa-edit"></i>
                    </button>
                    <button class="btn btn-danger btn-xs" on:click={() => removeRecord(r.name, r.rtype)} title="Delete">
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
    {/if}
    
    <!-- Logs Tab -->
    {#if activeTab === 'logs'}
    <div class="card">
      <div class="card-header">
        <h3 class="card-title">DNS Query Logs</h3>
        <div class="card-tools">
          <div class="input-group input-group-sm" style="width: 250px;">
            <input type="text" class="form-control" placeholder="Search domain or IP..." bind:value={logSearchTerm}>
            <div class="input-group-append">
              <button class="btn btn-default" on:click={loadLogs}>
                <i class="fas fa-sync"></i>
              </button>
            </div>
          </div>
        </div>
      </div>
      <div class="card-body table-responsive p-0" style="max-height: 500px; overflow-y: auto;">
        {#if logsLoading}
          <div class="text-center p-3">
            <div class="spinner-border text-primary" role="status"></div>
          </div>
        {:else}
        <table class="table table-sm table-striped table-hover">
          <thead>
            <tr>
              <th>Time</th>
              <th>Client IP</th>
              <th>Domain</th>
              <th>Type</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredLogs as log}
              <tr class={log.status === 'Blocked' ? 'table-danger' : (log.status === 'Portal' ? 'table-info' : '')}>
                <td><small>{new Date(log.timestamp * 1000).toLocaleTimeString()}</small></td>
                <td><code>{log.client_ip}</code></td>
                <td class="text-truncate" style="max-width: 250px;" title={log.domain}>{log.domain}</td>
                <td><span class="badge badge-secondary">{log.query_type}</span></td>
                <td>
                  {#if log.status === 'Blocked'}
                    <span class="badge-status blocked">Blocked</span>
                  {:else if log.status === 'Redirected'}
                    <span class="badge-status redirected">Redirected</span>
                  {:else if log.status === 'Portal'}
                    <span class="badge-status portal">Portal</span>
                  {:else}
                    <span class="badge-status allowed">Allowed</span>
                  {/if}
                </td>
                <td>
                  <button class="btn btn-xs btn-outline-primary" 
                    on:click={() => { newRecord.name = log.domain; newRecord.rtype = 'A'; editMode = false; showModal = true; }}
                    title="Create DNS Record">
                    <i class="fas fa-plus"></i>
                  </button>
                  <a href="/admin/acl?domain={encodeURIComponent(log.domain)}" 
                    class="btn btn-xs btn-outline-warning" title="Create ACL Rule">
                    <i class="fas fa-shield-alt"></i>
                  </a>
                </td>
              </tr>
            {:else}
              <tr><td colspan="6" class="text-center text-muted">No logs found</td></tr>
            {/each}
          </tbody>
        </table>
        {/if}
      </div>
      <div class="card-footer text-muted">
        <small>Showing {filteredLogs.length} of {dnsLogs.length} queries. Auto-refreshes every 10 seconds.</small>
      </div>
    </div>
    {/if}
    
    <!-- Homelab Tab -->
    {#if activeTab === 'homelab'}
    <div class="card">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-magic mr-2"></i>Homelab Quick Setup</h3>
      </div>
      <div class="card-body">
        <p class="text-muted mb-3">
          Quickly set up DNS records for popular homelab services. Click a template to create a DNS record, 
          then use the ACL link to configure firewall rules for the service.
        </p>
        <div class="row">
          {#each homelabTemplates as template}
            <div class="col-lg-3 col-md-4 col-sm-6 mb-3">
              <div class="card card-outline card-primary h-100">
                <div class="card-body text-center p-3">
                  <i class="fas {template.icon} fa-2x mb-2 text-primary"></i>
                  <h6 class="mb-1">{template.name}</h6>
                  <small class="text-muted d-block text-truncate">{template.domain}</small>
                  <small class="text-muted">Port: {template.port}</small>
                </div>
                <div class="card-footer p-2">
                  <button class="btn btn-primary btn-sm btn-block" on:click={() => openTemplateModal(template)}>
                    <i class="fas fa-plus"></i> Setup
                  </button>
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
    
    <!-- ACL Integration Info -->
    <div class="callout callout-info">
      <h5><i class="fas fa-info-circle"></i> ACL Integration</h5>
      <p>
        After creating DNS records for your homelab services, visit the 
        <a href="/admin/acl"><strong>ACL & Permissions</strong></a> page to:
      </p>
      <ul class="mb-0">
        <li>Set up captive portal authentication for guest access</li>
        <li>Configure port forwarding rules</li>
        <li>Create firewall rules to control access to services</li>
        <li>Set bandwidth limits per user group</li>
      </ul>
    </div>
    {/if}
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
        <div>
          {#if !editMode}
            <a href="/admin/acl?domain={encodeURIComponent(newRecord.name)}" class="btn btn-warning mr-2" target="_blank">
              <i class="fas fa-shield-alt"></i> ACL Rules
            </a>
          {/if}
          <button type="button" class="btn btn-primary" on:click={handleSave}>Save changes</button>
        </div>
      </div>
    </div>
  </div>
</div>
{/if}

{#if showTemplateModal && selectedTemplate}
<div class="modal fade show" style="display: block; background: rgba(0,0,0,0.5)">
  <div class="modal-dialog">
    <div class="modal-content">
      <div class="modal-header bg-primary">
        <h4 class="modal-title">
          <i class="fas {selectedTemplate.icon} mr-2"></i>
          Setup {selectedTemplate.name}
        </h4>
        <button type="button" class="close" on:click={() => { showTemplateModal = false; selectedTemplate = null; }}>
          <span>&times;</span>
        </button>
      </div>
      <div class="modal-body">
        <div class="alert alert-info">
          <i class="fas fa-info-circle"></i> 
          Enter the IP address of your <strong>{selectedTemplate.name}</strong> server to create a DNS record.
        </div>
        <div class="form-group">
          <label>Domain Name</label>
          <input type="text" class="form-control" bind:value={newRecord.name}>
          <small class="text-muted">Suggested: {selectedTemplate.domain}</small>
        </div>
        <div class="form-group">
          <label>Server IP Address</label>
          <input type="text" class="form-control" bind:value={newRecord.value} placeholder="192.168.1.x">
        </div>
        <div class="form-group">
          <label>Default Port</label>
          <input type="text" class="form-control" value={selectedTemplate.port} disabled>
          <small class="text-muted">Configure port forwarding in ACL if needed</small>
        </div>
      </div>
      <div class="modal-footer justify-content-between">
        <button type="button" class="btn btn-default" on:click={() => { showTemplateModal = false; selectedTemplate = null; }}>Cancel</button>
        <div>
          <button type="button" class="btn btn-primary" on:click={applyTemplate}>
            <i class="fas fa-check"></i> Create DNS Record
          </button>
        </div>
      </div>
    </div>
  </div>
</div>
{/if}
