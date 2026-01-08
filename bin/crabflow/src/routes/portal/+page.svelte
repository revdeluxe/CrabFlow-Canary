<script>
  import { onMount } from 'svelte'
  import { api } from '$lib/tauri'
  import { goto } from '$app/navigation'

  let htmlContent = ""
  let loading = true

  onMount(async () => {
    try {
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
                // In a real environment, we should get the client IP from the server or request
                // For now, we use a placeholder or assume the backend handles it if we send a special flag
                // Since this runs in browser, 'invoke' might fail if not properly bridged.
                // Assuming this is running in a context where 'api' works.
                await api.tagUser(username, "127.0.0.1", "Web Client")
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
