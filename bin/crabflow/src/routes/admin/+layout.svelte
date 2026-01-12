<script>
  import { onMount, onDestroy } from 'svelte'
  import { page } from '$app/stores'
  import { api } from '$lib/tauri'
  import { goto } from '$app/navigation'
  import { session } from '$lib/stores/session'

  let sidebarCollapsed = false;

  onMount(async () => {
    // AdminLTE 4 uses different body classes
    document.body.classList.add('layout-fixed', 'sidebar-mini');

    // Load Bootstrap 5 and AdminLTE 4 scripts
    try {
      if (typeof window !== 'undefined') {
        await import('bootstrap/dist/js/bootstrap.bundle.min.js');
        await import('admin-lte/dist/js/adminlte.min.js');
      }
    } catch (e) {
      console.error("Failed to load AdminLTE scripts", e);
    }

    const token = localStorage.getItem('session_token')
    if (!token) {
      goto("/")
      return
    }

    try {
      const user = await api.checkAuth(token)
      if (user) {
        session.set({ user, token })
      } else {
        console.warn("Invalid session token")
        localStorage.removeItem('session_token')
        session.set(null)
        goto("/")
      }
    } catch (e) {
      console.error("Auth check failed:", e)
      localStorage.removeItem('session_token')
      session.set(null)
      goto("/")
    }
  })

  onDestroy(() => {
    if (typeof document !== 'undefined') {
      document.body.classList.remove('layout-fixed', 'sidebar-mini', 'sidebar-collapse');
    }
  })

  async function doLogout() {
    try {
      const token = localStorage.getItem('session_token')
      if (token) {
        await api.logout(token)
      }
    } catch (e) {
      console.error("Logout failed:", e)
    } finally {
      localStorage.removeItem('session_token')
      session.set(null)
      goto("/")
    }
  }

  function toggleSidebar() {
    sidebarCollapsed = !sidebarCollapsed;
    document.body.classList.toggle('sidebar-collapse');
  }

  function isActive(path) {
    return $page.url.pathname.includes(path);
  }
</script>

