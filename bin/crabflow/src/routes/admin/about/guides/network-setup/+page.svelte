<script>
  let activeSection = 'topology'
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0"><i class="fas fa-network-wired mr-2"></i>Network Setup Guide</h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
      <li class="breadcrumb-item"><a href="/admin/about/guides">Guides</a></li>
      <li class="breadcrumb-item active">Network Setup</li>
    </ol>
  </div>
</div>

<div class="row mt-3">
  <!-- Sidebar -->
  <div class="col-md-3">
    <div class="card card-outline card-info">
      <div class="card-header">
        <h3 class="card-title">Sections</h3>
      </div>
      <div class="card-body p-0">
        <ul class="nav nav-pills flex-column">
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'topology' ? 'active' : ''}"
                    on:click={() => activeSection = 'topology'}>
              <i class="fas fa-project-diagram mr-2"></i> Network Topology
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'interfaces' ? 'active' : ''}"
                    on:click={() => activeSection = 'interfaces'}>
              <i class="fas fa-ethernet mr-2"></i> Interfaces
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'routing' ? 'active' : ''}"
                    on:click={() => activeSection = 'routing'}>
              <i class="fas fa-route mr-2"></i> Routing & NAT
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'wifi' ? 'active' : ''}"
                    on:click={() => activeSection = 'wifi'}>
              <i class="fas fa-wifi mr-2"></i> WiFi Hotspot
            </button>
          </li>
        </ul>
      </div>
    </div>
    
    <a href="/admin/settings" class="btn btn-info btn-block mt-3">
      <i class="fas fa-cog mr-2"></i> Network Settings
    </a>
  </div>
  
  <!-- Content -->
  <div class="col-md-9">
    {#if activeSection === 'topology'}
      <div class="card">
        <div class="card-header bg-info">
          <h3 class="card-title"><i class="fas fa-project-diagram mr-2"></i>Network Topology</h3>
        </div>
        <div class="card-body">
          <p>Choose a topology that matches your setup:</p>
          
          <!-- Topology 1: Simple -->
          <div class="card card-outline card-success mb-4">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-laptop mr-2"></i>Simple Mode (Single Interface)</h5>
            </div>
            <div class="card-body">
              <div class="row">
                <div class="col-md-6">
                  <pre class="bg-light p-3">
┌─────────────┐
│   Internet  │
└──────┬──────┘
       │
┌──────┴──────┐
│   Router    │
└──────┬──────┘
       │
┌──────┴──────┐
│  CrabFlow   │ (DHCP/DNS server)
└──────┬──────┘
       │
┌──────┴──────┐
│   Switch    │
└──────┬──────┘
       │
   [Devices]</pre>
                </div>
                <div class="col-md-6">
                  <h6>Use When:</h6>
                  <ul>
                    <li>CrabFlow runs on your network</li>
                    <li>You only need DHCP/DNS services</li>
                    <li>Your router handles NAT/routing</li>
                  </ul>
                  <h6>Configuration:</h6>
                  <ul>
                    <li>Disable DHCP on your router</li>
                    <li>Point router DNS to CrabFlow</li>
                    <li>CrabFlow provides DHCP/DNS</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Topology 2: Router Mode -->
          <div class="card card-outline card-primary mb-4">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-server mr-2"></i>Router Mode (Two Interfaces)</h5>
            </div>
            <div class="card-body">
              <div class="row">
                <div class="col-md-6">
                  <pre class="bg-light p-3">
┌─────────────┐
│   Internet  │
└──────┬──────┘
       │
┌──────┴──────┐
│    Modem    │
└──────┬──────┘
       │ (WAN - eth0)
┌──────┴──────┐
│  CrabFlow   │ (Router + Services)
└──────┬──────┘
       │ (LAN - eth1)
┌──────┴──────┐
│   Switch    │
└──────┬──────┘
       │
   [Devices]</pre>
                </div>
                <div class="col-md-6">
                  <h6>Use When:</h6>
                  <ul>
                    <li>CrabFlow replaces your router</li>
                    <li>Full traffic control needed</li>
                    <li>Captive portal required</li>
                    <li>Advanced firewall rules</li>
                  </ul>
                  <h6>Configuration:</h6>
                  <ul>
                    <li>WAN: Connected to modem/ISP</li>
                    <li>LAN: Connected to switch</li>
                    <li>Enable NAT & routing</li>
                    <li>Enable all services</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Topology 3: WiFi AP -->
          <div class="card card-outline card-warning mb-4">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-wifi mr-2"></i>WiFi Access Point Mode</h5>
            </div>
            <div class="card-body">
              <div class="row">
                <div class="col-md-6">
                  <pre class="bg-light p-3">
┌─────────────┐
│   Internet  │
└──────┬──────┘
       │
┌──────┴──────┐
│   Router    │
└──────┬──────┘
       │ (WAN - eth0)
┌──────┴──────┐
│  CrabFlow   │ → WiFi (wlan0)
└─────────────┘      │
                     │
              [WiFi Devices]</pre>
                </div>
                <div class="col-md-6">
                  <h6>Use When:</h6>
                  <ul>
                    <li>Adding WiFi to wired network</li>
                    <li>Guest WiFi with captive portal</li>
                    <li>Separate WiFi network</li>
                  </ul>
                  <h6>Configuration:</h6>
                  <ul>
                    <li>WAN: Connected to existing network</li>
                    <li>WiFi: CrabFlow creates hotspot</li>
                    <li>Enable captive portal</li>
                    <li>Optionally isolate WiFi from LAN</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'interfaces'}
      <div class="card">
        <div class="card-header bg-success">
          <h3 class="card-title"><i class="fas fa-ethernet mr-2"></i>Network Interfaces</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-info-circle mr-2"></i>Interface Types</h5>
            <p>CrabFlow can work with various interface types: Ethernet (eth), WiFi (wlan), Virtual (veth), and VLANs.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-plug mr-2 text-success"></i>Identifying Interfaces</h5>
          <p>Go to <strong>Monitor</strong> to see available interfaces:</p>
          
          <table class="table table-bordered">
            <thead class="thead-dark">
              <tr>
                <th>Platform</th>
                <th>Ethernet</th>
                <th>WiFi</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td><strong>Windows</strong></td>
                <td>Ethernet, Ethernet 2</td>
                <td>Wi-Fi, Wi-Fi 2</td>
              </tr>
              <tr>
                <td><strong>Linux</strong></td>
                <td>eth0, enp3s0</td>
                <td>wlan0, wlp2s0</td>
              </tr>
              <tr>
                <td><strong>macOS</strong></td>
                <td>en0, en1</td>
                <td>en0 (often same as ethernet)</td>
              </tr>
            </tbody>
          </table>
          
          <h5 class="mt-4"><i class="fas fa-cog mr-2 text-success"></i>Interface Roles</h5>
          <div class="row">
            <div class="col-md-6">
              <div class="card bg-light">
                <div class="card-header"><strong>WAN / Uplink</strong></div>
                <div class="card-body">
                  <p>Connects to internet/upstream network</p>
                  <ul>
                    <li>Gets IP from ISP/upstream DHCP</li>
                    <li>Or configured with static IP</li>
                    <li>Gateway to internet</li>
                  </ul>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card bg-light">
                <div class="card-header"><strong>LAN / Downlink</strong></div>
                <div class="card-body">
                  <p>Connects to your local devices</p>
                  <ul>
                    <li>CrabFlow provides DHCP here</li>
                    <li>Devices connect to this network</li>
                    <li>Portal redirects happen here</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'routing'}
      <div class="card">
        <div class="card-header bg-warning">
          <h3 class="card-title text-dark"><i class="fas fa-route mr-2"></i>Routing & NAT</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-exchange-alt mr-2"></i>What is NAT?</h5>
            <p>NAT (Network Address Translation) allows multiple devices to share a single public IP address. When a device on your LAN accesses the internet, NAT translates its private IP to your public IP.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-toggle-on mr-2 text-warning"></i>Enabling Routing</h5>
          <ol>
            <li>Go to <strong>ACL & Permissions</strong> → <strong>Forwarding / NAT</strong></li>
            <li>Enable <strong>IP Forwarding</strong></li>
            <li>Enable <strong>NAT</strong> (for internet access)</li>
            <li>Select <strong>Uplink Interface</strong> (internet-facing)</li>
            <li>Select <strong>Downlink Interface</strong> (LAN-facing)</li>
            <li>Save configuration</li>
          </ol>
          
          <h5 class="mt-4"><i class="fas fa-sitemap mr-2 text-warning"></i>Port Forwarding</h5>
          <p>Forward external ports to internal servers:</p>
          
          <table class="table table-sm table-bordered">
            <thead>
              <tr>
                <th>External Port</th>
                <th>Internal IP</th>
                <th>Internal Port</th>
                <th>Use</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>80</td>
                <td>192.168.1.10</td>
                <td>80</td>
                <td>Web server</td>
              </tr>
              <tr>
                <td>443</td>
                <td>192.168.1.10</td>
                <td>443</td>
                <td>HTTPS</td>
              </tr>
              <tr>
                <td>2222</td>
                <td>192.168.1.5</td>
                <td>22</td>
                <td>SSH (non-standard port)</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      
    {:else if activeSection === 'wifi'}
      <div class="card">
        <div class="card-header bg-primary">
          <h3 class="card-title"><i class="fas fa-wifi mr-2"></i>WiFi Hotspot</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-broadcast-tower mr-2"></i>Creating a Hotspot</h5>
            <p>CrabFlow can create a WiFi access point for devices to connect to. This is perfect for guest networks with captive portal.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-clipboard-list mr-2 text-primary"></i>Requirements</h5>
          <ul>
            <li>WiFi adapter that supports AP mode</li>
            <li>Wired connection for internet (recommended)</li>
            <li>Administrator privileges</li>
          </ul>
          
          <h5 class="mt-4"><i class="fas fa-play-circle mr-2 text-primary"></i>Setup Steps</h5>
          <ol>
            <li>Go to <strong>Settings</strong> → <strong>WiFi</strong></li>
            <li>Click <strong>Create Hotspot</strong></li>
            <li>Configure:
              <ul>
                <li><strong>SSID:</strong> Network name (e.g., "CrabFlow-Guest")</li>
                <li><strong>Password:</strong> WiFi password (8+ characters)</li>
                <li><strong>Security:</strong> WPA2 recommended</li>
                <li><strong>Channel:</strong> Auto or specific (1, 6, 11 for 2.4GHz)</li>
              </ul>
            </li>
            <li>Start the hotspot</li>
          </ol>
          
          <div class="callout callout-success">
            <h5><i class="fas fa-check-circle mr-2"></i>With Captive Portal</h5>
            <p class="mb-0">Combine WiFi hotspot with captive portal for a complete guest network solution. Enable captive portal in <a href="/admin/acl">ACL settings</a> after creating your hotspot.</p>
          </div>
          
          <div class="callout callout-warning">
            <h5><i class="fas fa-exclamation-triangle mr-2"></i>Windows Limitation</h5>
            <p class="mb-0">On Windows, only one WiFi network can be active at a time. If your computer uses WiFi for internet, you may need an additional adapter or use ethernet for internet.</p>
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
