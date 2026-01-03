<script>
  import { onMount, onDestroy } from 'svelte'
  import { goto } from '$app/navigation'
  import { session } from '$lib/stores/session'
  import { api } from '$lib/tauri'

  let username = ""
  let password = ""
  let error = null
  let loading = true

  onMount(async () => {
    document.body.classList.add('login-page');
    try {
      // Check if already logged in
      const token = localStorage.getItem('session_token')
      if (token) {
        const user = await api.checkAuth(token)
        if (user) {
          session.set({ user, token })
          if (user.role === "admin" || user.username === "admin") { // Fallback if role is missing
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

  onDestroy(() => {
    if (typeof document !== 'undefined') {
        document.body.classList.remove('login-page');
    }
  })

  async function doLogin() {
    try {
      const result = await api.login(username, password)
      
      if (result.success) {
        session.set({
          user: result.user,
          token: result.token
        })
        
        // Persist token
        localStorage.setItem('session_token', result.token)

        error = null

        if (result.user.role === "admin") {
          goto("/admin/dashboard")
        } else {
          goto("/portal/dashboard")
        }
      } else {
        error = result.message
      }
    } catch (e) {
      error = "Login failed: " + e
      console.error(e)
    }
  }
</script>

{#if loading}
  <div class="d-flex justify-content-center align-items-center" style="height: 100vh;">
    <div class="spinner-border text-primary" role="status">
      <span class="sr-only">Loading...</span>
    </div>
  </div>
{:else}
<div class="login-box" style="margin: 10vh auto;">
  <div class="login-logo">
    <a href="/"><b>Crab</b>Flow</a>
  </div>
  <!-- /.login-logo -->
  <div class="card">
    <div class="card-body login-card-body">
      <p class="login-box-msg">Sign in to start your session</p>

      {#if error}
        <div class="alert alert-danger">
          {error}
        </div>
      {/if}

      <form on:submit|preventDefault={doLogin}>
        <div class="input-group mb-3">
          <input type="text" class="form-control" placeholder="Username" bind:value={username}>
          <div class="input-group-append">
            <div class="input-group-text">
              <span class="fas fa-user"></span>
            </div>
          </div>
        </div>
        <div class="input-group mb-3">
          <input type="password" class="form-control" placeholder="Password" bind:value={password}>
          <div class="input-group-append">
            <div class="input-group-text">
              <span class="fas fa-lock"></span>
            </div>
          </div>
        </div>
        <div class="row">
          <div class="col-8">
            <div class="icheck-primary">
              <input type="checkbox" id="remember">
              <label for="remember">
                Remember Me
              </label>
            </div>
          </div>
          <!-- /.col -->
          <div class="col-4">
            <button type="submit" class="btn btn-primary btn-block">Sign In</button>
          </div>
          <!-- /.col -->
        </div>
      </form>

      <p class="mb-0">
        <a href="/register" class="text-center">Register a new membership</a>
      </p>
    </div>
    <!-- /.login-card-body -->
  </div>
</div>
{/if}
