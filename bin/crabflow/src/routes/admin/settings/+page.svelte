<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let setupConfig = null
  let showModal = false

  onMount(async () => {
    try {
      setupConfig = await invoke("load_setup_config")
    } catch (e) {
      console.error("Failed to load setup config:", e)
    }
  })

  async function reinitialize() {
    try {
      await invoke("reset_setup_config")
      setupConfig = await invoke("load_setup_config")
      alert("Setup has been reset. Restart CrabFlow to run the wizard again.")
    } catch (e) {
      console.error("Failed to reset setup:", e)
    }
    showModal = false
  }
</script>

<h2>Settings</h2>

<!-- Config card always visible -->
{#if setupConfig}
  <div class="config-card">
    <h3>Current Setup</h3>
    <ul>
      <li><strong>Hostname:</strong> {setupConfig.hostname}</li>
      <li><strong>Admin Email:</strong> {setupConfig.admin_email}</li>
      <li><strong>Admin User:</strong> {setupConfig.admin_user}</li>
      <li><strong>Telemetry:</strong> {setupConfig.telemetry ? "Enabled" : "Disabled"}</li>
      <li><strong>First Run:</strong> {setupConfig.first_run ? "true" : "false"}</li>
    </ul>
  </div>
{/if}

<!-- Reinitialize button -->
<button on:click={() => showModal = true}>Reinitialize Setup Wizard</button>

{#if showModal}
  <div class="modal-backdrop">
    <div class="modal">
      <h3>Confirm Reinitialize</h3>
      <p>This will wipe your current setup and require running the wizard again.</p>
      <div class="actions">
        <button on:click={reinitialize}>Yes, Reset</button>
        <button on:click={() => showModal = false}>Cancel</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .config-card {
    background: #f9f9f9;
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  .config-card ul {
    list-style: none;
    padding: 0;
  }
  .config-card li {
    margin: 0.25rem 0;
  }
  .modal-backdrop {
    position: fixed;
    top: 0; left: 0;
    width: 100%; height: 100%;
    background: rgba(0,0,0,0.5);
    display: flex; align-items: center; justify-content: center;
  }
  .modal {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    max-width: 400px;
    text-align: center;
  }
  .actions {
    margin-top: 1rem;
    display: flex;
    justify-content: space-around;
  }
</style>
