<script>
  import { api } from '$lib/tauri'
  import { onMount } from 'svelte'
  import { goto } from '$app/navigation'

  let setupConfig = {
    hostname: "",
    admin_email: "",
    admin_user: "",
    admin_pass: "",
    telemetry: false,
    first_run: false,
    monitor_interval: 5000,
    dhcp: {
      enabled: false,
      captive_portal: false,
      range_start: "192.168.1.100",
      range_end: "192.168.1.200",
      subnet_mask: "255.255.255.0",
      gateway: "192.168.1.1",
      dns_servers: ["8.8.8.8", "8.8.4.4"],
      lease_time: 86400
    }
  }
  let userSettings = {
    auto_approve_new_users: false
  }
  let showModal = false
  let showRiskModal = false
  let loading = true

  onMount(async () => {
    try {
      const [setup, userSet] = await Promise.all([
        api.loadSetup(),
        api.getUserSettings()
      ])
      setupConfig = setup
      
      // Ensure dhcp object exists if it wasn't in the file
      if (!setupConfig.dhcp) {
        setupConfig.dhcp = {
          enabled: false,
          captive_portal: false,
          range_start: "192.168.1.100",
          range_end: "192.168.1.200",
          subnet_mask: "255.255.255.0",
          gateway: "192.168.1.1",
          dns_servers: ["8.8.8.8", "8.8.4.4"],
          lease_time: 86400
        }
      }

      userSettings = userSet
    } catch (e) {
      console.error("Failed to load settings:", e)
    } finally {
      loading = false
    }
  })

  async function saveChanges() {
    try {
      await api.saveSetup(setupConfig)
      await api.setUserSettings(userSettings)
      alert("Settings saved successfully!")
    } catch (e) {
      console.error("Failed to save settings:", e)
      alert("Failed to save settings: " + e)
    }
  }

  function toggleAutoApprove() {
    if (!userSettings.auto_approve_new_users) {
      // User is trying to enable it -> show warning
      showRiskModal = true
    } else {
      // User is disabling it -> just do it
      userSettings.auto_approve_new_users = false
    }
  }

  function confirmAutoApprove() {
    userSettings.auto_approve_new_users = true
    showRiskModal = false
  }

  async function reinitialize() {
    try {
      await api.resetSetup()
      alert("Setup has been reset. Redirecting to setup wizard...")
      goto("/setup")
    } catch (e) {
      console.error("Failed to reset setup:", e)
      alert("Failed to reset setup: " + e)
    }
    showModal = false
  }

  async function restartNetworking() {
    if (!confirm("Are you sure you want to restart networking services? This may briefly interrupt connectivity.")) return
    try {
      await api.restartNetworking()
      alert("Networking services restarted.")
    } catch (e) {
      alert("Failed to restart networking: " + e)
    }
  }

  async function restartApp() {
    if (!confirm("Are you sure you want to restart the application?")) return
    try {
      await api.restartApplication()
    } catch (e) {
      alert("Failed to restart application: " + e)
    }
  }

  async function shutdownSystem() {
    if (!confirm("Are you sure you want to SHUTDOWN the system?")) return
    try {
      await api.shutdownSystem()
    } catch (e) {
      alert("Failed to shutdown system: " + e)
    }
  }
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>Settings</h1>
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
    {:else}
      <div class="card card-primary card-outline">
        <div class="card-header">
          <h3 class="card-title">Configuration</h3>
        </div>
        <div class="card-body">
          <form on:submit|preventDefault={saveChanges}>
            
            <h5 class="text-primary"><i class="fas fa-cogs mr-2"></i> General</h5>
            <div class="row">
              <div class="col-md-6">
                <div class="form-group">
                  <label>Hostname</label>
                  <input type="text" class="form-control" bind:value={setupConfig.hostname} />
                </div>
              </div>
              <div class="col-md-6">
                <div class="form-group">
                  <label>Admin Email</label>
                  <input type="email" class="form-control" bind:value={setupConfig.admin_email} />
                </div>
              </div>
              <div class="col-md-6">
                <div class="form-group">
                  <label>Admin Username</label>
                  <input type="text" class="form-control" bind:value={setupConfig.admin_user} />
                </div>
              </div>
              <div class="col-md-6">
                <div class="form-group">
                  <label>Admin Password</label>
                  <input type="password" class="form-control" bind:value={setupConfig.admin_pass} />
                </div>
              </div>
              <div class="col-md-12">
                <div class="form-group">
                  <div class="custom-control custom-switch">
                    <input type="checkbox" class="custom-control-input" id="telemetrySwitch" bind:checked={setupConfig.telemetry}>
                    <label class="custom-control-label" for="telemetrySwitch">Enable Telemetry</label>
                  </div>
                </div>
              </div>
            </div>

            <hr>

            <h5 class="text-primary" id="network-settings"><i class="fas fa-network-wired mr-2"></i> Network (DHCP)</h5>
            <div class="row">
              <div class="col-md-12">
                <div class="form-group">
                  <div class="custom-control custom-switch">
                    <input type="checkbox" class="custom-control-input" id="dhcpSwitch" bind:checked={setupConfig.dhcp.enabled}>
                    <label class="custom-control-label" for="dhcpSwitch">Enable DHCP Server</label>
                  </div>
                </div>
                <div class="form-group">
                  <div class="custom-control custom-switch">
                    <input type="checkbox" class="custom-control-input" id="cportalSwitch" bind:checked={setupConfig.dhcp.captive_portal}>
                    <label class="custom-control-label" for="cportalSwitch">Enable Captive Portal (Forces DNS to Gateway)</label>
                  </div>
                </div>
              </div>
              <div class="col-md-6">
                <div class="form-group">
                  <label>Range Start</label>
                  <input type="text" class="form-control" bind:value={setupConfig.dhcp.range_start} />
                </div>
              </div>
              <div class="col-md-6">
                <div class="form-group">
                  <label>Range End</label>
                  <input type="text" class="form-control" bind:value={setupConfig.dhcp.range_end} />
                </div>
              </div>
              <div class="col-md-6">
                <div class="form-group">
                  <label>Subnet Mask</label>
                  <input type="text" class="form-control" bind:value={setupConfig.dhcp.subnet_mask} />
                </div>
              </div>
              <div class="col-md-6">
                <div class="form-group">
                  <label>Gateway</label>
                  <input type="text" class="form-control" bind:value={setupConfig.dhcp.gateway} />
                </div>
              </div>
              <div class="col-md-6">
                <div class="form-group">
                  <label>DNS Servers</label>
                  <input type="text" class="form-control"
                    value={setupConfig.dhcp.dns_servers.join(', ')} 
                    on:input={(e) => setupConfig.dhcp.dns_servers = e.target.value.split(',').map(s => s.trim())} 
                  />
                </div>
              </div>
              <div class="col-md-6">
                <div class="form-group">
                  <label>Lease Time (s)</label>
                  <input type="number" class="form-control" bind:value={setupConfig.dhcp.lease_time} />
                </div>
              </div>
            </div>

            <hr>

            <h5 class="text-primary"><i class="fas fa-chart-line mr-2"></i> Monitor</h5>
            <div class="form-group">
              <label>Update Frequency (ms)</label>
              <input type="number" class="form-control" bind:value={setupConfig.monitor_interval} min="1000" step="100" />
              <small class="form-text text-muted">How often the dashboard updates system stats.</small>
            </div>

            <hr>

            <h5 class="text-primary"><i class="fas fa-users mr-2"></i> User Management</h5>
            <div class="form-group">
              <div class="custom-control custom-switch">
                <input type="checkbox" class="custom-control-input" id="autoApproveSwitch" checked={userSettings.auto_approve_new_users} on:click|preventDefault={toggleAutoApprove}>
                <label class="custom-control-label" for="autoApproveSwitch">Auto-approve new user registrations</label>
              </div>
              <small class="form-text text-muted">If enabled, new users can log in immediately without admin approval.</small>
            </div>

            <button type="submit" class="btn btn-primary mt-3"><i class="fas fa-save"></i> Save Changes</button>
          </form>
        </div>
      </div>

      <!-- Power Management -->
      <div class="card card-warning collapsed-card">
        <div class="card-header">
          <h3 class="card-title">Power Management</h3>
          <div class="card-tools">
            <button type="button" class="btn btn-tool" data-card-widget="collapse"><i class="fas fa-plus"></i></button>
          </div>
        </div>
        <div class="card-body">
          <p>Control system power and service states.</p>
          <div class="row">
            <div class="col-md-4">
              <button class="btn btn-info btn-block" on:click={restartNetworking}>
                <i class="fas fa-network-wired"></i> Restart Networking
              </button>
            </div>
            <div class="col-md-4">
              <button class="btn btn-warning btn-block" on:click={restartApp}>
                <i class="fas fa-sync"></i> Restart CrabFlow
              </button>
            </div>
            <div class="col-md-4">
              <button class="btn btn-danger btn-block" on:click={shutdownSystem}>
                <i class="fas fa-power-off"></i> Shutdown System
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="card card-danger card-outline collapsed-card">
        <div class="card-header">
          <h3 class="card-title">Danger Zone</h3>
          <div class="card-tools">
            <button type="button" class="btn btn-tool" data-card-widget="collapse"><i class="fas fa-plus"></i></button>
          </div>
        </div>
        <div class="card-body">
          <p>Resetting the setup will require you to run the initial configuration wizard again.</p>
          <button class="btn btn-danger" on:click={() => showModal = true}>Reinitialize Setup Wizard</button>
        </div>
      </div>
    {/if}
  </div>
