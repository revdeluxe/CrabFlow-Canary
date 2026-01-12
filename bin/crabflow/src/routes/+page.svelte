<script>
  import { onMount, onDestroy } from 'svelte'
  import { goto } from '$app/navigation'
  import { session } from '$lib/stores/session'
  import { api } from '$lib/tauri'

  let username = ""
  let password = ""
  let rememberMe = false
  let error = null
  let loading = true

  onMount(async () => {
    try {
      // Check if already logged in
      const token = localStorage.getItem('session_token')
      if (token) {
        const user = await api.checkAuth(token)
        if (user) {
          session.set({ user, token })
          if (["admin", "user_manager", "staff"].includes(user.role) || user.username === "admin") {
            goto("/admin/dashboard")
            return
          } else {
            goto("/portal/dashboard")
            return
          }
        }
      }

      const isFirstRun = await api.checkFirstRun()
      if (isFirstRun) {
        goto("/setup")
      }
    } catch (e) {
      console.error("Failed to check status:", e)
    } finally {
      loading = false
    }
  })

  async function doLogin() {
    error = null
    try {
      const result = await api.login(username, password)
      
      if (result.success) {
        session.set({
          user: result.user,
          token: result.token
        })
        
        // Persist token
        localStorage.setItem('session_token', result.token)

        if (["admin", "user_manager", "staff"].includes(result.user.role)) {
          goto("/admin/dashboard")
        } else {
          goto("/portal/dashboard")
        }
      } else {
        error = result.message || "Invalid credentials"
      }
    } catch (e) {
      error = "Login failed: " + e
      console.error(e)
    }
  }
</script>

{#if loading}
  <div class="auth-loading">
    <div class="auth-spinner"></div>
  </div>
{:else}
  <div class="auth-page">
    <div class="auth-container">
      <div class="auth-card">
        <div class="auth-header">
          <a href="/" class="auth-logo"><b>Crab</b>Flow</a>
          <p class="auth-subtitle">Network Management System</p>
        </div>
        
        <div class="auth-body">
          {#if error}
            <div class="auth-alert auth-alert-error">
              <i class="fas fa-exclamation-circle"></i>
              {error}
            </div>
          {/if}

          <form on:submit|preventDefault={doLogin}>
            <div class="auth-input-group">
              <input 
                type="text" 
                class="auth-input" 
                placeholder="Username" 
                bind:value={username}
                autocomplete="username"
                required
              >
              <i class="fas fa-user auth-input-icon"></i>
            </div>
            
            <div class="auth-input-group">
              <input 
                type="password" 
                class="auth-input" 
                placeholder="Password" 
                bind:value={password}
                autocomplete="current-password"
                required
              >
              <i class="fas fa-lock auth-input-icon"></i>
            </div>
            
            <label class="auth-checkbox">
              <input type="checkbox" bind:checked={rememberMe}>
              <span>Remember me</span>
            </label>
            
            <button type="submit" class="auth-btn auth-btn-primary">
              <i class="fas fa-sign-in-alt me-2"></i>
              Sign In
            </button>
          </form>
        </div>
        
        <div class="auth-footer">
          <span>Don't have an account? </span>
          <a href="/register" class="auth-link">Create one</a>
        </div>
      </div>
    </div>
  </div>
{/if}
