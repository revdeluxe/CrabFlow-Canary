<script>
  let currentStep = 1
  const totalSteps = 5
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0"><i class="fas fa-rocket mr-2"></i>Getting Started</h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
      <li class="breadcrumb-item"><a href="/admin/about/guides">Guides</a></li>
      <li class="breadcrumb-item active">Getting Started</li>
    </ol>
  </div>
</div>

<!-- Progress Bar -->
<div class="card mt-3">
  <div class="card-body">
    <h5 class="mb-3">Setup Progress</h5>
    <div class="progress" style="height: 25px;">
      <div class="progress-bar bg-primary" role="progressbar" 
           style="width: {(currentStep / totalSteps) * 100}%">
        Step {currentStep} of {totalSteps}
      </div>
    </div>
    <div class="d-flex justify-content-between mt-2">
      {#each Array(totalSteps) as _, i}
        <button class="btn btn-sm {currentStep === i + 1 ? 'btn-primary' : currentStep > i + 1 ? 'btn-success' : 'btn-outline-secondary'}"
                on:click={() => currentStep = i + 1}>
          {i + 1}
        </button>
      {/each}
    </div>
  </div>
</div>

<div class="row mt-3">
  <div class="col-12">
    {#if currentStep === 1}
      <!-- Step 1: Welcome -->
      <div class="card card-primary">
        <div class="card-header">
          <h3 class="card-title"><i class="fas fa-star mr-2"></i>Welcome to CrabFlow</h3>
        </div>
        <div class="card-body">
          <div class="row">
            <div class="col-md-6">
              <h4>What is CrabFlow?</h4>
              <p>CrabFlow is a comprehensive network management system that provides:</p>
              <ul>
                <li><i class="fas fa-check text-success mr-2"></i><strong>DHCP Server:</strong> Automatic IP assignment</li>
                <li><i class="fas fa-check text-success mr-2"></i><strong>DNS Server:</strong> Name resolution with ad-blocking</li>
                <li><i class="fas fa-check text-success mr-2"></i><strong>Firewall:</strong> Network security rules</li>
                <li><i class="fas fa-check text-success mr-2"></i><strong>Captive Portal:</strong> Guest authentication</li>
                <li><i class="fas fa-check text-success mr-2"></i><strong>User Management:</strong> Access control</li>
                <li><i class="fas fa-check text-success mr-2"></i><strong>Bandwidth Control:</strong> QoS and limits</li>
              </ul>
            </div>
            <div class="col-md-6">
              <div class="card bg-light">
                <div class="card-body text-center">
                  <i class="fas fa-network-wired fa-5x text-primary mb-3"></i>
                  <h5>Your Network, Your Control</h5>
                  <p class="text-muted">CrabFlow gives you complete visibility and control over your network infrastructure.</p>
                </div>
              </div>
            </div>
          </div>
          
          <div class="callout callout-info mt-3">
            <h5><i class="fas fa-info-circle mr-2"></i>Before You Begin</h5>
            <p class="mb-0">Make sure you have administrator access to your network equipment and know your network topology.</p>
          </div>
        </div>
        <div class="card-footer">
          <button class="btn btn-primary float-right" on:click={() => currentStep++}>
            Next: System Requirements <i class="fas fa-arrow-right ml-2"></i>
          </button>
        </div>
      </div>
      
    {:else if currentStep === 2}
      <!-- Step 2: Requirements -->
      <div class="card card-info">
        <div class="card-header">
          <h3 class="card-title"><i class="fas fa-clipboard-list mr-2"></i>System Requirements</h3>
        </div>
        <div class="card-body">
          <div class="row">
            <div class="col-md-6">
              <h5><i class="fas fa-desktop mr-2 text-info"></i>Hardware</h5>
              <table class="table table-sm">
                <tbody>
                  <tr>
                    <td><strong>CPU</strong></td>
                    <td>Dual-core 1GHz+</td>
                  </tr>
                  <tr>
                    <td><strong>RAM</strong></td>
                    <td>2GB minimum, 4GB recommended</td>
                  </tr>
                  <tr>
                    <td><strong>Storage</strong></td>
                    <td>500MB for application</td>
                  </tr>
                  <tr>
                    <td><strong>Network</strong></td>
                    <td>2+ network interfaces recommended</td>
                  </tr>
                </tbody>
              </table>
            </div>
            <div class="col-md-6">
              <h5><i class="fas fa-laptop-code mr-2 text-info"></i>Software</h5>
              <table class="table table-sm">
                <tbody>
                  <tr>
                    <td><strong>OS</strong></td>
                    <td>Windows 10/11, Linux, macOS</td>
                  </tr>
                  <tr>
                    <td><strong>Browser</strong></td>
                    <td>Chrome, Firefox, Edge (latest)</td>
                  </tr>
                  <tr>
                    <td><strong>Permissions</strong></td>
                    <td>Administrator/root access</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-network-wired mr-2 text-info"></i>Network Topology</h5>
          <div class="row">
            <div class="col-md-4">
              <div class="card card-outline card-success">
                <div class="card-header">
                  <h6 class="mb-0">Simple Setup</h6>
                </div>
                <div class="card-body text-center">
                  <p class="small">Internet → CrabFlow → Devices</p>
                  <span class="badge badge-success">Single Interface</span>
                </div>
              </div>
            </div>
            <div class="col-md-4">
              <div class="card card-outline card-primary">
                <div class="card-header">
                  <h6 class="mb-0">Router Mode</h6>
                </div>
                <div class="card-body text-center">
                  <p class="small">Internet → CrabFlow → Switch → Devices</p>
                  <span class="badge badge-primary">Two Interfaces</span>
                </div>
              </div>
            </div>
            <div class="col-md-4">
              <div class="card card-outline card-warning">
                <div class="card-header">
                  <h6 class="mb-0">Advanced</h6>
                </div>
                <div class="card-body text-center">
                  <p class="small">Multiple VLANs, WiFi AP</p>
                  <span class="badge badge-warning">Multiple Interfaces</span>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="card-footer">
          <button class="btn btn-secondary" on:click={() => currentStep--}>
            <i class="fas fa-arrow-left mr-2"></i> Back
          </button>
          <button class="btn btn-primary float-right" on:click={() => currentStep++}>
            Next: Network Setup <i class="fas fa-arrow-right ml-2"></i>
          </button>
        </div>
      </div>
      
    {:else if currentStep === 3}
      <!-- Step 3: Network Configuration -->
      <div class="card card-success">
        <div class="card-header">
          <h3 class="card-title"><i class="fas fa-ethernet mr-2"></i>Network Configuration</h3>
        </div>
        <div class="card-body">
          <h5>Configure Your Interfaces</h5>
          <p>CrabFlow needs to know which network interface to use:</p>
          
          <div class="row">
            <div class="col-md-6">
              <div class="card">
                <div class="card-header bg-success text-white">
                  <strong>Uplink (WAN)</strong>
                </div>
                <div class="card-body">
                  <p>Your internet connection:</p>
                  <ul>
                    <li>Connected to your ISP modem/router</li>
                    <li>Has public or upstream IP</li>
                    <li>Example: <code>eth0</code>, <code>Ethernet</code></li>
                  </ul>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card">
                <div class="card-header bg-info text-white">
                  <strong>Downlink (LAN)</strong>
                </div>
                <div class="card-body">
                  <p>Your local network:</p>
                  <ul>
                    <li>Connected to your devices/switch</li>
                    <li>CrabFlow assigns IPs here</li>
                    <li>Example: <code>eth1</code>, <code>wlan0</code></li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <div class="callout callout-warning mt-3">
            <h5><i class="fas fa-exclamation-triangle mr-2"></i>Single Interface Mode</h5>
            <p class="mb-0">If you only have one interface, CrabFlow can still work but won't route traffic between networks. DHCP and DNS will still function.</p>
          </div>
          
          <h5 class="mt-4">Quick Setup</h5>
          <ol>
            <li>Go to <a href="/admin/settings">Settings</a></li>
            <li>Select your network interfaces</li>
            <li>Configure IP ranges for your LAN</li>
            <li>Save and apply changes</li>
          </ol>
        </div>
        <div class="card-footer">
          <button class="btn btn-secondary" on:click={() => currentStep--}>
            <i class="fas fa-arrow-left mr-2"></i> Back
          </button>
          <button class="btn btn-primary float-right" on:click={() => currentStep++}>
            Next: Services Setup <i class="fas fa-arrow-right ml-2"></i>
          </button>
        </div>
      </div>
      
    {:else if currentStep === 4}
      <!-- Step 4: Services -->
      <div class="card card-warning">
        <div class="card-header">
          <h3 class="card-title text-dark"><i class="fas fa-cogs mr-2"></i>Enable Services</h3>
        </div>
        <div class="card-body">
          <p>Enable the services you need. You can always change these later.</p>
          
          <div class="row">
            <div class="col-md-6">
              <div class="card card-outline card-success">
                <div class="card-body">
                  <h5><i class="fas fa-server text-success mr-2"></i>DHCP Server</h5>
                  <p class="small text-muted">Automatically assigns IP addresses to devices</p>
                  <a href="/admin/dhcp" class="btn btn-sm btn-outline-success">Configure DHCP</a>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card card-outline card-info">
                <div class="card-body">
                  <h5><i class="fas fa-globe text-info mr-2"></i>DNS Server</h5>
                  <p class="small text-muted">Resolves domain names, blocks ads</p>
                  <a href="/admin/dns" class="btn btn-sm btn-outline-info">Configure DNS</a>
                </div>
              </div>
            </div>
          </div>
          
          <div class="row mt-3">
            <div class="col-md-6">
              <div class="card card-outline card-danger">
                <div class="card-body">
                  <h5><i class="fas fa-shield-alt text-danger mr-2"></i>Firewall</h5>
                  <p class="small text-muted">Protect your network with rules</p>
                  <a href="/admin/firewall" class="btn btn-sm btn-outline-danger">Configure Firewall</a>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card card-outline card-secondary">
                <div class="card-body">
                  <h5><i class="fas fa-user-lock text-secondary mr-2"></i>Captive Portal</h5>
                  <p class="small text-muted">Guest WiFi with login page</p>
                  <a href="/admin/acl" class="btn btn-sm btn-outline-secondary">Configure Portal</a>
                </div>
              </div>
            </div>
          </div>
          
          <div class="callout callout-info mt-3">
            <h5><i class="fas fa-lightbulb mr-2"></i>Recommended Order</h5>
            <p class="mb-0">
              <span class="badge badge-success">1. DHCP</span> →
              <span class="badge badge-info">2. DNS</span> →
              <span class="badge badge-danger">3. Firewall</span> →
              <span class="badge badge-secondary">4. Captive Portal</span>
            </p>
          </div>
        </div>
        <div class="card-footer">
          <button class="btn btn-secondary" on:click={() => currentStep--}>
            <i class="fas fa-arrow-left mr-2"></i> Back
          </button>
          <button class="btn btn-primary float-right" on:click={() => currentStep++}>
            Next: All Done! <i class="fas fa-arrow-right ml-2"></i>
          </button>
        </div>
      </div>
      
    {:else if currentStep === 5}
      <!-- Step 5: Complete -->
      <div class="card card-primary">
        <div class="card-header">
          <h3 class="card-title"><i class="fas fa-check-circle mr-2"></i>Setup Complete!</h3>
        </div>
        <div class="card-body text-center">
          <i class="fas fa-trophy fa-5x text-warning mb-4"></i>
          <h3>You're All Set!</h3>
          <p class="lead">CrabFlow is ready to manage your network.</p>
          
          <div class="row justify-content-center mt-4">
            <div class="col-md-8">
              <div class="card bg-light">
                <div class="card-body">
                  <h5>What's Next?</h5>
                  <div class="list-group list-group-flush text-left">
                    <a href="/admin/dashboard" class="list-group-item list-group-item-action">
                      <i class="fas fa-tachometer-alt text-primary mr-2"></i>
                      <strong>Dashboard</strong> - Monitor your network
                    </a>
                    <a href="/admin/users" class="list-group-item list-group-item-action">
                      <i class="fas fa-users text-success mr-2"></i>
                      <strong>Users</strong> - Add network users
                    </a>
                    <a href="/admin/groups" class="list-group-item list-group-item-action">
                      <i class="fas fa-user-friends text-info mr-2"></i>
                      <strong>Groups</strong> - Organize users into groups
                    </a>
                    <a href="/admin/about/guides" class="list-group-item list-group-item-action">
                      <i class="fas fa-book text-warning mr-2"></i>
                      <strong>Guides</strong> - Learn more features
                    </a>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="card-footer">
          <button class="btn btn-secondary" on:click={() => currentStep--}>
            <i class="fas fa-arrow-left mr-2"></i> Back
          </button>
          <a href="/admin/dashboard" class="btn btn-success float-right">
            <i class="fas fa-home mr-2"></i> Go to Dashboard
          </a>
        </div>
      </div>
    {/if}
  </div>
</div>