</section>

{#if showRiskModal}
  <div class="modal fade show" style="display: block; background: rgba(0,0,0,0.5);">
    <div class="modal-dialog">
      <div class="modal-content bg-danger">
        <div class="modal-header">
          <h4 class="modal-title">Security Risk Warning</h4>
          <button type="button" class="close" on:click={() => showRiskModal = false}>
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <p>Enabling auto-approval allows anyone with access to the registration page to create an account and access the system immediately.</p>
          <p>Are you sure you want to enable this?</p>
        </div>
        <div class="modal-footer justify-content-between">
          <button type="button" class="btn btn-outline-light" on:click={() => showRiskModal = false}>Cancel</button>
          <button type="button" class="btn btn-outline-light" on:click={confirmAutoApprove}>Yes, Enable Risk</button>
        </div>
      </div>
    </div>
  </div>
{/if}

{#if showModal}
  <div class="modal fade show" style="display: block; background: rgba(0,0,0,0.5);">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title">Confirm Reinitialize</h4>
          <button type="button" class="close" on:click={() => showModal = false}>
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <p>This will wipe your current setup and require running the wizard again.</p>
        </div>
        <div class="modal-footer justify-content-between">
          <button type="button" class="btn btn-default" on:click={() => showModal = false}>Cancel</button>
          <button type="button" class="btn btn-danger" on:click={reinitialize}>Yes, Reset</button>
        </div>
      </div>
    </div>
  </div>
{/if}
