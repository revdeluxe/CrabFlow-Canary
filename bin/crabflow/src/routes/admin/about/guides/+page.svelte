<script>
  import { goto } from '$app/navigation'
  
  const guides = [
    {
      id: 'getting-started',
      title: 'Getting Started',
      description: 'Initial setup and first steps with CrabFlow',
      icon: 'fas fa-rocket',
      color: 'primary'
    },
    {
      id: 'network-setup',
      title: 'Network Setup',
      description: 'Configure your network interfaces and routing',
      icon: 'fas fa-network-wired',
      color: 'info'
    },
    {
      id: 'dhcp',
      title: 'DHCP Server',
      description: 'Set up DHCP for automatic IP assignment',
      icon: 'fas fa-server',
      color: 'success'
    },
    {
      id: 'dns',
      title: 'DNS Server',
      description: 'Configure DNS resolution and ad-blocking',
      icon: 'fas fa-globe',
      color: 'warning'
    },
    {
      id: 'firewall',
      title: 'Firewall Rules',
      description: 'Protect your network with firewall rules',
      icon: 'fas fa-shield-alt',
      color: 'danger'
    },
    {
      id: 'acl',
      title: 'ACL & Permissions',
      description: 'Access control lists and captive portal setup',
      icon: 'fas fa-user-lock',
      color: 'purple'
    },
    {
      id: 'users-groups',
      title: 'Users & Groups',
      description: 'Manage users, groups, and permissions',
      icon: 'fas fa-users',
      color: 'teal'
    },
    {
      id: 'captive-portal',
      title: 'Captive Portal',
      description: 'Configure guest network authentication',
      icon: 'fas fa-sign-in-alt',
      color: 'orange'
    }
  ]
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0"><i class="fas fa-book mr-2"></i>Setup Guides</h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
      <li class="breadcrumb-item"><a href="/admin/about">About</a></li>
      <li class="breadcrumb-item active">Guides</li>
    </ol>
  </div>
</div>

<div class="row mt-3">
  <div class="col-12">
    <div class="card">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-compass mr-2"></i>Choose a Guide</h3>
      </div>
      <div class="card-body">
        <p class="text-muted mb-4">
          Select a topic below to learn how to configure and use CrabFlow's features.
          Each guide includes step-by-step instructions and common scenarios.
        </p>
        
        <div class="row">
          {#each guides as guide}
            <div class="col-lg-3 col-md-4 col-sm-6 mb-4">
              <div class="card card-outline card-{guide.color} h-100 guide-card" 
                   on:click={() => goto(`/admin/about/guides/${guide.id}`)}
                   on:keypress={(e) => e.key === 'Enter' && goto(`/admin/about/guides/${guide.id}`)}
                   role="button"
                   tabindex="0">
                <div class="card-body text-center">
                  <i class="{guide.icon} fa-3x text-{guide.color} mb-3"></i>
                  <h5 class="card-title">{guide.title}</h5>
                  <p class="card-text text-muted small">{guide.description}</p>
                </div>
                <div class="card-footer bg-transparent text-center">
                  <span class="text-{guide.color}">
                    View Guide <i class="fas fa-arrow-right ml-1"></i>
                  </span>
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Quick Tips Section -->
<div class="row">
  <div class="col-md-6">
    <div class="card card-info">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-lightbulb mr-2"></i>Quick Tips</h3>
      </div>
      <div class="card-body">
        <ul class="list-unstyled">
          <li class="mb-2">
            <i class="fas fa-check text-success mr-2"></i>
            Start with <strong>Getting Started</strong> if you're new to CrabFlow
          </li>
          <li class="mb-2">
            <i class="fas fa-check text-success mr-2"></i>
            Configure <strong>Network Setup</strong> before other services
          </li>
          <li class="mb-2">
            <i class="fas fa-check text-success mr-2"></i>
            Set up <strong>DHCP</strong> before <strong>DNS</strong> for best results
          </li>
          <li class="mb-2">
            <i class="fas fa-check text-success mr-2"></i>
            Use the <i class="fas fa-question-circle"></i> button on any page for quick help
          </li>
        </ul>
      </div>
    </div>
  </div>
  
  <div class="col-md-6">
    <div class="card card-warning">
      <div class="card-header">
        <h3 class="card-title"><i class="fas fa-exclamation-triangle mr-2"></i>Common Scenarios</h3>
      </div>
      <div class="card-body">
        <div class="list-group list-group-flush">
          <a href="/admin/about/guides/captive-portal" class="list-group-item list-group-item-action">
            <i class="fas fa-wifi text-info mr-2"></i>
            <strong>Guest WiFi with Login</strong>
            <small class="text-muted d-block">Set up a captive portal for guest access</small>
          </a>
          <a href="/admin/about/guides/dns" class="list-group-item list-group-item-action">
            <i class="fas fa-ban text-danger mr-2"></i>
            <strong>Block Ads Network-Wide</strong>
            <small class="text-muted d-block">Configure DNS-based ad blocking</small>
          </a>
          <a href="/admin/about/guides/acl" class="list-group-item list-group-item-action">
            <i class="fas fa-clock text-warning mr-2"></i>
            <strong>Bandwidth Limits by Group</strong>
            <small class="text-muted d-block">Set QoS rules for different user groups</small>
          </a>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .guide-card {
    cursor: pointer;
    transition: transform 0.2s, box-shadow 0.2s;
  }
  .guide-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 4px 15px rgba(0,0,0,0.1);
  }
</style>
