<script>
  import { api } from '$lib/tauri'
  import { goto } from '$app/navigation'

  let username = ""
  let password = ""
  let confirmPassword = ""
  let error = null
  let success = null
  let loading = false

  async function doRegister() {
    if (password !== confirmPassword) {
      error = "Passwords do not match"
      return
    }
    
    if (password.length < 4) {
      error = "Password must be at least 4 characters"
      return
    }
    
    loading = true
    error = null
    success = null

    try {
      const result = await api.registerUser(username, password)
      if (result.error) {
        error = result.error
      } else {
        success = result.message || "Registration successful! Redirecting to login..."
        setTimeout(() => {
          goto("/")
        }, 2000)
      }
    } catch (e) {
      error = "Registration failed: " + (e.message || e)
    } finally {
      loading = false
    }
  }
</script>

<div class="auth-page">
  <div class="auth-container">
    <div class="auth-card">
      <div class="auth-header">
        <a href="/" class="auth-logo"><b>Crab</b>Flow</a>
        <p class="auth-subtitle">Create your account</p>
      </div>
      
      <div class="auth-body">
        {#if error}
          <div class="auth-alert auth-alert-error">
            <i class="fas fa-exclamation-circle"></i>
            {error}
          </div>
        {/if}

        {#if success}
          <div class="auth-alert auth-alert-success">
            <i class="fas fa-check-circle"></i>
            {success}
          </div>
        {/if}

        <form on:submit|preventDefault={doRegister}>
          <div class="auth-input-group">
            <input 
              type="text" 
              class="auth-input" 
              placeholder="Username" 
              bind:value={username}
              autocomplete="username"
              required
              minlength="3"
            >
            <i class="fas fa-user auth-input-icon"></i>
          </div>
          
          <div class="auth-input-group">
            <input 
              type="password" 
              class="auth-input" 
              placeholder="Password" 
              bind:value={password}
              autocomplete="new-password"
              required
              minlength="4"
            >
            <i class="fas fa-lock auth-input-icon"></i>
          </div>
          
          <div class="auth-input-group">
            <input 
              type="password" 
              class="auth-input" 
              placeholder="Confirm Password" 
              bind:value={confirmPassword}
              autocomplete="new-password"
              required
            >
            <i class="fas fa-lock auth-input-icon"></i>
          </div>
          
          <button type="submit" class="auth-btn auth-btn-primary" disabled={loading}>
            {#if loading}
              <i class="fas fa-spinner fa-spin me-2"></i>
              Creating Account...
            {:else}
              <i class="fas fa-user-plus me-2"></i>
              Create Account
            {/if}
          </button>
        </form>
      </div>
      
      <div class="auth-footer">
        <span>Already have an account? </span>
        <a href="/" class="auth-link">Sign in</a>
      </div>
    </div>
  </div>
</div>
