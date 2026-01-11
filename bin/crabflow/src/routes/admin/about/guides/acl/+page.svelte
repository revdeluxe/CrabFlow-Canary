<script>
  let activeScenario = 'captive-portal'
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0"><i class="fas fa-user-lock mr-2"></i>ACL & Permissions Guide</h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
      <li class="breadcrumb-item"><a href="/admin/about/guides">Guides</a></li>
      <li class="breadcrumb-item active">ACL</li>
    </ol>
  </div>
</div>

<div class="row mt-3">
  <!-- Sidebar Navigation -->
  <div class="col-md-3">
    <div class="card card-outline card-purple">
      <div class="card-header">
        <h3 class="card-title">Scenarios</h3>
      </div>
      <div class="card-body p-0">
        <ul class="nav nav-pills flex-column">
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeScenario === 'captive-portal' ? 'active' : ''}"
                    on:click={() => activeScenario = 'captive-portal'}>
              <i class="fas fa-sign-in-alt mr-2"></i> Captive Portal
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeScenario === 'routes' ? 'active' : ''}"
                    on:click={() => activeScenario = 'routes'}>
              <i class="fas fa-route mr-2"></i> Route Rules
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeScenario === 'forwarding' ? 'active' : ''}"
                    on:click={() => activeScenario = 'forwarding'}>
              <i class="fas fa-exchange-alt mr-2"></i> Forwarding / NAT
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeScenario === 'dataflow' ? 'active' : ''}"
                    on:click={() => activeScenario = 'dataflow'}>
              <i class="fas fa-tachometer-alt mr-2"></i> Dataflow / QoS
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeScenario === 'group-limits' ? 'active' : ''}"
                    on:click={() => activeScenario = 'group-limits'}>
              <i class="fas fa-users-cog mr-2"></i> Group Bandwidth
            </button>
          </li>
        </ul>
      </div>
    </div>
    
    <a href="/admin/acl" class="btn btn-purple btn-block mt-3">
      <i class="fas fa-cog mr-2"></i> Open ACL Settings
    </a>
  </div>
  
  <!-- Content Area -->
  <div class="col-md-9">
    {#if activeScenario === 'captive-portal'}
      <!-- Captive Portal Guide -->
      <div class="card">
        <div class="card-header bg-purple">
          <h3 class="card-title"><i class="fas fa-sign-in-alt mr-2"></i>Setting Up a Captive Portal</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-info-circle mr-2"></i>What is a Captive Portal?</h5>
            <p>A captive portal intercepts network traffic and redirects users to a login page before they can access the internet. This is commonly used for guest WiFi, hotels, cafes, and schools.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-list-ol mr-2 text-purple"></i>Step-by-Step Setup</h5>
          
          <div class="timeline">
            <div class="time-label">
              <span class="bg-purple">Prerequisites</span>
            </div>
            <div>
              <i class="fas fa-check bg-success"></i>
              <div class="timeline-item">
                <h3 class="timeline-header">Ensure DHCP is configured</h3>
                <div class="timeline-body">
                  The captive portal requires DHCP to be running so devices get IP addresses from CrabFlow.
                  Go to <a href="/admin/dhcp">DHCP Settings</a> and ensure the server is enabled.
                </div>
              </div>
            </div>
            <div>
              <i class="fas fa-check bg-success"></i>
              <div class="timeline-item">
                <h3 class="timeline-header">Configure DNS server</h3>
                <div class="timeline-body">
                  DNS must be configured to redirect detection requests.
                  Go to <a href="/admin/dns">DNS Settings</a> and ensure it's running.
                </div>
              </div>
            </div>
            
            <div class="time-label">
              <span class="bg-primary">Configuration</span>
            </div>
            <div>
              <i class="fas fa-toggle-on bg-primary"></i>
              <div class="timeline-item">
                <h3 class="timeline-header">1. Enable Captive Portal</h3>
                <div class="timeline-body">
                  <ol>
                    <li>Go to <strong>ACL & Permissions</strong> → <strong>Captive Portal</strong> tab</li>
                    <li>Toggle <strong>Enable Captive Portal</strong> to ON</li>
                    <li>Set the <strong>Portal Redirect URL</strong> (default: <code>http://portal.crabflow.local</code>)</li>
                  </ol>
                </div>
              </div>
            </div>
            <div>
              <i class="fas fa-key bg-warning"></i>
              <div class="timeline-item">
                <h3 class="timeline-header">2. Configure Authentication</h3>
                <div class="timeline-body">
                  <ul>
                    <li><strong>Require Authentication:</strong> Users must log in before accessing internet</li>
                    <li><strong>Session Timeout:</strong> How long until users need to re-authenticate (in minutes)</li>
                  </ul>
                </div>
              </div>
            </div>
            <div>
              <i class="fas fa-globe bg-info"></i>
              <div class="timeline-item">
                <h3 class="timeline-header">3. Set Allowed Domains</h3>
                <div class="timeline-body">
                  Add domains that should be accessible <em>before</em> authentication:
                  <ul>
                    <li>Your portal domain</li>
                    <li>Payment processors (if using paid WiFi)</li>
                    <li>Any required services</li>
                  </ul>
                </div>
              </div>
            </div>
            <div>
              <i class="fas fa-save bg-success"></i>
              <div class="timeline-item">
                <h3 class="timeline-header">4. Save Configuration</h3>
                <div class="timeline-body">
                  Click <strong>Save Configuration</strong> to apply changes. The portal will be active immediately.
                </div>
              </div>
            </div>
          </div>
          
          <div class="callout callout-warning mt-4">
            <h5><i class="fas fa-exclamation-triangle mr-2"></i>Important Notes</h5>
            <ul class="mb-0">
              <li>Detection domains (connectivitycheck.gstatic.com, etc.) are automatically handled</li>
              <li>Users will see "Sign in to network" on their devices</li>
              <li>Customize the portal appearance in <a href="/admin/portal-editor">Portal Editor</a></li>
            </ul>
          </div>
        </div>
      </div>
      
    {:else if activeScenario === 'routes'}
      <!-- Routes Guide -->
      <div class="card">
        <div class="card-header bg-info">
          <h3 class="card-title"><i class="fas fa-route mr-2"></i>Route Rules</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-info-circle mr-2"></i>What are Route Rules?</h5>
            <p>Route rules control how traffic flows through your network. You can allow, deny, or redirect traffic based on source, destination, and other criteria.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-list mr-2 text-info"></i>Rule Types</h5>
          
          <div class="row">
            <div class="col-md-4">
              <div class="info-box bg-success">
                <span class="info-box-icon"><i class="fas fa-check"></i></span>
                <div class="info-box-content">
                  <span class="info-box-text">ALLOW</span>
                  <span class="info-box-number">Permit traffic</span>
                </div>
              </div>
            </div>
            <div class="col-md-4">
              <div class="info-box bg-danger">
                <span class="info-box-icon"><i class="fas fa-ban"></i></span>
                <div class="info-box-content">
                  <span class="info-box-text">DENY</span>
                  <span class="info-box-number">Block traffic</span>
                </div>
              </div>
            </div>
            <div class="col-md-4">
              <div class="info-box bg-warning">
                <span class="info-box-icon"><i class="fas fa-directions"></i></span>
                <div class="info-box-content">
                  <span class="info-box-text">REDIRECT</span>
                  <span class="info-box-number">Send elsewhere</span>
                </div>
              </div>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-code mr-2 text-info"></i>Example Rules</h5>
          
          <table class="table table-bordered">
            <thead class="thead-light">
              <tr>
                <th>Scenario</th>
                <th>Source</th>
                <th>Destination</th>
                <th>Action</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>Block social media</td>
                <td><code>any</code></td>
                <td><code>facebook.com, instagram.com</code></td>
                <td><span class="badge badge-danger">DENY</span></td>
              </tr>
              <tr>
                <td>Allow server room only</td>
                <td><code>192.168.1.0/24</code></td>
                <td><code>10.0.0.0/8</code></td>
                <td><span class="badge badge-success">ALLOW</span></td>
              </tr>
              <tr>
                <td>Redirect HTTP to HTTPS</td>
                <td><code>any:80</code></td>
                <td><code>any:443</code></td>
                <td><span class="badge badge-warning">REDIRECT</span></td>
              </tr>
            </tbody>
          </table>
          
          <div class="callout callout-warning">
            <h5><i class="fas fa-sort-amount-up mr-2"></i>Rule Priority</h5>
            <p class="mb-0">Rules are processed in order from top to bottom. The first matching rule wins. Place more specific rules above general rules.</p>
          </div>
        </div>
      </div>
      
    {:else if activeScenario === 'forwarding'}
      <!-- Forwarding Guide -->
      <div class="card">
        <div class="card-header bg-success">
          <h3 class="card-title"><i class="fas fa-exchange-alt mr-2"></i>Forwarding & NAT</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-info-circle mr-2"></i>What is Forwarding?</h5>
            <p>Forwarding allows traffic to pass between network interfaces. NAT (Network Address Translation) allows devices on a private network to access the internet through a single public IP.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-cogs mr-2 text-success"></i>Configuration</h5>
          
          <div class="row">
            <div class="col-md-6">
              <div class="card card-outline card-success">
                <div class="card-header">
                  <h5 class="card-title mb-0">Basic Setup</h5>
                </div>
                <div class="card-body">
                  <ol>
                    <li><strong>Enable Forwarding:</strong> Toggle ON</li>
                    <li><strong>Enable NAT:</strong> Toggle ON for internet access</li>
                    <li><strong>Uplink Interface:</strong> Your internet connection (e.g., eth0)</li>
                    <li><strong>Downlink Interface:</strong> Your LAN (e.g., wlan0)</li>
                  </ol>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card card-outline card-info">
                <div class="card-header">
                  <h5 class="card-title mb-0">Port Forwarding</h5>
                </div>
                <div class="card-body">
                  <p>Forward external ports to internal servers:</p>
                  <ul>
                    <li>Web server: Port 80 → 192.168.1.10:80</li>
                    <li>Game server: Port 25565 → 192.168.1.20:25565</li>
                    <li>SSH: Port 2222 → 192.168.1.5:22</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <div class="callout callout-danger mt-3">
            <h5><i class="fas fa-exclamation-circle mr-2"></i>Security Warning</h5>
            <p class="mb-0">Port forwarding exposes internal services to the internet. Only forward necessary ports and ensure services are properly secured.</p>
          </div>
        </div>
      </div>
      
    {:else if activeScenario === 'dataflow'}
      <!-- Dataflow Guide -->
      <div class="card">
        <div class="card-header bg-warning">
          <h3 class="card-title text-dark"><i class="fas fa-tachometer-alt mr-2"></i>Dataflow & QoS</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-info-circle mr-2"></i>What is QoS?</h5>
            <p>Quality of Service (QoS) prioritizes certain types of traffic over others, ensuring critical applications get the bandwidth they need.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-sliders-h mr-2 text-warning"></i>QoS Priority Levels</h5>
          
          <table class="table table-bordered">
            <thead class="thead-dark">
              <tr>
                <th>Priority</th>
                <th>Use Case</th>
                <th>Example Traffic</th>
              </tr>
            </thead>
            <tbody>
              <tr class="table-danger">
                <td><strong>Highest</strong></td>
                <td>Real-time communications</td>
                <td>VoIP, Video calls</td>
              </tr>
              <tr class="table-warning">
                <td><strong>High</strong></td>
                <td>Interactive applications</td>
                <td>Gaming, Remote desktop</td>
              </tr>
              <tr class="table-info">
                <td><strong>Normal</strong></td>
                <td>Standard web traffic</td>
                <td>Browsing, Email</td>
              </tr>
              <tr class="table-secondary">
                <td><strong>Low</strong></td>
                <td>Background transfers</td>
                <td>Downloads, Updates</td>
              </tr>
              <tr class="table-light">
                <td><strong>Lowest</strong></td>
                <td>Bulk transfers</td>
                <td>Backups, Torrents</td>
              </tr>
            </tbody>
          </table>
          
          <h5 class="mt-4"><i class="fas fa-chart-line mr-2 text-warning"></i>Bandwidth Limits</h5>
          <p>Set maximum bandwidth for the entire network:</p>
          <ul>
            <li><strong>Download Limit:</strong> Max download speed (Mbps)</li>
            <li><strong>Upload Limit:</strong> Max upload speed (Mbps)</li>
          </ul>
        </div>
      </div>
      
    {:else if activeScenario === 'group-limits'}
      <!-- Group Bandwidth Guide -->
      <div class="card">
        <div class="card-header bg-teal">
          <h3 class="card-title"><i class="fas fa-users-cog mr-2"></i>Group Bandwidth Limits</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-info-circle mr-2"></i>What are Group Limits?</h5>
            <p>Group bandwidth limits allow you to set different speed caps for different user groups. This ensures fair bandwidth distribution across your network.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-sitemap mr-2 text-teal"></i>Typical Setup</h5>
          
          <div class="row">
            <div class="col-md-6">
              <div class="card bg-light">
                <div class="card-header">
                  <h5 class="mb-0"><i class="fas fa-crown text-warning mr-2"></i>Admin Group</h5>
                </div>
                <div class="card-body">
                  <ul class="list-unstyled mb-0">
                    <li><i class="fas fa-download text-success mr-2"></i>Download: <strong>Unlimited</strong></li>
                    <li><i class="fas fa-upload text-info mr-2"></i>Upload: <strong>Unlimited</strong></li>
                    <li><i class="fas fa-star text-warning mr-2"></i>Priority: <strong>Highest</strong></li>
                  </ul>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card bg-light">
                <div class="card-header">
                  <h5 class="mb-0"><i class="fas fa-user text-primary mr-2"></i>Staff Group</h5>
                </div>
                <div class="card-body">
                  <ul class="list-unstyled mb-0">
                    <li><i class="fas fa-download text-success mr-2"></i>Download: <strong>50 Mbps</strong></li>
                    <li><i class="fas fa-upload text-info mr-2"></i>Upload: <strong>20 Mbps</strong></li>
                    <li><i class="fas fa-star text-warning mr-2"></i>Priority: <strong>High</strong></li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <div class="row mt-3">
            <div class="col-md-6">
              <div class="card bg-light">
                <div class="card-header">
                  <h5 class="mb-0"><i class="fas fa-user-friends text-success mr-2"></i>Guest Group</h5>
                </div>
                <div class="card-body">
                  <ul class="list-unstyled mb-0">
                    <li><i class="fas fa-download text-success mr-2"></i>Download: <strong>10 Mbps</strong></li>
                    <li><i class="fas fa-upload text-info mr-2"></i>Upload: <strong>5 Mbps</strong></li>
                    <li><i class="fas fa-star text-warning mr-2"></i>Priority: <strong>Low</strong></li>
                  </ul>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card bg-light">
                <div class="card-header">
                  <h5 class="mb-0"><i class="fas fa-ban text-danger mr-2"></i>Restricted Group</h5>
                </div>
                <div class="card-body">
                  <ul class="list-unstyled mb-0">
                    <li><i class="fas fa-download text-success mr-2"></i>Download: <strong>2 Mbps</strong></li>
                    <li><i class="fas fa-upload text-info mr-2"></i>Upload: <strong>1 Mbps</strong></li>
                    <li><i class="fas fa-star text-warning mr-2"></i>Priority: <strong>Lowest</strong></li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-list-ol mr-2 text-teal"></i>Setup Steps</h5>
          <ol>
            <li>Go to <a href="/admin/groups">Groups</a> and create your user groups</li>
            <li>Open <strong>ACL & Permissions</strong> → <strong>Dataflow / QoS</strong> tab</li>
            <li>In the <strong>Group Bandwidth Limits</strong> section, add limits for each group</li>
            <li>Set download/upload limits and priority for each group</li>
            <li>Save the configuration</li>
          </ol>
          
          <div class="callout callout-success">
            <h5><i class="fas fa-lightbulb mr-2"></i>Pro Tip</h5>
            <p class="mb-0">Users inherit the limits of their group. If a user is in multiple groups, the least restrictive limits apply.</p>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .nav-link {
    border-radius: 0;
  }
  .nav-link:not(.active) {
    color: #333;
  }
  .nav-link:not(.active):hover {
    background-color: #f4f4f4;
  }
  .timeline {
    margin: 0;
    padding: 0;
    position: relative;
  }
  .timeline::before {
    content: '';
    position: absolute;
    top: 0;
    bottom: 0;
    width: 4px;
    background: #ddd;
    left: 31px;
    margin: 0;
    border-radius: 2px;
  }
  .timeline > div {
    margin-bottom: 15px;
    position: relative;
  }
  .timeline > div > .timeline-item {
    margin-left: 60px;
    margin-right: 15px;
    background: #fff;
    border-radius: 3px;
    padding: 10px;
    border: 1px solid #ddd;
  }
  .timeline > div > i {
    width: 30px;
    height: 30px;
    line-height: 30px;
    font-size: 15px;
    text-align: center;
    position: absolute;
    border-radius: 50%;
    left: 18px;
    top: 0;
    color: #fff;
  }
  .time-label > span {
    padding: 5px 10px;
    display: inline-block;
    border-radius: 4px;
    color: #fff;
  }
  .timeline-header {
    font-size: 16px;
    font-weight: 600;
    margin: 0;
    padding: 0 0 10px 0;
    border-bottom: 1px solid #f4f4f4;
  }
  .timeline-body {
    padding-top: 10px;
  }
</style>
