<script>
  let activeSection = 'overview'
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0"><i class="fas fa-server mr-2"></i>DHCP Server Guide</h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
      <li class="breadcrumb-item"><a href="/admin/about/guides">Guides</a></li>
      <li class="breadcrumb-item active">DHCP</li>
    </ol>
  </div>
</div>

<div class="row mt-3">
  <!-- Sidebar -->
  <div class="col-md-3">
    <div class="card card-outline card-success">
      <div class="card-header">
        <h3 class="card-title">Sections</h3>
      </div>
      <div class="card-body p-0">
        <ul class="nav nav-pills flex-column">
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'overview' ? 'active' : ''}"
                    on:click={() => activeSection = 'overview'}>
              <i class="fas fa-info-circle mr-2"></i> Overview
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'pools' ? 'active' : ''}"
                    on:click={() => activeSection = 'pools'}>
              <i class="fas fa-layer-group mr-2"></i> IP Pools
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'static' ? 'active' : ''}"
                    on:click={() => activeSection = 'static'}>
              <i class="fas fa-thumbtack mr-2"></i> Static Leases
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'options' ? 'active' : ''}"
                    on:click={() => activeSection = 'options'}>
              <i class="fas fa-sliders-h mr-2"></i> DHCP Options
            </button>
          </li>
        </ul>
      </div>
    </div>
    
    <a href="/admin/dhcp" class="btn btn-success btn-block mt-3">
      <i class="fas fa-cog mr-2"></i> Open DHCP Settings
    </a>
  </div>
  
  <!-- Content -->
  <div class="col-md-9">
    {#if activeSection === 'overview'}
      <div class="card">
        <div class="card-header bg-success">
          <h3 class="card-title"><i class="fas fa-server mr-2"></i>DHCP Overview</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-question-circle mr-2"></i>What is DHCP?</h5>
            <p>DHCP (Dynamic Host Configuration Protocol) automatically assigns IP addresses to devices on your network. Without DHCP, you'd need to manually configure each device.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-exchange-alt mr-2 text-success"></i>How DHCP Works</h5>
          <div class="row text-center">
            <div class="col-md-3">
              <div class="card bg-light">
                <div class="card-body">
                  <i class="fas fa-laptop fa-2x text-primary mb-2"></i>
                  <h6>1. Discover</h6>
                  <small>Device broadcasts "I need an IP"</small>
                </div>
              </div>
            </div>
            <div class="col-md-3">
              <div class="card bg-light">
                <div class="card-body">
                  <i class="fas fa-server fa-2x text-success mb-2"></i>
                  <h6>2. Offer</h6>
                  <small>Server offers available IP</small>
                </div>
              </div>
            </div>
            <div class="col-md-3">
              <div class="card bg-light">
                <div class="card-body">
                  <i class="fas fa-hand-paper fa-2x text-warning mb-2"></i>
                  <h6>3. Request</h6>
                  <small>Device accepts offer</small>
                </div>
              </div>
            </div>
            <div class="col-md-3">
              <div class="card bg-light">
                <div class="card-body">
                  <i class="fas fa-check-circle fa-2x text-info mb-2"></i>
                  <h6>4. Acknowledge</h6>
                  <small>Server confirms lease</small>
                </div>
              </div>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-play-circle mr-2 text-success"></i>Quick Start</h5>
          <ol>
            <li>Go to <strong>DHCP</strong> settings</li>
            <li>Enable DHCP server</li>
            <li>Set IP range (e.g., 192.168.1.100 - 192.168.1.200)</li>
            <li>Configure gateway and DNS</li>
            <li>Save and restart service</li>
          </ol>
          
          <div class="callout callout-warning">
            <h5><i class="fas fa-exclamation-triangle mr-2"></i>Important</h5>
            <p class="mb-0">Only run one DHCP server per network segment. Disable DHCP on your router if using CrabFlow's DHCP.</p>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'pools'}
      <div class="card">
        <div class="card-header bg-info">
          <h3 class="card-title"><i class="fas fa-layer-group mr-2"></i>IP Address Pools</h3>
        </div>
        <div class="card-body">
          <p>IP pools define the range of addresses that DHCP can assign to devices.</p>
          
          <h5><i class="fas fa-calculator mr-2 text-info"></i>Planning Your IP Range</h5>
          <table class="table table-bordered">
            <thead class="thead-dark">
              <tr>
                <th>Network Size</th>
                <th>Subnet</th>
                <th>Suggested Range</th>
                <th>Available IPs</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>Small Home</td>
                <td>192.168.1.0/24</td>
                <td>.100 - .200</td>
                <td>~100 devices</td>
              </tr>
              <tr>
                <td>Medium Office</td>
                <td>10.0.0.0/24</td>
                <td>.50 - .250</td>
                <td>~200 devices</td>
              </tr>
              <tr>
                <td>Large Network</td>
                <td>172.16.0.0/16</td>
                <td>.0.100 - .255.250</td>
                <td>~65,000 devices</td>
              </tr>
            </tbody>
          </table>
          
          <div class="callout callout-info">
            <h5><i class="fas fa-lightbulb mr-2"></i>Best Practice</h5>
            <p class="mb-0">Reserve the first 50-100 IPs for static assignments (servers, printers, etc.) and use the rest for DHCP.</p>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'static'}
      <div class="card">
        <div class="card-header bg-warning">
          <h3 class="card-title text-dark"><i class="fas fa-thumbtack mr-2"></i>Static Leases (Reservations)</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-question-circle mr-2"></i>What are Static Leases?</h5>
            <p>Static leases (also called reservations) ensure a specific device always gets the same IP address based on its MAC address.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-server mr-2 text-warning"></i>When to Use Static Leases</h5>
          <div class="row">
            <div class="col-md-6">
              <ul class="list-group">
                <li class="list-group-item">
                  <i class="fas fa-print text-info mr-2"></i>
                  <strong>Printers</strong> - Consistent address for printing
                </li>
                <li class="list-group-item">
                  <i class="fas fa-server text-success mr-2"></i>
                  <strong>Servers</strong> - NAS, media servers, etc.
                </li>
                <li class="list-group-item">
                  <i class="fas fa-camera text-danger mr-2"></i>
                  <strong>Security cameras</strong> - Fixed IP for monitoring
                </li>
              </ul>
            </div>
            <div class="col-md-6">
              <ul class="list-group">
                <li class="list-group-item">
                  <i class="fas fa-home text-warning mr-2"></i>
                  <strong>Smart home hubs</strong> - Reliable connections
                </li>
                <li class="list-group-item">
                  <i class="fas fa-gamepad text-purple mr-2"></i>
                  <strong>Gaming consoles</strong> - Port forwarding
                </li>
                <li class="list-group-item">
                  <i class="fas fa-desktop text-secondary mr-2"></i>
                  <strong>Workstations</strong> - Remote access
                </li>
              </ul>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-plus-circle mr-2 text-warning"></i>Creating a Static Lease</h5>
          <ol>
            <li>Go to <strong>DHCP</strong> → <strong>Leases</strong> tab</li>
            <li>Find the device in current leases, or click <strong>Add Static Lease</strong></li>
            <li>Enter the device's <strong>MAC address</strong></li>
            <li>Choose an <strong>IP address</strong> (preferably outside DHCP pool)</li>
            <li>Add a <strong>hostname</strong> for easy identification</li>
            <li>Save the reservation</li>
          </ol>
        </div>
      </div>
      
    {:else if activeSection === 'options'}
      <div class="card">
        <div class="card-header bg-primary">
          <h3 class="card-title"><i class="fas fa-sliders-h mr-2"></i>DHCP Options</h3>
        </div>
        <div class="card-body">
          <p>DHCP options provide additional configuration to clients beyond just an IP address.</p>
          
          <table class="table table-bordered">
            <thead class="thead-dark">
              <tr>
                <th>Option</th>
                <th>Code</th>
                <th>Description</th>
                <th>Example</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td><strong>Subnet Mask</strong></td>
                <td>1</td>
                <td>Network mask</td>
                <td>255.255.255.0</td>
              </tr>
              <tr>
                <td><strong>Router/Gateway</strong></td>
                <td>3</td>
                <td>Default gateway IP</td>
                <td>192.168.1.1</td>
              </tr>
              <tr>
                <td><strong>DNS Servers</strong></td>
                <td>6</td>
                <td>DNS server IPs</td>
                <td>192.168.1.1, 8.8.8.8</td>
              </tr>
              <tr>
                <td><strong>Domain Name</strong></td>
                <td>15</td>
                <td>Local domain suffix</td>
                <td>home.local</td>
              </tr>
              <tr>
                <td><strong>Lease Time</strong></td>
                <td>51</td>
                <td>How long lease is valid</td>
                <td>86400 (24 hours)</td>
              </tr>
              <tr>
                <td><strong>NTP Server</strong></td>
                <td>42</td>
                <td>Time server</td>
                <td>pool.ntp.org</td>
              </tr>
            </tbody>
          </table>
          
          <div class="callout callout-success">
            <h5><i class="fas fa-magic mr-2"></i>Auto-Configuration</h5>
            <p class="mb-0">CrabFlow automatically sets most options based on your network configuration. You only need to customize if you have special requirements.</p>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .nav-link:not(.active) {
    color: #333;
  }
  .nav-link:not(.active):hover {
    background-color: #f4f4f4;
  }
</style>
