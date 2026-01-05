<script>
import { goto } from '$app/navigation'
import { api } from '$lib/tauri'
import { onMount } from 'svelte'

let step = 1

let hostname = ""
let adminEmail = ""
let adminUser = ""
let adminPass = ""
let telemetry = false
let loading = false
let interfaces = []
let bindAddress = "0.0.0.0"
let upstreamInterface = "0.0.0.0"

onMount(async () => {
    try {
        interfaces = await api.listInterfaces()
        // Auto-select primary interface for upstream
        const primary = interfaces.find(i => i.is_primary);
        if (primary) {
            const ip = primary.ips.find(ip => ip.includes('.'));
            if (ip) upstreamInterface = ip;
        }
    } catch (e) {
        console.error("Failed to list interfaces", e)
    }
})

function next() {
  step += 1
}

async function finish() {
  loading = true
  try {
    await api.saveSetup({
      hostname,
      admin_email: adminEmail,
      admin_user: adminUser,
      admin_pass: adminPass,
      telemetry,
      first_run: false,
      dhcp: {
          enabled: false,
          captive_portal: false,
          bind_address: bindAddress,
          upstream_interface: upstreamInterface,
          range_start: "192.168.1.100",
          range_end: "192.168.1.200",
          subnet_mask: "255.255.255.0",
          gateway: "192.168.1.1",
          dns_servers: ["8.8.8.8", "8.8.4.4"],
          lease_time: 86400
      },
      hotspot: {
          enabled: false,
          ssid: "CrabFlow-Hotspot",
          password: "password123",
          interface: "Wi-Fi"
      }
    })
    // Apply upstream interface immediately
    try {
        await api.updateUpstreamInterface(upstreamInterface);
    } catch(err) {
        console.warn("Failed to apply upstream interface immediately:", err);
    }
    
    goto("/admin/dashboard")
  } catch (e) {
    console.error("Setup failed:", e)
    alert("Setup failed: " + e)
  } finally {
    loading = false
  }
}
</script>

<div class="login-page" style="min-height: 100vh; display: flex; align-items: center; justify-content: center;">
  <div class="login-box" style="width: 500px;">
    <div class="login-logo">
      <a href="/"><b>Crab</b>Flow</a>
    </div>
    
    <div class="card">
      <div class="card-body login-card-body">
        {#if step === 1}
            <p class="login-box-msg">Welcome to CrabFlow</p>
            <p class="text-center mb-4">This wizard will guide you through the initial configuration of your SDN controller.</p>
            <div class="row">
                <div class="col-12">
                    <button class="btn btn-primary btn-block" on:click={next}>Begin Setup</button>
                </div>
            </div>
        {/if}

        {#if step === 2}
            <p class="login-box-msg">Network Configuration</p>
            <form on:submit|preventDefault>
                <div class="input-group mb-3">
                    <input type="text" class="form-control" placeholder="Hostname" bind:value={hostname}>
                    <div class="input-group-append">
                        <div class="input-group-text">
                            <span class="fas fa-server"></span>
                        </div>
                    </div>
                </div>
                <div class="input-group mb-3">
                    <input type="email" class="form-control" placeholder="Admin Email" bind:value={adminEmail}>
                    <div class="input-group-append">
                        <div class="input-group-text">
                            <span class="fas fa-envelope"></span>
                        </div>
                    </div>
                </div>
                <div class="form-group mb-3">
                    <label>Service Interface (LAN/Hotspot)</label>
                    <select class="form-control" bind:value={bindAddress}>
                        <option value="0.0.0.0">All Interfaces (0.0.0.0)</option>
                        {#each interfaces as iface}
                            {#each iface.ips as ip}
                                {#if ip.includes('.')}
                                    <option value={ip}>
                                        {iface.name} ({ip})
                                    </option>
                                {/if}
                            {/each}
                        {/each}
                    </select>
                    <small class="form-text text-muted">Interface to serve DHCP/DNS on (e.g. Hotspot adapter).</small>
                </div>

                <div class="form-group mb-3">
                    <label>Upstream Interface (Internet)</label>
                    <select class="form-control" bind:value={upstreamInterface}>
                        <option value="0.0.0.0">Auto / Default Route</option>
                        {#each interfaces as iface}
                            {#each iface.ips as ip}
                                {#if ip.includes('.')}
                                    <option value={ip}>
                                        {iface.name} ({ip}) {iface.is_primary ? ' - Internet Active' : ''}
                                    </option>
                                {/if}
                            {/each}
                        {/each}
                    </select>
                    <small class="form-text text-muted">Interface used to forward DNS queries to the internet.</small>
                </div>
                <div class="row">
                    <div class="col-4">
                        <button type="button" class="btn btn-default btn-block" on:click={() => step = 1}>Back</button>
                    </div>
                    <div class="col-4 offset-4">
                        <button type="button" class="btn btn-primary btn-block" on:click={next} disabled={!hostname || !adminEmail}>Next</button>
                    </div>
                </div>
            </form>
        {/if}

        {#if step === 3}
             <p class="login-box-msg">Admin Account</p>
             <form on:submit|preventDefault>
                <div class="input-group mb-3">
                    <input type="text" class="form-control" placeholder="Username" bind:value={adminUser}>
                    <div class="input-group-append">
                        <div class="input-group-text">
                            <span class="fas fa-user"></span>
                        </div>
                    </div>
                </div>
                <div class="input-group mb-3">
                    <input type="password" class="form-control" placeholder="Password" bind:value={adminPass}>
                    <div class="input-group-append">
                        <div class="input-group-text">
                            <span class="fas fa-lock"></span>
                        </div>
                    </div>
                </div>
                <div class="row mb-3">
                    <div class="col-12">
                        <div class="icheck-primary">
                            <input type="checkbox" id="telemetry" bind:checked={telemetry}>
                            <label for="telemetry" class="ml-2">
                                Allow anonymous telemetry
                            </label>
                        </div>
                    </div>
                </div>
                <div class="row">
                    <div class="col-4">
                        <button type="button" class="btn btn-default btn-block" on:click={() => step = 2}>Back</button>
                    </div>
                    <div class="col-4 offset-4">
                        <button type="button" class="btn btn-primary btn-block" on:click={finish} disabled={!adminUser || !adminPass || loading}>
                            {loading ? '...' : 'Finish'}
                        </button>
                    </div>
                </div>
             </form>
        {/if}
      </div>
    </div>
    
    <div class="text-center mt-2">
        <small>Step {step} of 3</small>
    </div>
  </div>
</div>