<!-- AdminLTE 4 Layout Structure -->
<div class="app-wrapper">
  <!-- Header / Navbar -->
  <nav class="app-header navbar navbar-expand bg-body">
    <div class="container-fluid">
      <!-- Start navbar links -->
      <ul class="navbar-nav">
        <li class="nav-item">
          <a class="nav-link" href="#" role="button" on:click|preventDefault={toggleSidebar}>
            <i class="fas fa-bars"></i>
          </a>
        </li>
        <li class="nav-item d-none d-md-block">
          <a href="/admin/dashboard" class="nav-link">Home</a>
        </li>
      </ul>

      <!-- End navbar links -->
      <ul class="navbar-nav ms-auto">
        <li class="nav-item">
          <a class="nav-link" href="#" on:click|preventDefault={doLogout} role="button">
            <i class="fas fa-sign-out-alt"></i> Logout
          </a>
        </li>
      </ul>
    </div>
  </nav>
  <!-- /.app-header -->

  <!-- Sidebar -->
  <aside class="app-sidebar bg-dark shadow" data-bs-theme="dark">
    <!-- Sidebar Brand -->
    <div class="sidebar-brand">
      <a href="/admin/dashboard" class="brand-link">
        <span class="brand-text fw-light sidebar-brand-text">CrabFlow</span>
      </a>
    </div>
    <!-- /.sidebar-brand -->

    <!-- Sidebar Wrapper -->
    <div class="sidebar-wrapper" data-overlayscrollbars-viewport>
      <nav class="mt-2">
        <!-- Sidebar Menu -->
        <ul class="nav sidebar-menu flex-column" data-lte-toggle="treeview" role="menu" data-accordion="false">
          
          <li class="nav-item">
            <a href="/admin/dashboard" class="nav-link" class:active={isActive('/admin/dashboard')}>
              <i class="nav-icon fas fa-tachometer-alt"></i>
              <span>Dashboard</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/devices" class="nav-link" class:active={isActive('/admin/devices')}>
              <i class="nav-icon fas fa-laptop"></i>
              <span>Devices</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/dhcp" class="nav-link" class:active={isActive('/admin/dhcp')}>
              <i class="nav-icon fas fa-network-wired"></i>
              <span>DHCP</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/dns" class="nav-link" class:active={isActive('/admin/dns')}>
              <i class="nav-icon fas fa-globe"></i>
              <span>DNS</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/firewall" class="nav-link" class:active={isActive('/admin/firewall')}>
              <i class="nav-icon fas fa-shield-alt"></i>
              <span>Firewall</span>
            </a>
          </li>

          {#if $session?.user?.role === 'admin'}
          <li class="nav-item">
            <a href="/admin/acl" class="nav-link" class:active={isActive('/admin/acl')}>
              <i class="nav-icon fas fa-lock"></i>
              <span>ACL & Permissions</span>
            </a>
          </li>
          {/if}

          <li class="nav-item">
            <a href="/admin/monitor" class="nav-link" class:active={isActive('/admin/monitor')}>
              <i class="nav-icon fas fa-chart-line"></i>
              <span>Monitor</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/adblock" class="nav-link" class:active={isActive('/admin/adblock')}>
              <i class="nav-icon fas fa-ban"></i>
              <span>Ad-Block</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/users" class="nav-link" class:active={isActive('/admin/users')}>
              <i class="nav-icon fas fa-users"></i>
              <span>Users</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/groups" class="nav-link" class:active={isActive('/admin/groups')}>
              <i class="nav-icon fas fa-users-cog"></i>
              <span>Groups</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/portal-editor" class="nav-link" class:active={isActive('/admin/portal-editor')}>
              <i class="nav-icon fas fa-edit"></i>
              <span>Portal Editor</span>
            </a>
          </li>

          <li class="nav-header">SYSTEM</li>

          <li class="nav-item">
            <a href="/admin/profile" class="nav-link" class:active={isActive('/admin/profile')}>
              <i class="nav-icon fas fa-user"></i>
              <span>Profile</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/settings" class="nav-link" class:active={isActive('/admin/settings')}>
              <i class="nav-icon fas fa-cogs"></i>
              <span>Settings</span>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/about" class="nav-link" class:active={isActive('/admin/about')}>
              <i class="nav-icon fas fa-info-circle"></i>
              <span>About</span>
            </a>
          </li>

        </ul>
        <!-- /.sidebar-menu -->
      </nav>
    </div>
    <!-- /.sidebar-wrapper -->
  </aside>
  <!-- /.app-sidebar -->

  <!-- Main Content -->
  <main class="app-main">
    <div class="app-content">
      <div class="container-fluid">
        <slot />
      </div>
    </div>
  </main>
  <!-- /.app-main -->
</div>
<!-- /.app-wrapper -->

<style>
  /* AdminLTE 4 specific overrides */
  .sidebar-brand {
    padding: 0.8rem 1rem;
    border-bottom: 1px solid rgba(255,255,255,0.1);
  }
  
  .brand-link {
    color: #fff;
    text-decoration: none;
    font-size: 1.25rem;
  }
  
  .nav-header {
    padding: 0.5rem 1rem;
    font-size: 0.75rem;
    color: rgba(255,255,255,0.5);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  
  .sidebar-menu .nav-link {
    color: rgba(255,255,255,0.8);
    padding: 0.5rem 1rem;
    display: flex;
    align-items: center;
    border-radius: 0.25rem;
    margin: 0.125rem 0.5rem;
    transition: background-color 0.15s ease-in-out;
  }
  
  .sidebar-menu .nav-link:hover {
    background-color: rgba(255,255,255,0.1);
    color: #fff;
  }
  
  .sidebar-menu .nav-link.active {
    background-color: var(--bs-primary);
    color: #fff;
  }
  
  .sidebar-menu .nav-icon {
    width: 1.6rem;
    margin-right: 0.5rem;
    text-align: center;
  }
  
  .app-sidebar {
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    width: 250px;
    z-index: 1040;
    overflow-y: auto;
  }
  
  .app-header {
    margin-left: 250px;
    transition: margin-left 0.3s ease-in-out;
  }
  
  .app-main {
    margin-left: 250px;
    margin-top: 0;
    min-height: calc(100vh - 57px);
    transition: margin-left 0.3s ease-in-out;
  }
  
  :global(body.sidebar-collapse) .app-header,
  :global(body.sidebar-collapse) .app-main {
    margin-left: 4.6rem;
  }
  
  :global(body.sidebar-collapse) .app-sidebar {
    width: 4.6rem;
  }
  
  :global(body.sidebar-collapse) .app-sidebar:hover {
    width: 250px;
  }
  
  :global(body.sidebar-collapse) .app-sidebar .sidebar-brand-text,
  :global(body.sidebar-collapse) .app-sidebar .nav-link span,
  :global(body.sidebar-collapse) .app-sidebar .nav-header {
    opacity: 0;
    white-space: nowrap;
  }
  
  :global(body.sidebar-collapse) .app-sidebar:hover .sidebar-brand-text,
  :global(body.sidebar-collapse) .app-sidebar:hover .nav-link span,
  :global(body.sidebar-collapse) .app-sidebar:hover .nav-header {
    opacity: 1;
  }
</style>
