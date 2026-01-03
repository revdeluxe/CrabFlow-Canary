<script>
  import { onMount, onDestroy } from 'svelte'
  import { page } from '$app/stores'
  import { api } from '$lib/tauri'
  import { goto } from '$app/navigation'
  import { session } from '$lib/stores/session'

  onMount(async () => {
    document.body.classList.add('sidebar-mini', 'layout-fixed');

    // Load AdminLTE scripts
    try {
        if (typeof window !== 'undefined') {
            const jquery = (await import('jquery')).default;
            window.$ = window.jQuery = jquery;
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
        // Restore session
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
        document.body.classList.remove('sidebar-mini', 'layout-fixed');
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
    document.body.classList.toggle('sidebar-collapse');
  }
</script>

<div class="wrapper">
  <!-- Navbar -->
  <nav class="main-header navbar navbar-expand navbar-white navbar-light">
    <!-- Left navbar links -->
    <ul class="navbar-nav">
      <li class="nav-item">
        <a class="nav-link" href="#" role="button" on:click|preventDefault={toggleSidebar}><i class="fas fa-bars"></i></a>
      </li>
      <li class="nav-item d-none d-sm-inline-block">
        <a href="/admin/dashboard" class="nav-link">Home</a>
      </li>
    </ul>

    <!-- Right navbar links -->
    <ul class="navbar-nav ml-auto">
      <li class="nav-item">
        <a class="nav-link" href="#" on:click|preventDefault={doLogout} role="button">
          <i class="fas fa-sign-out-alt"></i> Logout
        </a>
      </li>
    </ul>
  </nav>
  <!-- /.navbar -->

  <!-- Main Sidebar Container -->
  <aside class="main-sidebar sidebar-dark-primary elevation-4">
    <!-- Brand Logo -->
    <a href="/admin/dashboard" class="brand-link">
      <span class="brand-text font-weight-light">CrabFlow</span>
    </a>

    <!-- Sidebar -->
    <div class="sidebar">
      <!-- Sidebar Menu -->
      <nav class="mt-2">
        <ul class="nav nav-pills nav-sidebar flex-column" data-widget="treeview" role="menu" data-accordion="false">
          
          <li class="nav-item">
            <a href="/admin/dashboard" class="nav-link" class:active={$page.url.pathname.includes('/admin/dashboard')}>
              <i class="nav-icon fas fa-tachometer-alt"></i>
              <p>Dashboard</p>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/devices" class="nav-link" class:active={$page.url.pathname.includes('/admin/devices')}>
              <i class="nav-icon fas fa-laptop"></i>
              <p>Devices</p>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/dhcp" class="nav-link" class:active={$page.url.pathname.includes('/admin/dhcp')}>
              <i class="nav-icon fas fa-network-wired"></i>
              <p>DHCP</p>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/dns" class="nav-link" class:active={$page.url.pathname.includes('/admin/dns')}>
              <i class="nav-icon fas fa-globe"></i>
              <p>DNS</p>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/firewall" class="nav-link" class:active={$page.url.pathname.includes('/admin/firewall')}>
              <i class="nav-icon fas fa-shield-alt"></i>
              <p>Firewall</p>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/monitor" class="nav-link" class:active={$page.url.pathname.includes('/admin/monitor')}>
              <i class="nav-icon fas fa-chart-line"></i>
              <p>Monitor</p>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/users" class="nav-link" class:active={$page.url.pathname.includes('/admin/users')}>
              <i class="nav-icon fas fa-users"></i>
              <p>Users</p>
            </a>
          </li>

          <li class="nav-header">SYSTEM</li>

          <li class="nav-item">
            <a href="/admin/profile" class="nav-link" class:active={$page.url.pathname.includes('/admin/profile')}>
              <i class="nav-icon fas fa-user"></i>
              <p>Profile</p>
            </a>
          </li>

          <li class="nav-item">
            <a href="/admin/settings" class="nav-link" class:active={$page.url.pathname.includes('/admin/settings')}>
              <i class="nav-icon fas fa-cogs"></i>
              <p>Settings</p>
            </a>
          </li>

        </ul>
      </nav>
      <!-- /.sidebar-menu -->
    </div>
    <!-- /.sidebar -->
  </aside>

  <!-- Content Wrapper. Contains page content -->
  <div class="content-wrapper">
    <!-- Main content -->
    <section class="content pt-3">
      <div class="container-fluid">
         <slot />
      </div>
    </section>
    <!-- /.content -->
  </div>
  <!-- /.content-wrapper -->
</div>
