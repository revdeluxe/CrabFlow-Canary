<script>
  let activeSection = 'overview'
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0"><i class="fas fa-shield-alt mr-2"></i>Firewall Guide</h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
      <li class="breadcrumb-item"><a href="/admin/about/guides">Guides</a></li>
      <li class="breadcrumb-item active">Firewall</li>
    </ol>
  </div>
</div>

<div class="row mt-3">
  <!-- Sidebar -->
  <div class="col-md-3">
    <div class="card card-outline card-danger">
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
            <button class="nav-link text-left w-100 {activeSection === 'rules' ? 'active' : ''}"
                    on:click={() => activeSection = 'rules'}>
              <i class="fas fa-list-alt mr-2"></i> Creating Rules
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'scenarios' ? 'active' : ''}"
                    on:click={() => activeSection = 'scenarios'}>
              <i class="fas fa-lightbulb mr-2"></i> Common Scenarios
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'best-practices' ? 'active' : ''}"
                    on:click={() => activeSection = 'best-practices'}>
              <i class="fas fa-medal mr-2"></i> Best Practices
            </button>
          </li>
        </ul>
      </div>
    </div>
    
    <a href="/admin/firewall" class="btn btn-danger btn-block mt-3">
      <i class="fas fa-cog mr-2"></i> Open Firewall Settings
    </a>
  </div>
  
  <!-- Content -->
  <div class="col-md-9">
    {#if activeSection === 'overview'}
      <div class="card">
        <div class="card-header bg-danger">
          <h3 class="card-title"><i class="fas fa-shield-alt mr-2"></i>Firewall Overview</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-question-circle mr-2"></i>What is a Firewall?</h5>
            <p>A firewall monitors and controls incoming and outgoing network traffic based on predetermined security rules. It acts as a barrier between trusted and untrusted networks.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-filter mr-2 text-danger"></i>Rule Actions</h5>
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
                  <span class="info-box-number">Block silently</span>
                </div>
              </div>
            </div>
            <div class="col-md-4">
              <div class="info-box bg-warning">
                <span class="info-box-icon"><i class="fas fa-times"></i></span>
                <div class="info-box-content">
                  <span class="info-box-text">REJECT</span>
                  <span class="info-box-number">Block with response</span>
                </div>
              </div>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-arrows-alt mr-2 text-danger"></i>Traffic Directions</h5>
          <table class="table table-bordered">
            <tr>
              <td class="bg-primary text-white"><strong>INPUT</strong></td>
              <td>Traffic coming INTO CrabFlow (e.g., accessing admin panel)</td>
            </tr>
            <tr>
              <td class="bg-success text-white"><strong>OUTPUT</strong></td>
              <td>Traffic going OUT from CrabFlow (e.g., DNS queries)</td>
            </tr>
            <tr>
              <td class="bg-info text-white"><strong>FORWARD</strong></td>
              <td>Traffic passing THROUGH CrabFlow (e.g., LAN to Internet)</td>
            </tr>
          </table>
        </div>
      </div>
      
    {:else if activeSection === 'rules'}
      <div class="card">
        <div class="card-header bg-primary">
          <h3 class="card-title"><i class="fas fa-list-alt mr-2"></i>Creating Firewall Rules</h3>
        </div>
        <div class="card-body">
          <h5><i class="fas fa-puzzle-piece mr-2 text-primary"></i>Rule Components</h5>
          
          <table class="table table-bordered">
            <thead class="thead-dark">
              <tr>
                <th>Component</th>
                <th>Description</th>
                <th>Examples</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td><strong>Source</strong></td>
                <td>Where traffic originates</td>
                <td><code>any</code>, <code>192.168.1.0/24</code>, <code>10.0.0.5</code></td>
              </tr>
              <tr>
                <td><strong>Destination</strong></td>
                <td>Where traffic is going</td>
                <td><code>any</code>, <code>8.8.8.8</code>, <code>facebook.com</code></td>
              </tr>
              <tr>
                <td><strong>Protocol</strong></td>
                <td>Network protocol</td>
                <td><code>TCP</code>, <code>UDP</code>, <code>ICMP</code>, <code>any</code></td>
              </tr>
              <tr>
                <td><strong>Port</strong></td>
                <td>Service port number</td>
                <td><code>80</code>, <code>443</code>, <code>22</code>, <code>any</code></td>
              </tr>
              <tr>
                <td><strong>Action</strong></td>
                <td>What to do with traffic</td>
                <td><code>ALLOW</code>, <code>DENY</code>, <code>REJECT</code></td>
              </tr>
            </tbody>
          </table>
          
          <h5 class="mt-4"><i class="fas fa-plus-circle mr-2 text-primary"></i>Adding a Rule</h5>
          <ol>
            <li>Go to <strong>Firewall</strong> page</li>
            <li>Click <strong>Add Rule</strong></li>
            <li>Select the <strong>chain</strong> (INPUT, OUTPUT, FORWARD)</li>
            <li>Configure source, destination, protocol, port</li>
            <li>Choose the <strong>action</strong></li>
            <li>Add a descriptive <strong>name</strong></li>
            <li>Set <strong>priority</strong> (lower = processed first)</li>
            <li>Save the rule</li>
          </ol>
          
          <div class="callout callout-warning">
            <h5><i class="fas fa-sort-amount-up mr-2"></i>Rule Order Matters!</h5>
            <p class="mb-0">Rules are processed in priority order. The first matching rule wins. Always put more specific rules before general rules.</p>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'scenarios'}
      <div class="card">
        <div class="card-header bg-info">
          <h3 class="card-title"><i class="fas fa-lightbulb mr-2"></i>Common Scenarios</h3>
        </div>
        <div class="card-body">
          <!-- Scenario 1 -->
          <div class="card card-outline card-danger mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-ban mr-2"></i>Block Social Media</h5>
            </div>
            <div class="card-body">
              <p>Prevent access to social media sites during work hours:</p>
              <table class="table table-sm table-bordered">
                <tr><td>Chain</td><td>FORWARD</td></tr>
                <tr><td>Source</td><td>192.168.1.0/24 (your LAN)</td></tr>
                <tr><td>Destination</td><td>facebook.com, instagram.com, twitter.com</td></tr>
                <tr><td>Action</td><td><span class="badge badge-danger">DENY</span></td></tr>
              </table>
            </div>
          </div>
          
          <!-- Scenario 2 -->
          <div class="card card-outline card-success mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-server mr-2"></i>Allow Web Server Access</h5>
            </div>
            <div class="card-body">
              <p>Allow external access to your web server:</p>
              <table class="table table-sm table-bordered">
                <tr><td>Chain</td><td>INPUT</td></tr>
                <tr><td>Source</td><td>any</td></tr>
                <tr><td>Destination</td><td>192.168.1.10 (web server)</td></tr>
                <tr><td>Protocol</td><td>TCP</td></tr>
                <tr><td>Port</td><td>80, 443</td></tr>
                <tr><td>Action</td><td><span class="badge badge-success">ALLOW</span></td></tr>
              </table>
            </div>
          </div>
          
          <!-- Scenario 3 -->
          <div class="card card-outline card-warning mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-user-shield mr-2"></i>Isolate Guest Network</h5>
            </div>
            <div class="card-body">
              <p>Prevent guests from accessing your main network:</p>
              <table class="table table-sm table-bordered">
                <tr><td>Chain</td><td>FORWARD</td></tr>
                <tr><td>Source</td><td>192.168.10.0/24 (guest network)</td></tr>
                <tr><td>Destination</td><td>192.168.1.0/24 (main network)</td></tr>
                <tr><td>Action</td><td><span class="badge badge-danger">DENY</span></td></tr>
              </table>
            </div>
          </div>
          
          <!-- Scenario 4 -->
          <div class="card card-outline card-info mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-gamepad mr-2"></i>Port Forward for Gaming</h5>
            </div>
            <div class="card-body">
              <p>Forward game server ports to your gaming PC:</p>
              <table class="table table-sm table-bordered">
                <tr><td>Chain</td><td>FORWARD</td></tr>
                <tr><td>Source</td><td>any</td></tr>
                <tr><td>Destination Port</td><td>25565 (Minecraft)</td></tr>
                <tr><td>Forward To</td><td>192.168.1.50:25565</td></tr>
                <tr><td>Action</td><td><span class="badge badge-success">ALLOW + DNAT</span></td></tr>
              </table>
            </div>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'best-practices'}
      <div class="card">
        <div class="card-header bg-success">
          <h3 class="card-title"><i class="fas fa-medal mr-2"></i>Best Practices</h3>
        </div>
        <div class="card-body">
          <div class="row">
            <div class="col-md-6">
              <div class="card bg-light">
                <div class="card-header">
                  <i class="fas fa-check text-success mr-2"></i> DO
                </div>
                <div class="card-body">
                  <ul class="mb-0">
                    <li>Start with a "deny all" policy</li>
                    <li>Only allow necessary traffic</li>
                    <li>Use specific rules over broad ones</li>
                    <li>Document your rules with comments</li>
                    <li>Regularly review and audit rules</li>
                    <li>Test rules in a safe environment first</li>
                    <li>Log blocked traffic for analysis</li>
                  </ul>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card bg-light">
                <div class="card-header">
                  <i class="fas fa-times text-danger mr-2"></i> DON'T
                </div>
                <div class="card-body">
                  <ul class="mb-0">
                    <li>Don't use "allow all" rules</li>
                    <li>Don't expose management interfaces</li>
                    <li>Don't forget to allow established connections</li>
                    <li>Don't create overlapping rules</li>
                    <li>Don't ignore firewall logs</li>
                    <li>Don't lock yourself out!</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <div class="callout callout-danger mt-4">
            <h5><i class="fas fa-exclamation-circle mr-2"></i>Warning: Don't Lock Yourself Out!</h5>
            <p class="mb-0">Always ensure you have a rule allowing access to the CrabFlow admin interface before enabling strict firewall rules. If locked out, you may need physical access to reset.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-layer-group mr-2 text-success"></i>Recommended Rule Order</h5>
          <ol>
            <li><strong>Anti-spoofing rules</strong> - Block invalid source IPs</li>
            <li><strong>Allow established/related</strong> - Permit response traffic</li>
            <li><strong>Allow essential services</strong> - DNS, DHCP, NTP</li>
            <li><strong>Allow management</strong> - SSH, admin interface</li>
            <li><strong>Application-specific rules</strong> - Your custom rules</li>
            <li><strong>Default deny</strong> - Block everything else</li>
          </ol>
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
