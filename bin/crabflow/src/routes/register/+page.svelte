<script>
  import { api } from '$lib/tauri'
  import { goto } from '$app/navigation'
  import { onMount, onDestroy } from 'svelte'

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

  onMount(() => {
      document.body.classList.add('register-page');
  });
  
  onDestroy(() => {
      document.body.classList.remove('register-page');
  });
</script>

<div class="register-box">
  <div class="card card-outline card-primary">
    <div class="card-header text-center">
      <a href="/" class="h1"><b>Crab</b>Flow</a>
    </div>
    <div class="card-body">
      <p class="login-box-msg">Register a new membership</p>

      {#if error}
        <div class="alert alert-danger">
            <i class="icon fas fa-ban"></i> {error}
        </div>
      {/if}

      {#if success}
        <div class="alert alert-success">
            <i class="icon fas fa-check"></i> {success}
        </div>
      {/if}

      <form on:submit|preventDefault={doRegister}>
        <div class="input-group mb-3">
          <input type="text" class="form-control" placeholder="Username" bind:value={username} required>
          <div class="input-group-append">
            <div class="input-group-text">
              <span class="fas fa-user"></span>
            </div>
          </div>
        </div>
        <div class="input-group mb-3">
          <input type="password" class="form-control" placeholder="Password" bind:value={password} required>
          <div class="input-group-append">
            <div class="input-group-text">
              <span class="fas fa-lock"></span>
            </div>
          </div>
        </div>
        <div class="input-group mb-3">
          <input type="password" class="form-control" placeholder="Retype password" bind:value={confirmPassword} required>
          <div class="input-group-append">
            <div class="input-group-text">
              <span class="fas fa-lock"></span>
            </div>
          </div>
        </div>
        <div class="row">
          <div class="col-8">
            <div class="icheck-primary">
              <!-- Checkbox for terms could go here -->
            </div>
          </div>
          <!-- /.col -->
          <div class="col-4">
            <button type="submit" class="btn btn-primary btn-block" disabled={loading}>
                {loading ? '...' : 'Register'}
            </button>
          </div>
          <!-- /.col -->
        </div>
      </form>

      <a href="/" class="text-center mt-2 d-block">I already have a membership</a>
    </div>
    <!-- /.form-box -->
  </div><!-- /.card -->
</div>
<!-- /.register-box -->

<style>
    /* Ensure the box is centered vertically in the page if the body class handles the flex centering */
    /* .register-page class on body in AdminLTE usually handles this min-height and centering */
</style>
