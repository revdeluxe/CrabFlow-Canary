<script>
  import { onMount } from 'svelte'
  import { api } from '$lib/tauri'

  let activeTab = 'captive-portal'
  let loading = true
  let saving = false

  // Captive Portal Settings
  let captivePortalEnabled = false
  let portalRedirectUrl = 'http://portal.crabflow.local'
  let authenticationRequired = true
  let sessionTimeout = 3600
  let allowedBeforeAuth = []
  let detectionDomains = []
  let newAllowedDomain = ''
  let newDetectionDomain = ''
  
  // Route Rules
  let routeRules = []
  let showRouteModal = false
  let editingRoute = null
  let newRoute = {
    name: '',
    source: '*',
    destination: '*',
    port: '*',
    protocol: 'any',
    action: 'allow',
    priority: 100
  }

  // Forwarding Settings
  let forwardingEnabled = false
  let natEnabled = true
  let uplinkInterface = ''
  let downlinkInterface = ''
  let availableInterfaces = []
  let forwardingRules = []
  let showForwardModal = false
  let newForwardRule = {
    name: '',
    sourceNetwork: '192.168.137.0/24',
    targetInterface: '',
    masquerade: true
  }

  // Dataflow / Bandwidth Settings
  let bandwidthLimits = {
    globalUpload: 0,
    globalDownload: 0,
    perClientUpload: 0,
    perClientDownload: 0,
    enabled: false
  }
  let qosEnabled = false
  let qosRules = []
  let groupLimits = []
  let showGroupLimitModal = false
  let editingGroupLimit = null
  let newGroupLimit = {
    group_name: '',
    upload_limit: 0,
    download_limit: 0,
    priority: 5,
    enabled: true
  }
  let availableGroups = []

  async function loadConfig() {
    loading = true
    try {
      const config = await api.invokeCommand('get_acl_config')
      
      // Captive Portal
      captivePortalEnabled = config.captive_portal?.enabled ?? false
      portalRedirectUrl = config.captive_portal?.redirect_url ?? 'http://portal.crabflow.local'
      authenticationRequired = config.captive_portal?.auth_required ?? true
      sessionTimeout = config.captive_portal?.session_timeout ?? 3600
      allowedBeforeAuth = config.captive_portal?.allowed_domains ?? []
      detectionDomains = config.captive_portal?.detection_domains ?? [
        'www.msftconnecttest.com',
        'captive.apple.com',
        'connectivitycheck.gstatic.com',
        'clients3.google.com',
        'www.apple.com'
      ]

      // Routes
      routeRules = config.routes ?? []

      // Forwarding
      forwardingEnabled = config.forwarding?.enabled ?? false
      natEnabled = config.forwarding?.nat_enabled ?? true
      uplinkInterface = config.forwarding?.uplink ?? ''
      downlinkInterface = config.forwarding?.downlink ?? ''
      forwardingRules = config.forwarding?.rules ?? []

      // Dataflow
      bandwidthLimits = config.dataflow?.bandwidth ?? bandwidthLimits
      qosEnabled = config.dataflow?.qos_enabled ?? false
      qosRules = config.dataflow?.qos_rules ?? []
      groupLimits = config.dataflow?.group_limits ?? []

    } catch (e) {
      console.error('Failed to load ACL config:', e)
    }
    
    try {
      availableInterfaces = await api.invokeCommand('list_interfaces')
    } catch (e) {
      console.error('Failed to load interfaces:', e)
    }
    
    try {
      availableGroups = await api.invokeCommand('list_groups')
    } catch (e) {
      console.error('Failed to load groups:', e)
      availableGroups = []
    }
    
    loading = false
  }

  async function saveConfig() {
    saving = true
    try {
      await api.invokeCommand('save_acl_config', {
        config: {
          captive_portal: {
            enabled: captivePortalEnabled,
            redirect_url: portalRedirectUrl,
            auth_required: authenticationRequired,
            session_timeout: sessionTimeout,
            allowed_domains: allowedBeforeAuth,
            detection_domains: detectionDomains
          },
          routes: routeRules,
          forwarding: {
            enabled: forwardingEnabled,
            nat_enabled: natEnabled,
            uplink: uplinkInterface,
            downlink: downlinkInterface,
            rules: forwardingRules
          },
          dataflow: {
            bandwidth: bandwidthLimits,
            qos_enabled: qosEnabled,
            qos_rules: qosRules,
            group_limits: groupLimits
          }
        }
      })
      alert('Configuration saved successfully!')
    } catch (e) {
      console.error('Failed to save config:', e)
      alert('Failed to save configuration: ' + e)
    }
    saving = false
  }

  onMount(loadConfig)

  // Captive Portal helpers
  function addAllowedDomain() {
    if (newAllowedDomain && !allowedBeforeAuth.includes(newAllowedDomain)) {
      allowedBeforeAuth = [...allowedBeforeAuth, newAllowedDomain]
      newAllowedDomain = ''
    }
  }

  function removeAllowedDomain(domain) {
    allowedBeforeAuth = allowedBeforeAuth.filter(d => d !== domain)
  }

  function addDetectionDomain() {
    if (newDetectionDomain && !detectionDomains.includes(newDetectionDomain)) {
      detectionDomains = [...detectionDomains, newDetectionDomain]
      newDetectionDomain = ''
    }
  }

  function removeDetectionDomain(domain) {
    detectionDomains = detectionDomains.filter(d => d !== domain)
  }

  // Route helpers
  function openAddRoute() {
    newRoute = {
      name: '',
      source: '*',
      destination: '*',
      port: '*',
      protocol: 'any',
      action: 'allow',
      priority: 100
    }
    editingRoute = null
    showRouteModal = true
  }

  function openEditRoute(route, index) {
    newRoute = { ...route }
    editingRoute = index
    showRouteModal = true
  }

  function saveRoute() {
    if (editingRoute !== null) {
      routeRules[editingRoute] = { ...newRoute }
      routeRules = [...routeRules]
    } else {
      routeRules = [...routeRules, { ...newRoute }]
    }
    showRouteModal = false
  }

  function deleteRoute(index) {
    if (confirm('Delete this route rule?')) {
      routeRules = routeRules.filter((_, i) => i !== index)
    }
  }

  // Forwarding helpers
  function openAddForward() {
    newForwardRule = {
      name: '',
      sourceNetwork: '192.168.137.0/24',
      targetInterface: uplinkInterface || '',
      masquerade: true
    }
    showForwardModal = true
  }

  function saveForwardRule() {
    forwardingRules = [...forwardingRules, { ...newForwardRule }]
    showForwardModal = false
  }

  function deleteForwardRule(index) {
    if (confirm('Delete this forwarding rule?')) {
      forwardingRules = forwardingRules.filter((_, i) => i !== index)
    }
  }

  // Group bandwidth limit helpers
  function openAddGroupLimit() {
    newGroupLimit = {
      group_name: '',
      upload_limit: 0,
      download_limit: 0,
      priority: 5,
      enabled: true
    }
    editingGroupLimit = null
    showGroupLimitModal = true
  }

  function openEditGroupLimit(limit, index) {
    newGroupLimit = { ...limit }
    editingGroupLimit = index
    showGroupLimitModal = true
  }

  function saveGroupLimit() {
    if (!newGroupLimit.group_name) {
      alert('Please select a group')
      return
    }
    
    if (editingGroupLimit !== null) {
      groupLimits[editingGroupLimit] = { ...newGroupLimit }
      groupLimits = [...groupLimits]
    } else {
      // Check if group already has a limit
      if (groupLimits.some(l => l.group_name === newGroupLimit.group_name)) {
        alert('This group already has a bandwidth limit. Edit the existing one instead.')
        return
      }
      groupLimits = [...groupLimits, { ...newGroupLimit }]
    }
    showGroupLimitModal = false
  }

  function deleteGroupLimit(index) {
    if (confirm('Delete this group bandwidth limit?')) {
      groupLimits = groupLimits.filter((_, i) => i !== index)
    }
  }
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>
          <i class="fas fa-lock me-2"></i> Access Control & Permissions
          <a href="/admin/about/guides/acl" class="btn btn-sm btn-outline-info ml-2" title="View ACL Setup Guide">
            <i class="fas fa-question-circle"></i>
          </a>
        </h1>
      </div>
      <div class="col-sm-6 text-right">
        <button class="btn btn-success" on:click={saveConfig} disabled={saving || loading}>
          <i class="fas fa-save me-1"></i> {saving ? 'Saving...' : 'Save All Changes'}
        </button>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    {#if loading}
      <div class="text-center p-5">
        <i class="fas fa-spinner fa-spin fa-3x"></i>
        <p class="mt-3">Loading configuration...</p>
      </div>
    {:else}
      <!-- Tab Navigation -->
      <div class="card card-primary card-outline card-outline-tabs">
        <div class="card-header p-0 border-bottom-0">
          <ul class="nav nav-tabs" role="tablist">
            <li class="nav-item">
              <button class="nav-link" class:active={activeTab === 'captive-portal'} on:click={() => activeTab = 'captive-portal'}>
                <i class="fas fa-wifi me-1"></i> Captive Portal
              </button>
            </li>
            <li class="nav-item">
              <button class="nav-link" class:active={activeTab === 'routes'} on:click={() => activeTab = 'routes'}>
                <i class="fas fa-route me-1"></i> Routes & ACL
              </button>
            </li>
            <li class="nav-item">
              <button class="nav-link" class:active={activeTab === 'forwarding'} on:click={() => activeTab = 'forwarding'}>
                <i class="fas fa-exchange-alt me-1"></i> Forwarding / NAT
              </button>
            </li>
            <li class="nav-item">
              <button class="nav-link" class:active={activeTab === 'dataflow'} on:click={() => activeTab = 'dataflow'}>
                <i class="fas fa-tachometer-alt me-1"></i> Dataflow / QoS
              </button>
            </li>
          </ul>
        </div>

        <div class="card-body">
          <!-- Captive Portal Tab -->
          {#if activeTab === 'captive-portal'}
            <div class="row">
              <div class="col-lg-6">
                <div class="card card-secondary">
                  <div class="card-header">
                    <h3 class="card-title">Portal Settings</h3>
                  </div>
                  <div class="card-body">
                    <div class="form-group">
                      <div class="custom-control custom-switch">
                        <input type="checkbox" class="custom-control-input" id="portalEnabled" bind:checked={captivePortalEnabled}>
                        <label class="custom-control-label" for="portalEnabled">
                          <strong>Enable Captive Portal</strong>
                        </label>
                      </div>
                      <small class="form-text text-muted">
                        When enabled, devices will see "Sign in to network" prompt and must authenticate before accessing the internet.
                      </small>
                    </div>

                    <hr>

                    <div class="form-group">
                      <label for="redirectUrl">Portal Redirect URL</label>
                      <input type="text" class="form-control" id="redirectUrl" bind:value={portalRedirectUrl} disabled={!captivePortalEnabled}>
                      <small class="form-text text-muted">URL where unauthenticated users are redirected</small>
                    </div>

                    <div class="form-group">
                      <div class="custom-control custom-switch">
                        <input type="checkbox" class="custom-control-input" id="authRequired" bind:checked={authenticationRequired} disabled={!captivePortalEnabled}>
                        <label class="custom-control-label" for="authRequired">Require Authentication</label>
                      </div>
                    </div>

                    <div class="form-group">
                      <label for="sessionTimeout">Session Timeout (seconds)</label>
                      <input type="number" class="form-control" id="sessionTimeout" bind:value={sessionTimeout} disabled={!captivePortalEnabled}>
                      <small class="form-text text-muted">How long until users must re-authenticate (0 = never)</small>
                    </div>
                  </div>
                </div>

                <div class="card card-info">
                  <div class="card-header">
                    <h3 class="card-title"><i class="fas fa-info-circle me-1"></i> How It Works</h3>
                  </div>
                  <div class="card-body">
                    <p><strong>The "Sign in to network" notification</strong> appears automatically when:</p>
                    <ol>
                      <li>Device connects via DHCP</li>
                      <li>Device checks connectivity URLs (listed below)</li>
                      <li>DNS hijacks return the portal IP instead</li>
                      <li>HTTP server returns special response triggering the prompt</li>
                    </ol>
                    <p class="mb-0 text-success"><i class="fas fa-check-circle"></i> CrabFlow handles all of this automatically!</p>
                  </div>
                </div>
              </div>

              <div class="col-lg-6">
                <div class="card card-warning">
                  <div class="card-header">
                    <h3 class="card-title">Captive Portal Detection Domains</h3>
                  </div>
                  <div class="card-body">
                    <p class="text-muted">These domains are used by devices to detect captive portals. CrabFlow will intercept requests to these domains and respond appropriately to trigger the "Sign in to network" prompt.</p>
                    
                    <div class="input-group mb-3">
                      <input type="text" class="form-control" placeholder="Add domain..." bind:value={newDetectionDomain} disabled={!captivePortalEnabled}>
                      <div class="input-group-append">
                        <button class="btn btn-warning" on:click={addDetectionDomain} disabled={!captivePortalEnabled}>
                          <i class="fas fa-plus"></i>
                        </button>
                      </div>
                    </div>

                    <div class="domain-list" style="max-height: 200px; overflow-y: auto;">
                      {#each detectionDomains as domain}
                        <div class="d-flex justify-content-between align-items-center p-2 border-bottom">
                          <span>
                            {#if domain.includes('msft')}
                              <i class="fab fa-windows text-info me-1"></i>
                            {:else if domain.includes('apple')}
                              <i class="fab fa-apple text-secondary me-1"></i>
                            {:else if domain.includes('google')}
                              <i class="fab fa-google text-danger me-1"></i>
                            {:else}
                              <i class="fas fa-globe me-1"></i>
                            {/if}
                            {domain}
                          </span>
                          <button class="btn btn-sm btn-outline-danger" on:click={() => removeDetectionDomain(domain)} disabled={!captivePortalEnabled}>
                            <i class="fas fa-times"></i>
                          </button>
                        </div>
                      {/each}
                    </div>
                  </div>
                </div>

                <div class="card card-success">
                  <div class="card-header">
                    <h3 class="card-title">Allowed Domains Before Auth</h3>
                  </div>
                  <div class="card-body">
                    <p class="text-muted">These domains will be accessible even before the user authenticates (e.g., for login APIs).</p>
                    
                    <div class="input-group mb-3">
                      <input type="text" class="form-control" placeholder="Add allowed domain..." bind:value={newAllowedDomain} disabled={!captivePortalEnabled}>
                      <div class="input-group-append">
                        <button class="btn btn-success" on:click={addAllowedDomain} disabled={!captivePortalEnabled}>
                          <i class="fas fa-plus"></i>
                        </button>
                      </div>
                    </div>

                    <div class="domain-list" style="max-height: 150px; overflow-y: auto;">
                      {#each allowedBeforeAuth as domain}
                        <div class="d-flex justify-content-between align-items-center p-2 border-bottom">
                          <span><i class="fas fa-check-circle text-success me-1"></i> {domain}</span>
                          <button class="btn btn-sm btn-outline-danger" on:click={() => removeAllowedDomain(domain)} disabled={!captivePortalEnabled}>
                            <i class="fas fa-times"></i>
                          </button>
                        </div>
                      {/each}
                      {#if allowedBeforeAuth.length === 0}
                        <p class="text-muted text-center mb-0">No domains allowed before authentication</p>
                      {/if}
                    </div>
                  </div>
                </div>
              </div>
            </div>

          <!-- Routes Tab -->
          {:else if activeTab === 'routes'}
            <div class="row">
              <div class="col-12">
                <div class="card">
                  <div class="card-header">
                    <h3 class="card-title">Access Control Rules</h3>
                    <div class="card-tools">
                      <button class="btn btn-primary btn-sm" on:click={openAddRoute}>
                        <i class="fas fa-plus me-1"></i> Add Rule
                      </button>
                    </div>
                  </div>
                  <div class="card-body table-responsive p-0">
                    <table class="table table-hover text-nowrap">
                      <thead>
                        <tr>
                          <th>Priority</th>
                          <th>Name</th>
                          <th>Source</th>
                          <th>Destination</th>
                          <th>Port</th>
                          <th>Protocol</th>
                          <th>Action</th>
                          <th>Actions</th>
                        </tr>
                      </thead>
                      <tbody>
                        {#each routeRules.sort((a, b) => a.priority - b.priority) as rule, i}
                          <tr>
                            <td><span class="badge badge-secondary">{rule.priority}</span></td>
                            <td>{rule.name}</td>
                            <td><code>{rule.source}</code></td>
                            <td><code>{rule.destination}</code></td>
                            <td>{rule.port}</td>
                            <td><span class="badge badge-info">{rule.protocol.toUpperCase()}</span></td>
                            <td>
                              {#if rule.action === 'allow'}
                                <span class="badge badge-success"><i class="fas fa-check me-1"></i> Allow</span>
                              {:else if rule.action === 'deny'}
                                <span class="badge badge-danger"><i class="fas fa-ban me-1"></i> Deny</span>
                              {:else}
                                <span class="badge badge-warning"><i class="fas fa-forward me-1"></i> Forward</span>
                              {/if}
                            </td>
                            <td>
                              <button class="btn btn-xs btn-info" on:click={() => openEditRoute(rule, i)}>
                                <i class="fas fa-edit"></i>
                              </button>
                              <button class="btn btn-xs btn-danger" on:click={() => deleteRoute(i)}>
                                <i class="fas fa-trash"></i>
                              </button>
                            </td>
                          </tr>
                        {:else}
                          <tr>
                            <td colspan="8" class="text-center text-muted">No route rules configured. Default policy: Allow all.</td>
                          </tr>
                        {/each}
                      </tbody>
                    </table>
                  </div>
                </div>

                <div class="callout callout-info">
                  <h5><i class="fas fa-info-circle"></i> Route Rule Priority</h5>
                  <p>Rules are evaluated in order of priority (lowest number first). The first matching rule determines the action. Use <code>*</code> for wildcards.</p>
                </div>
              </div>
            </div>

          <!-- Forwarding Tab -->
          {:else if activeTab === 'forwarding'}
            <div class="row">
              <div class="col-lg-6">
                <div class="card card-primary">
                  <div class="card-header">
                    <h3 class="card-title">IP Forwarding</h3>
                  </div>
                  <div class="card-body">
                    <div class="form-group">
                      <div class="custom-control custom-switch">
                        <input type="checkbox" class="custom-control-input" id="forwardingEnabled" bind:checked={forwardingEnabled}>
                        <label class="custom-control-label" for="forwardingEnabled">
                          <strong>Enable IP Forwarding</strong>
                        </label>
                      </div>
                      <small class="form-text text-muted">Allow packets to be forwarded between network interfaces</small>
                    </div>

                    <div class="form-group">
                      <div class="custom-control custom-switch">
                        <input type="checkbox" class="custom-control-input" id="natEnabled" bind:checked={natEnabled} disabled={!forwardingEnabled}>
                        <label class="custom-control-label" for="natEnabled">Enable NAT (Masquerade)</label>
                      </div>
                      <small class="form-text text-muted">Translate internal IPs to the uplink interface's IP</small>
                    </div>

                    <hr>

                    <div class="form-group">
                      <label for="uplinkIface">Uplink Interface (Internet)</label>
                      <select class="form-control" id="uplinkIface" bind:value={uplinkInterface} disabled={!forwardingEnabled}>
                        <option value="">-- Select Interface --</option>
                        {#each availableInterfaces as iface}
                          <option value={iface.name}>{iface.name} ({iface.ip || 'No IP'})</option>
                        {/each}
                      </select>
                      <small class="form-text text-muted">The interface connected to the internet</small>
                    </div>

                    <div class="form-group">
                      <label for="downlinkIface">Downlink Interface (Clients)</label>
                      <select class="form-control" id="downlinkIface" bind:value={downlinkInterface} disabled={!forwardingEnabled}>
                        <option value="">-- Select Interface --</option>
                        {#each availableInterfaces as iface}
                          <option value={iface.name}>{iface.name} ({iface.ip || 'No IP'})</option>
                        {/each}
                      </select>
                      <small class="form-text text-muted">The interface serving DHCP clients (hotspot)</small>
                    </div>
                  </div>
                </div>
              </div>

              <div class="col-lg-6">
                <div class="card card-secondary">
                  <div class="card-header">
                    <h3 class="card-title">Forwarding Rules</h3>
                    <div class="card-tools">
                      <button class="btn btn-light btn-sm" on:click={openAddForward} disabled={!forwardingEnabled}>
                        <i class="fas fa-plus"></i> Add Rule
                      </button>
                    </div>
                  </div>
                  <div class="card-body p-0">
                    <table class="table table-sm">
                      <thead>
                        <tr>
                          <th>Name</th>
                          <th>Source Network</th>
                          <th>Target</th>
                          <th>NAT</th>
                          <th></th>
                        </tr>
                      </thead>
                      <tbody>
                        {#each forwardingRules as rule, i}
                          <tr>
                            <td>{rule.name}</td>
                            <td><code>{rule.sourceNetwork}</code></td>
                            <td>{rule.targetInterface}</td>
                            <td>
                              {#if rule.masquerade}
                                <span class="badge badge-success">Yes</span>
                              {:else}
                                <span class="badge badge-secondary">No</span>
                              {/if}
                            </td>
                            <td>
                              <button class="btn btn-xs btn-danger" on:click={() => deleteForwardRule(i)}>
                                <i class="fas fa-trash"></i>
                              </button>
                            </td>
                          </tr>
                        {:else}
                          <tr>
                            <td colspan="5" class="text-center text-muted">No custom forwarding rules</td>
                          </tr>
                        {/each}
                      </tbody>
                    </table>
                  </div>
                </div>
              </div>
            </div>

          <!-- Dataflow Tab -->
          {:else if activeTab === 'dataflow'}
            <div class="row">
              <div class="col-lg-6">
                <div class="card card-primary">
                  <div class="card-header">
                    <h3 class="card-title"><i class="fas fa-tachometer-alt mr-2"></i>Global Bandwidth Limits</h3>
                  </div>
                  <div class="card-body">
                    <div class="form-group">
                      <div class="custom-control custom-switch">
                        <input type="checkbox" class="custom-control-input" id="bwEnabled" bind:checked={bandwidthLimits.enabled}>
                        <label class="custom-control-label" for="bwEnabled">
                          <strong>Enable Bandwidth Limiting</strong>
                        </label>
                      </div>
                    </div>

                    <hr>

                    <h6><i class="fas fa-globe mr-1"></i> Global Limits (All Traffic)</h6>
                    <div class="row">
                      <div class="col-6">
                        <div class="form-group">
                          <label>Upload (Mbps)</label>
                          <input type="number" class="form-control" bind:value={bandwidthLimits.globalUpload} disabled={!bandwidthLimits.enabled} placeholder="0 = unlimited">
                        </div>
                      </div>
                      <div class="col-6">
                        <div class="form-group">
                          <label>Download (Mbps)</label>
                          <input type="number" class="form-control" bind:value={bandwidthLimits.globalDownload} disabled={!bandwidthLimits.enabled} placeholder="0 = unlimited">
                        </div>
                      </div>
                    </div>

                    <h6><i class="fas fa-user mr-1"></i> Per-Client Default Limits</h6>
                    <div class="row">
                      <div class="col-6">
                        <div class="form-group">
                          <label>Upload (Mbps)</label>
                          <input type="number" class="form-control" bind:value={bandwidthLimits.perClientUpload} disabled={!bandwidthLimits.enabled} placeholder="0 = unlimited">
                        </div>
                      </div>
                      <div class="col-6">
                        <div class="form-group">
                          <label>Download (Mbps)</label>
                          <input type="number" class="form-control" bind:value={bandwidthLimits.perClientDownload} disabled={!bandwidthLimits.enabled} placeholder="0 = unlimited">
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                <div class="card card-secondary">
                  <div class="card-header">
                    <h3 class="card-title"><i class="fas fa-sort-amount-up mr-2"></i>Quality of Service (QoS)</h3>
                  </div>
                  <div class="card-body">
                    <div class="form-group">
                      <div class="custom-control custom-switch">
                        <input type="checkbox" class="custom-control-input" id="qosEnabled" bind:checked={qosEnabled}>
                        <label class="custom-control-label" for="qosEnabled">
                          <strong>Enable QoS</strong>
                        </label>
                      </div>
                      <small class="form-text text-muted">Prioritize certain traffic types over others</small>
                    </div>

                    <div class="callout callout-warning">
                      <h6><i class="fas fa-exclamation-triangle"></i> Advanced Feature</h6>
                      <p class="mb-0">QoS requires packet inspection and may impact performance on low-end hardware.</p>
                    </div>
                  </div>
                </div>
              </div>

              <div class="col-lg-6">
                <div class="card card-success">
                  <div class="card-header">
                    <h3 class="card-title"><i class="fas fa-users mr-2"></i>Group Bandwidth Limits</h3>
                    <div class="card-tools">
                      <button class="btn btn-light btn-sm" on:click={openAddGroupLimit} disabled={!bandwidthLimits.enabled}>
                        <i class="fas fa-plus"></i> Add Group Limit
                      </button>
                    </div>
                  </div>
                  <div class="card-body p-0">
                    <table class="table table-sm table-hover mb-0">
                      <thead class="thead-light">
                        <tr>
                          <th>Group</th>
                          <th>Upload</th>
                          <th>Download</th>
                          <th>Priority</th>
                          <th>Status</th>
                          <th></th>
                        </tr>
                      </thead>
                      <tbody>
                        {#each groupLimits as limit, i}
                          <tr>
                            <td><i class="fas fa-users-cog text-primary mr-1"></i>{limit.group_name}</td>
                            <td>{limit.upload_limit || '∞'} Mbps</td>
                            <td>{limit.download_limit || '∞'} Mbps</td>
                            <td>
                              <span class="badge" class:badge-success={limit.priority >= 7} class:badge-warning={limit.priority >= 4 && limit.priority < 7} class:badge-secondary={limit.priority < 4}>
                                {limit.priority}/10
                              </span>
                            </td>
                            <td>
                              {#if limit.enabled}
                                <span class="badge badge-success">Active</span>
                              {:else}
                                <span class="badge badge-secondary">Disabled</span>
                              {/if}
                            </td>
                            <td>
                              <button class="btn btn-xs btn-info" on:click={() => openEditGroupLimit(limit, i)}>
                                <i class="fas fa-edit"></i>
                              </button>
                              <button class="btn btn-xs btn-danger" on:click={() => deleteGroupLimit(i)}>
                                <i class="fas fa-trash"></i>
                              </button>
                            </td>
                          </tr>
                        {:else}
                          <tr>
                            <td colspan="6" class="text-center text-muted py-3">
                              <i class="fas fa-info-circle mr-1"></i>
                              No group bandwidth limits configured. Users in groups will use default per-client limits.
                            </td>
                          </tr>
                        {/each}
                      </tbody>
                    </table>
                  </div>
                  <div class="card-footer text-muted">
                    <small><i class="fas fa-info-circle mr-1"></i>Group limits override per-client defaults. Higher priority groups get bandwidth first.</small>
                  </div>
                </div>

                <div class="card card-info">
                  <div class="card-header">
                    <h3 class="card-title"><i class="fas fa-chart-pie mr-1"></i> Traffic Statistics</h3>
                  </div>
                  <div class="card-body">
                    <p class="text-muted mb-0">View real-time traffic statistics in the <a href="/admin/monitor">Monitor</a> page.</p>
                  </div>
                </div>
              </div>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</section>

<!-- Route Rule Modal -->
{#if showRouteModal}
  <div class="modal fade show" style="display: block; background: rgba(0,0,0,0.5);">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">{editingRoute !== null ? 'Edit' : 'Add'} Route Rule</h5>
          <button type="button" class="close" on:click={() => showRouteModal = false}>
            <span>&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Rule Name</label>
            <input type="text" class="form-control" bind:value={newRoute.name} placeholder="e.g., Block Social Media">
          </div>
          <div class="row">
            <div class="col-6">
              <div class="form-group">
                <label>Source IP/CIDR</label>
                <input type="text" class="form-control" bind:value={newRoute.source} placeholder="* or 192.168.1.0/24">
              </div>
            </div>
            <div class="col-6">
              <div class="form-group">
                <label>Destination IP/CIDR</label>
                <input type="text" class="form-control" bind:value={newRoute.destination} placeholder="* or 10.0.0.0/8">
              </div>
            </div>
          </div>
          <div class="row">
            <div class="col-4">
              <div class="form-group">
                <label>Port</label>
                <input type="text" class="form-control" bind:value={newRoute.port} placeholder="* or 80,443">
              </div>
            </div>
            <div class="col-4">
              <div class="form-group">
                <label>Protocol</label>
                <select class="form-control" bind:value={newRoute.protocol}>
                  <option value="any">Any</option>
                  <option value="tcp">TCP</option>
                  <option value="udp">UDP</option>
                  <option value="icmp">ICMP</option>
                </select>
              </div>
            </div>
            <div class="col-4">
              <div class="form-group">
                <label>Priority</label>
                <input type="number" class="form-control" bind:value={newRoute.priority} min="1" max="9999">
              </div>
            </div>
          </div>
          <div class="form-group">
            <label>Action</label>
            <select class="form-control" bind:value={newRoute.action}>
              <option value="allow">Allow</option>
              <option value="deny">Deny</option>
              <option value="forward">Forward</option>
            </select>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" on:click={() => showRouteModal = false}>Cancel</button>
          <button type="button" class="btn btn-primary" on:click={saveRoute}>Save Rule</button>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Forward Rule Modal -->
{#if showForwardModal}
  <div class="modal fade show" style="display: block; background: rgba(0,0,0,0.5);">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">Add Forwarding Rule</h5>
          <button type="button" class="close" on:click={() => showForwardModal = false}>
            <span>&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Rule Name</label>
            <input type="text" class="form-control" bind:value={newForwardRule.name} placeholder="e.g., Guest Network">
          </div>
          <div class="form-group">
            <label>Source Network (CIDR)</label>
            <input type="text" class="form-control" bind:value={newForwardRule.sourceNetwork} placeholder="192.168.137.0/24">
          </div>
          <div class="form-group">
            <label>Target Interface</label>
            <select class="form-control" bind:value={newForwardRule.targetInterface}>
              <option value="">-- Select --</option>
              {#each availableInterfaces as iface}
                <option value={iface.name}>{iface.name}</option>
              {/each}
            </select>
          </div>
          <div class="form-group">
            <div class="custom-control custom-switch">
              <input type="checkbox" class="custom-control-input" id="masquerade" bind:checked={newForwardRule.masquerade}>
              <label class="custom-control-label" for="masquerade">Enable NAT/Masquerade</label>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" on:click={() => showForwardModal = false}>Cancel</button>
          <button type="button" class="btn btn-primary" on:click={saveForwardRule}>Add Rule</button>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Group Bandwidth Limit Modal -->
{#if showGroupLimitModal}
  <div class="modal fade show" style="display: block; background: rgba(0,0,0,0.5);">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header bg-success text-white">
          <h5 class="modal-title"><i class="fas fa-users mr-2"></i>{editingGroupLimit !== null ? 'Edit' : 'Add'} Group Bandwidth Limit</h5>
          <button type="button" class="close text-white" on:click={() => showGroupLimitModal = false}>
            <span>&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Group</label>
            <select class="form-control" bind:value={newGroupLimit.group_name} disabled={editingGroupLimit !== null}>
              <option value="">-- Select Group --</option>
              {#each availableGroups as group}
                <option value={group.name}>{group.name}</option>
              {/each}
              {#if availableGroups.length === 0}
                <option value="" disabled>No groups available - create groups first</option>
              {/if}
            </select>
            <small class="form-text text-muted">Select a user group to apply bandwidth limits to</small>
          </div>
          
          <div class="row">
            <div class="col-6">
              <div class="form-group">
                <label><i class="fas fa-upload text-success mr-1"></i>Upload Limit (Mbps)</label>
                <input type="number" class="form-control" bind:value={newGroupLimit.upload_limit} min="0" placeholder="0 = unlimited">
              </div>
            </div>
            <div class="col-6">
              <div class="form-group">
                <label><i class="fas fa-download text-info mr-1"></i>Download Limit (Mbps)</label>
                <input type="number" class="form-control" bind:value={newGroupLimit.download_limit} min="0" placeholder="0 = unlimited">
              </div>
            </div>
          </div>
          
          <div class="form-group">
            <label>Priority (1-10)</label>
            <input type="range" class="form-control-range" bind:value={newGroupLimit.priority} min="1" max="10">
            <div class="d-flex justify-content-between text-muted">
              <small>Low Priority</small>
              <strong class="text-primary">{newGroupLimit.priority}</strong>
              <small>High Priority</small>
            </div>
            <small class="form-text text-muted">Higher priority groups get bandwidth allocation first</small>
          </div>
          
          <div class="form-group">
            <div class="custom-control custom-switch">
              <input type="checkbox" class="custom-control-input" id="groupLimitEnabled" bind:checked={newGroupLimit.enabled}>
              <label class="custom-control-label" for="groupLimitEnabled">Enable this limit</label>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" on:click={() => showGroupLimitModal = false}>Cancel</button>
          <button type="button" class="btn btn-success" on:click={saveGroupLimit}>
            <i class="fas fa-save mr-1"></i>{editingGroupLimit !== null ? 'Update' : 'Add'} Limit
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .nav-tabs .nav-link {
    cursor: pointer;
    border: none;
    color: #495057;
  }
  .nav-tabs .nav-link.active {
    background-color: #fff;
    border-color: #dee2e6 #dee2e6 #fff;
    color: #007bff;
    font-weight: 500;
  }
  .domain-list {
    background: #f8f9fa;
    border-radius: 4px;
  }
  .btn-xs {
    padding: 0.125rem 0.375rem;
    font-size: 0.75rem;
  }
</style>
