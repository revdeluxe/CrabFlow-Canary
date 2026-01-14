<script>
  import { onMount } from 'svelte'
  import { api } from '$lib/tauri'
  import { goto } from '$app/navigation'

  let htmlContent = ""
  let loading = true

  onMount(async () => {
    try {
      // Ensure ACL is present/enabled so captive portal endpoints work on fresh installs
      try { await api.ensureDefaultAcl(); } catch (e) { console.warn('ensureDefaultAcl error:', e) }
      htmlContent = await api.getPortalTemplate()
    } catch (e) {
      console.error("Failed to load portal template:", e)
      // Fallback if API fails
      htmlContent = `<div style="text-align:center; margin-top: 50px;"><h1>Welcome</h1><p>Please login.</p></div>`
    } finally {
      loading = false
    }

    // Expose login handler to the global scope so the injected HTML can call it
    window.handleLogin = async (event) => {
        event.preventDefault()
        const username = document.getElementById('username')?.value
        const password = document.getElementById('password')?.value
        const errorDiv = document.getElementById('error-message')

        if (!username || !password) {
            if(errorDiv) { errorDiv.innerText = "Please enter username and password"; errorDiv.style.display = 'block'; }
            return
        }

        try {
            await api.login(username, password)
            
            // Attempt to authorize device (Captive Portal)
            // Note: tag_user signature is (username, ip, deviceName). MAC is inferred by backend.
                try {
                // Prefer the hostname the client used to reach the UI. That hostname
                // will normally be the server address (e.g. 10.0.0.1) when accessed
                // over the LAN. Avoid sending 127.0.0.1 which causes backend to
                // treat the MAC as unknown/dummy.
                const clientIp = (window.location && window.location.hostname) ? window.location.hostname : '127.0.0.1';
                await api.tagUser(username, clientIp, "Web Client")
            } catch (tagErr) {
                console.warn("Tag user failed (might be non-critical if just dashboard access):", tagErr)
            }

            // Redirect to success page or original URL (if we knew it)
            // For now, go to user dashboard
            goto('/portal/dashboard')
        } catch (e) {
            if(errorDiv) { errorDiv.innerText = "Login failed: " + e; errorDiv.style.display = 'block'; }
            else alert("Login failed: " + e)
        }
    }
  })
</script>

<svelte:head>
    <title>Captive Portal Login</title>
</svelte:head>

{#if loading}
    <div style="display: flex; justify-content: center; align-items: center; height: 100vh;">Loading...</div>
{:else}
    {@html htmlContent}
{/if}
