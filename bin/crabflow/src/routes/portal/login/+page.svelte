<script>
  import { onMount } from 'svelte'
  
  // This is where you would implement the captive portal logic
  // For example, checking if the user is authenticated, and if not, showing a login form
  // The backend logic for intercepting traffic and redirecting to this page would be in src-tauri/src/network/cportal.rs
</script>

<div class="container" style="height: 100vh; display: flex; align-items: center; justify-content: center; flex-direction: column;">
  <h1>Welcome to CrabFlow Network</h1>
  <p>Please sign in to access the internet.</p>
  
  <div class="card" style="width: 100%; max-width: 400px; margin-top: 2rem;">
    <form>
      <label style="display: block; margin-bottom: 0.5rem;">Voucher Code / Username</label>
      <input type="text" placeholder="Enter code" style="width: 100%; padding: 0.5rem; margin-bottom: 1rem;" />
      <button type="submit" style="width: 100%;">Connect</button>
    </form>
  </div>
</div>
