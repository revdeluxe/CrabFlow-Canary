<script>
  let activeTab = 'about' // 'about' or 'manual'
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0">About</h1>
  </div>
  <div class="col-sm-6">
    <a href="/admin/about/guides" class="btn btn-primary float-sm-right">
      <i class="fas fa-book mr-1"></i> Setup Guides
    </a>
  </div>
</div>

<!-- Tabs -->
<ul class="nav nav-tabs mb-3">
  <li class="nav-item">
    <button class="nav-link {activeTab === 'about' ? 'active' : ''}" on:click={() => activeTab = 'about'}>
      <i class="fas fa-info-circle mr-1"></i> About
    </button>
  </li>
  <li class="nav-item">
    <button class="nav-link {activeTab === 'manual' ? 'active' : ''}" on:click={() => activeTab = 'manual'}>
      <i class="fas fa-book mr-1"></i> Manual
    </button>
  </li>
</ul>

{#if activeTab === 'about'}
<div class="row">
  <div class="col-md-6">
    <div class="card card-primary card-outline">
      <div class="card-body box-profile">
        <div class="text-center">
          <img class="profile-user-img img-fluid img-circle"
               src="/favicon.png"
               alt="CrabFlow Logo"
               style="width: 100px; height: 100px;"
               on:error={(e) => e.currentTarget.style.display = 'none'}>
        </div>

        <h3 class="profile-username text-center">CrabFlow Canary</h3>

        <p class="text-muted text-center">Network Management System</p>

        <ul class="list-group list-group-unbordered mb-3">
          <li class="list-group-item">
            <b>Version</b> <span class="float-right badge badge-info">0.1.0 (Canary)</span>
          </li>
          <li class="list-group-item">
            <b>Build</b> <span class="float-right text-muted">Tauri + SvelteKit</span>
          </li>
          <li class="list-group-item">
            <b>License</b> <span class="float-right text-muted">MIT</span>
          </li>
        </ul>
        
        <a href="https://github.com/revdeluxe/crabflow" target="_blank" class="btn btn-outline-dark btn-block">
          <i class="fab fa-github mr-1"></i> View on GitHub
        </a>
      </div>
    </div>
  </div>
  
  <div class="col-md-6">
    <div class="card card-info">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-users mr-2"></i>Development Team</h3>
      </div>
      <div class="card-body p-0">
        <ul class="list-group list-group-flush">
          <li class="list-group-item">
            <div class="d-flex justify-content-between align-items-center">
              <div>
                <i class="fas fa-code text-primary mr-2"></i>
                <strong>Richard Villareal</strong>
                <span class="badge badge-primary ml-2">Lead Programmer</span>
              </div>
              <a href="https://revdeluxe.github.io" target="_blank" class="btn btn-sm btn-outline-primary">
                <i class="fas fa-external-link-alt"></i> Portfolio
              </a>
            </div>
          </li>
          <li class="list-group-item">
            <div class="d-flex justify-content-between align-items-center">
              <div>
                <i class="fas fa-user-tie text-success mr-2"></i>
                <strong>Keane Gavriel Gomez</strong>
                <span class="badge badge-success ml-2">Associate & Team Leader</span>
              </div>
              <a href="#" target="_blank" class="btn btn-sm btn-outline-success">
                <i class="fas fa-file-alt"></i> CV
              </a>
            </div>
          </li>
          <li class="list-group-item">
            <div class="d-flex justify-content-between align-items-center">
              <div>
                <i class="fas fa-search text-warning mr-2"></i>
                <strong>Teyah Dela Rosa</strong>
                <span class="badge badge-warning ml-2">Researcher</span>
              </div>
              <a href="#" target="_blank" class="btn btn-sm btn-outline-warning">
                <i class="fas fa-file-alt"></i> CV
              </a>
            </div>
          </li>
          <li class="list-group-item">
            <div class="d-flex justify-content-between align-items-center">
              <div>
                <i class="fas fa-search text-info mr-2"></i>
                <strong>Yishin Aguiluz</strong>
                <span class="badge badge-info ml-2">Researcher</span>
              </div>
              <a href="#" target="_blank" class="btn btn-sm btn-outline-info">
                <i class="fas fa-file-alt"></i> CV
              </a>
            </div>
          </li>
        </ul>
      </div>
    </div>
    
    <div class="card card-secondary">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-heart mr-2"></i>Built With</h3>
      </div>
      <div class="card-body">
        <div class="row text-center">
          <div class="col-3">
            <i class="fab fa-rust fa-2x text-orange"></i>
            <p class="mb-0"><small>Rust</small></p>
          </div>
          <div class="col-3">
            <i class="fas fa-crab fa-2x text-danger"></i>
            <p class="mb-0"><small>Tauri</small></p>
          </div>
          <div class="col-3">
            <i class="fab fa-svelte fa-2x text-orange"></i>
            <p class="mb-0"><small>Svelte</small></p>
          </div>
          <div class="col-3">
            <i class="fas fa-palette fa-2x text-info"></i>
            <p class="mb-0"><small>AdminLTE</small></p>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
{/if}

{#if activeTab === 'manual'}
<div class="row">
  <div class="col-12">
    <div class="card">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-book-open mr-2"></i>CrabFlow User Manual</h3>
      </div>
      <div class="card-body">
        <!-- Getting Started -->
        <div class="callout callout-info">
          <h5><i class="fas fa-rocket"></i> Getting Started</h5>
          <p>CrabFlow is a network management system that provides DHCP, DNS, and captive portal functionality for small networks and homelabs.</p>
        </div>
        
        <!-- Dashboard -->
        <h4 class="mt-4"><i class="fas fa-tachometer-alt mr-2 text-primary"></i>Dashboard</h4>
        <p>The dashboard provides an at-a-glance view of your network:</p>
        <ul>
          <li><strong>Service Status</strong> - View the status of DHCP and DNS servers</li>
          <li><strong>Quick Stats</strong> - See active clients, DNS queries, and blocked ads</li>
          <li><strong>Resource Usage</strong> - Monitor CPU and memory consumption</li>
          <li><strong>Quick Actions</strong> - Fast links to common tasks</li>
        </ul>
        
        <hr>
        
        <!-- DHCP Management -->
        <h4 class="mt-4"><i class="fas fa-network-wired mr-2 text-info"></i>DHCP Management</h4>
        <p>Manage IP address allocation for your network:</p>
        <ul>
          <li><strong>Server Mode</strong> - CrabFlow acts as a DHCP server, assigning IPs to clients</li>
          <li><strong>Client Mode</strong> - CrabFlow receives its IP from an upstream DHCP server</li>
          <li><strong>Static Leases</strong> - Reserve specific IPs for devices by MAC address</li>
          <li><strong>Lease Management</strong> - View and manage active DHCP leases</li>
        </ul>
        <div class="alert alert-warning">
          <i class="fas fa-exclamation-triangle"></i>
          <strong>Note:</strong> Only enable DHCP Server mode if no other DHCP server exists on the network to avoid conflicts.
        </div>
        
        <hr>
        
        <!-- DNS Management -->
        <h4 class="mt-4"><i class="fas fa-globe mr-2 text-success"></i>DNS Management</h4>
        <p>Configure local DNS resolution and view query logs:</p>
        <ul>
          <li><strong>DNS Records</strong> - Create A, AAAA, CNAME, and TXT records for local domains</li>
          <li><strong>Query Logs</strong> - View all DNS queries with status (Allowed, Blocked, Portal)</li>
          <li><strong>Homelab Templates</strong> - Quick setup for popular services like Plex, Home Assistant, etc.</li>
          <li><strong>ACL Integration</strong> - Create firewall rules directly from DNS records</li>
        </ul>
        
        <hr>
        
        <!-- ACL & Permissions -->
        <h4 class="mt-4"><i class="fas fa-shield-alt mr-2 text-warning"></i>ACL & Permissions</h4>
        <p>Control access and traffic flow on your network:</p>
        <ul>
          <li><strong>Captive Portal</strong> - Require authentication before internet access</li>
          <li><strong>Routes</strong> - Define custom routing rules</li>
          <li><strong>Forwarding</strong> - Set up port forwarding for services</li>
          <li><strong>Dataflow/QoS</strong> - Configure bandwidth limits per user group</li>
        </ul>
        
        <hr>
        
        <!-- Ad Blocking -->
        <h4 class="mt-4"><i class="fas fa-ban mr-2 text-danger"></i>Ad Blocking</h4>
        <p>Block ads and trackers at the DNS level:</p>
        <ul>
          <li><strong>Blocklists</strong> - Subscribe to community-maintained blocklists</li>
          <li><strong>Whitelist</strong> - Allow specific domains that are blocked</li>
          <li><strong>Custom Rules</strong> - Add your own domains to block</li>
        </ul>
        
        <hr>
        
        <!-- Users & Groups -->
        <h4 class="mt-4"><i class="fas fa-users mr-2 text-purple"></i>Users & Groups</h4>
        <p>Manage network users and their permissions:</p>
        <ul>
          <li><strong>User Accounts</strong> - Create accounts for captive portal authentication</li>
          <li><strong>Groups</strong> - Organize users into groups with shared permissions</li>
          <li><strong>Bandwidth Limits</strong> - Set upload/download limits per group</li>
        </ul>
        
        <hr>
        
        <!-- Hotspot -->
        <h4 class="mt-4"><i class="fas fa-wifi mr-2 text-cyan"></i>Hotspot</h4>
        <p>Create a wireless access point (requires compatible hardware):</p>
        <ul>
          <li><strong>SSID</strong> - Set the network name</li>
          <li><strong>Password</strong> - Configure WPA2 security</li>
          <li><strong>Captive Portal</strong> - Automatically redirect users to login page</li>
        </ul>
        
      </div>
      <div class="card-footer">
        <small class="text-muted">
          <i class="fas fa-question-circle mr-1"></i>
          Need help? Open an issue on 
          <a href="https://github.com/revdeluxe/crabflow/issues" target="_blank">GitHub</a>
        </small>
      </div>
    </div>
  </div>
</div>
{/if}
