<script>
  import { api } from '$lib/tauri'
  import { onMount } from 'svelte'

  let templateContent = ""
  let loading = true
  let saving = false

  onMount(async () => {
    try {
      templateContent = await api.getPortalTemplate()
    } catch (e) {
      alert("Failed to load template: " + e)
    } finally {
      loading = false
    }
  })

  async function saveTemplate() {
    saving = true
    try {
      await api.savePortalTemplate(templateContent)
      alert("Template saved successfully!")
    } catch (e) {
      alert("Failed to save template: " + e)
    } finally {
      saving = false
    }
  }

  async function resetDefault() {
    if (!confirm("Are you sure you want to reset to the default template? This will overwrite your changes.")) return
    try {
        // Deleting the file or just fetching default logic from backend?
        // Backend returns default if file missing.
        // But we want to see the default code.
        // Let's just clear the content and reload, assuming backend handles empty file or we can implement a reset command.
        // Actually, the backend `get_portal_template` returns default if file missing.
        // So we can just delete the file? Or we can hardcode default here?
        // Better: Add a reset command or just manually set it here.
        // For now, let's just warn the user they can delete the file manually or we can implement a reset endpoint later.
        // Or, we can just set the variable to a known default string.
        
        templateContent = `<div class="container" style="height: 100vh; display: flex; align-items: center; justify-content: center; flex-direction: column; font-family: sans-serif;">
  <h1>Welcome to CrabFlow Network</h1>
  <p>Please sign in to access the internet.</p>
  
  <div class="card" style="width: 100%; max-width: 400px; margin-top: 2rem; padding: 2rem; border: 1px solid #ccc; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
    <form id="login-form" onsubmit="handleLogin(event)">
      <label style="display: block; margin-bottom: 0.5rem;">Username / Voucher</label>
      <input type="text" id="username" name="username" placeholder="Enter code" style="width: 100%; padding: 0.5rem; margin-bottom: 1rem;" required />
      
      <label style="display: block; margin-bottom: 0.5rem;">Password</label>
      <input type="password" id="password" name="password" placeholder="Enter password" style="width: 100%; padding: 0.5rem; margin-bottom: 1rem;" required />

      <button type="submit" style="width: 100%; padding: 0.75rem; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;">Connect</button>
    </form>
    <div id="error-message" style="color: red; margin-top: 1rem; display: none;"></div>
  </div>
</div>`
    } catch (e) {
        console.error(e)
    }
  }
</script>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>Captive Portal Editor</h1>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    <div class="card card-primary card-outline">
      <div class="card-header">
        <h3 class="card-title">HTML Template Editor</h3>
        <div class="card-tools">
            <button class="btn btn-warning btn-sm" on:click={resetDefault}>Reset to Default</button>
        </div>
      </div>
      <div class="card-body">
        <div class="alert alert-info">
            <h5><i class="icon fas fa-info"></i> Instructions</h5>
            <p>Edit the HTML below to customize the captive portal login page.</p>
            <ul>
                <li>Ensure you keep the <code>form</code> with <code>onsubmit="handleLogin(event)"</code>.</li>
                <li>Input fields must have ids <code>username</code> and <code>password</code>.</li>
                <li>The <code>handleLogin</code> function is provided by the system automatically.</li>
            </ul>
        </div>
        
        {#if loading}
            <p>Loading...</p>
        {:else}
            <div class="form-group">
                <textarea class="form-control" rows="20" style="font-family: monospace; white-space: pre;" bind:value={templateContent}></textarea>
            </div>
        {/if}
      </div>
      <div class="card-footer">
        <button class="btn btn-primary" on:click={saveTemplate} disabled={saving || loading}>
            {saving ? 'Saving...' : 'Save Changes'}
        </button>
        <a href="/admin/settings" class="btn btn-default float-right">Back to Settings</a>
      </div>
    </div>
  </div>
</section>
