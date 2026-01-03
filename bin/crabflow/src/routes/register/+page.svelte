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
    
    loading = true
    error = null
    success = null

    try {
      const message = await api.registerUser(username, password)
      success = message
      setTimeout(() => {
        goto("/")
      }, 3000)
    } catch (e) {
      error = "Registration failed: " + e
    } finally {
      loading = false
    }
  }
</script>

<div class="middle center-align" style="height: 100vh;">
  <article class="round" style="width: 100%; max-width: 400px;">
    <h5 class="center-align">Register</h5>
    
    {#if error}
      <div class="chip error margin-bottom">{error}</div>
    {/if}

    {#if success}
      <div class="chip success margin-bottom">{success}</div>
    {/if}

    <form on:submit|preventDefault={doRegister}>
      <div class="field label border">
        <input type="text" bind:value={username} required />
        <label>Username</label>
      </div>
      
      <div class="field label border">
        <input type="password" bind:value={password} required />
        <label>Password</label>
      </div>

      <div class="field label border">
        <input type="password" bind:value={confirmPassword} required />
        <label>Confirm Password</label>
      </div>
      
      <button type="submit" class="fill" style="width: 100%;" disabled={loading}>
        {loading ? "Registering..." : "Create Account"}
      </button>
      
      <div class="center-align margin-top">
        <a href="/" class="button transparent">Back to Login</a>
      </div>
    </form>
  </article>
</div>
