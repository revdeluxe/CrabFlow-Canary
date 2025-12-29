<script>
  import { page } from '$app/stores'
</script>

<style>
  .layout {
    display: flex;
    height: 100vh;
  }
  .sidebar {
    width: 220px;
    background: #1e1e1e;
    color: #fff;
    padding: 1rem;
    display: flex;
    flex-direction: column;
  }
  .sidebar a {
    color: #fff;
    text-decoration: none;
    margin: 0.5rem 0;
    padding: 0.5rem;
    border-radius: 4px;
  }
  .sidebar a.active {
    background: #333;
  }
  .content {
    flex: 1;
    padding: 2rem;
    overflow-y: auto;
    background: #f5f5f5;
  }
</style>

<div class="layout">
  <nav class="sidebar">
    <h2>CrabFlow</h2>
    <a href="/dashboard" class={$page.url.pathname === '/dashboard' ? 'active' : ''}>Dashboard</a>
    <a href="/dhcp" class={$page.url.pathname === '/dhcp' ? 'active' : ''}>DHCP</a>
    <a href="/dns" class={$page.url.pathname === '/dns' ? 'active' : ''}>DNS</a>
    <a href="/monitor" class={$page.url.pathname === '/monitor' ? 'active' : ''}>Traffic Monitor</a>
    <a href="/devices" class={$page.url.pathname === '/devices' ? 'active' : ''}>Devices</a>
    <a href="/portal" class={$page.url.pathname.startsWith('/portal') ? 'active' : ''}>Captive Portal</a>
    <a href="/admin" class={$page.url.pathname.startsWith('/admin') ? 'active' : ''}>Admin UI</a>
  </nav>

  <main class="content">
    <slot />
  </main>
</div>
