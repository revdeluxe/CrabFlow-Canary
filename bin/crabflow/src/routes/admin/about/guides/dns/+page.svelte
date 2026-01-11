<script>
  let activeSection = 'overview'
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0"><i class="fas fa-globe mr-2"></i>DNS Server Guide</h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
      <li class="breadcrumb-item"><a href="/admin/about/guides">Guides</a></li>
      <li class="breadcrumb-item active">DNS</li>
    </ol>
  </div>
</div>

<div class="row mt-3">
  <!-- Sidebar -->
  <div class="col-md-3">
    <div class="card card-outline card-warning">
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
            <button class="nav-link text-left w-100 {activeSection === 'records' ? 'active' : ''}"
                    on:click={() => activeSection = 'records'}>
              <i class="fas fa-list mr-2"></i> DNS Records
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'adblock' ? 'active' : ''}"
                    on:click={() => activeSection = 'adblock'}>
              <i class="fas fa-ban mr-2"></i> Ad Blocking
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'homelab' ? 'active' : ''}"
                    on:click={() => activeSection = 'homelab'}>
              <i class="fas fa-server mr-2"></i> Homelab DNS
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'troubleshoot' ? 'active' : ''}"
                    on:click={() => activeSection = 'troubleshoot'}>
              <i class="fas fa-wrench mr-2"></i> Troubleshooting
            </button>
          </li>
        </ul>
      </div>
    </div>
    
    <a href="/admin/dns" class="btn btn-warning btn-block mt-3">
      <i class="fas fa-cog mr-2"></i> Open DNS Settings
    </a>
  </div>
  
  <!-- Content -->
  <div class="col-md-9">
    {#if activeSection === 'overview'}
      <div class="card">
        <div class="card-header bg-warning">
          <h3 class="card-title text-dark"><i class="fas fa-globe mr-2"></i>DNS Overview</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-question-circle mr-2"></i>What is DNS?</h5>
            <p>DNS (Domain Name System) translates human-readable domain names (like google.com) into IP addresses (like 142.250.80.46) that computers use to communicate.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-star mr-2 text-warning"></i>CrabFlow DNS Features</h5>
          <div class="row">
            <div class="col-md-6">
              <ul class="list-group">
                <li class="list-group-item">
                  <i class="fas fa-check text-success mr-2"></i>
                  Local DNS resolution
                </li>
                <li class="list-group-item">
                  <i class="fas fa-check text-success mr-2"></i>
                  Custom DNS records (A, CNAME, MX, TXT)
                </li>
                <li class="list-group-item">
                  <i class="fas fa-check text-success mr-2"></i>
                  DNS-based ad blocking
                </li>
              </ul>
            </div>
            <div class="col-md-6">
              <ul class="list-group">
                <li class="list-group-item">
                  <i class="fas fa-check text-success mr-2"></i>
                  Query logging & analytics
                </li>
                <li class="list-group-item">
                  <i class="fas fa-check text-success mr-2"></i>
                  Upstream DNS configuration
                </li>
                <li class="list-group-item">
                  <i class="fas fa-check text-success mr-2"></i>
                  Homelab templates
                </li>
              </ul>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-cogs mr-2 text-warning"></i>Basic Setup</h5>
          <ol>
            <li>Go to <strong>DNS</strong> page</li>
            <li>Ensure DNS server is <strong>enabled</strong></li>
            <li>Configure <strong>upstream servers</strong> (default: Cloudflare & Google)</li>
            <li>Add your <strong>local domain</strong> (e.g., home.local)</li>
            <li>Save configuration</li>
          </ol>
        </div>
      </div>
      
    {:else if activeSection === 'records'}
      <div class="card">
        <div class="card-header bg-info">
          <h3 class="card-title"><i class="fas fa-list mr-2"></i>DNS Records</h3>
        </div>
        <div class="card-body">
          <h5><i class="fas fa-database mr-2 text-info"></i>Record Types</h5>
          
          <table class="table table-bordered">
            <thead class="thead-dark">
              <tr>
                <th>Type</th>
                <th>Purpose</th>
                <th>Example</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td><span class="badge badge-primary">A</span></td>
                <td>Maps hostname to IPv4 address</td>
                <td><code>server.home.local → 192.168.1.10</code></td>
              </tr>
              <tr>
                <td><span class="badge badge-success">AAAA</span></td>
                <td>Maps hostname to IPv6 address</td>
                <td><code>server.home.local → fe80::1</code></td>
              </tr>
              <tr>
                <td><span class="badge badge-warning">CNAME</span></td>
                <td>Alias to another hostname</td>
                <td><code>www.home.local → server.home.local</code></td>
              </tr>
              <tr>
                <td><span class="badge badge-info">MX</span></td>
                <td>Mail server for domain</td>
                <td><code>home.local → mail.home.local (priority 10)</code></td>
              </tr>
              <tr>
                <td><span class="badge badge-secondary">TXT</span></td>
                <td>Text record (SPF, verification)</td>
                <td><code>home.local → "v=spf1 ..."</code></td>
              </tr>
            </tbody>
          </table>
          
          <h5 class="mt-4"><i class="fas fa-plus-circle mr-2 text-info"></i>Adding Records</h5>
          <ol>
            <li>Go to <strong>DNS</strong> → <strong>Records</strong> tab</li>
            <li>Click <strong>Add Record</strong></li>
            <li>Select record type</li>
            <li>Enter hostname (without domain for local records)</li>
            <li>Enter value (IP address or target hostname)</li>
            <li>Click <strong>Save</strong></li>
          </ol>
          
          <div class="callout callout-warning">
            <h5><i class="fas fa-lightbulb mr-2"></i>Tip</h5>
            <p class="mb-0">Use <code>.local</code> domains for internal resources to avoid conflicts with public DNS.</p>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'adblock'}
      <div class="card">
        <div class="card-header bg-danger">
          <h3 class="card-title"><i class="fas fa-ban mr-2"></i>Ad Blocking</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-shield-alt mr-2"></i>DNS-based Ad Blocking</h5>
            <p>CrabFlow blocks ads at the DNS level. When a device tries to load an ad, the DNS request is blocked before any connection is made.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-cogs mr-2 text-danger"></i>Setup</h5>
          <ol>
            <li>Go to <strong>Ad-Block</strong> page</li>
            <li>Enable <strong>Ad Blocking</strong></li>
            <li>Choose blocklists to use</li>
            <li>Optionally add custom blocked domains</li>
            <li>Add whitelist entries for false positives</li>
          </ol>
          
          <h5 class="mt-4"><i class="fas fa-list-alt mr-2 text-danger"></i>Recommended Blocklists</h5>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>List</th>
                <th>Description</th>
                <th>Size</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td><strong>AdGuard DNS</strong></td>
                <td>General ads & trackers</td>
                <td>~50K domains</td>
              </tr>
              <tr>
                <td><strong>Steven Black</strong></td>
                <td>Unified hosts file</td>
                <td>~80K domains</td>
              </tr>
              <tr>
                <td><strong>OISD</strong></td>
                <td>Comprehensive blocking</td>
                <td>~150K domains</td>
              </tr>
            </tbody>
          </table>
          
          <div class="callout callout-warning">
            <h5><i class="fas fa-exclamation-triangle mr-2"></i>Troubleshooting</h5>
            <p class="mb-0">If a website breaks after enabling ad-blocking, check the query logs to find which domain was blocked and add it to your whitelist.</p>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'homelab'}
      <div class="card">
        <div class="card-header bg-success">
          <h3 class="card-title"><i class="fas fa-server mr-2"></i>Homelab DNS Templates</h3>
        </div>
        <div class="card-body">
          <p>CrabFlow includes pre-configured templates for popular homelab services:</p>
          
          <div class="row">
            <div class="col-md-6">
              <div class="card card-outline card-info mb-3">
                <div class="card-header">
                  <i class="fas fa-home mr-2"></i> Home Assistant
                </div>
                <div class="card-body">
                  <code>homeassistant.local → Your HA IP</code>
                  <br><small class="text-muted">Smart home automation platform</small>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card card-outline card-warning mb-3">
                <div class="card-header">
                  <i class="fas fa-plex mr-2"></i> Plex
                </div>
                <div class="card-body">
                  <code>plex.local → Your Plex IP</code>
                  <br><small class="text-muted">Media server</small>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card card-outline card-primary mb-3">
                <div class="card-header">
                  <i class="fas fa-shield-alt mr-2"></i> Pi-hole
                </div>
                <div class="card-body">
                  <code>pihole.local → Your Pi-hole IP</code>
                  <br><small class="text-muted">Network-wide ad blocking</small>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card card-outline card-success mb-3">
                <div class="card-header">
                  <i class="fas fa-box mr-2"></i> Proxmox
                </div>
                <div class="card-body">
                  <code>proxmox.local → Your Proxmox IP</code>
                  <br><small class="text-muted">Virtualization platform</small>
                </div>
              </div>
            </div>
          </div>
          
          <h5 class="mt-3"><i class="fas fa-magic mr-2 text-success"></i>Using Templates</h5>
          <ol>
            <li>Go to <strong>DNS</strong> page</li>
            <li>Scroll to <strong>Homelab Templates</strong></li>
            <li>Click on a service card</li>
            <li>Enter the IP address of your service</li>
            <li>Template records are automatically created</li>
          </ol>
        </div>
      </div>
      
    {:else if activeSection === 'troubleshoot'}
      <div class="card">
        <div class="card-header bg-secondary">
          <h3 class="card-title"><i class="fas fa-wrench mr-2"></i>Troubleshooting</h3>
        </div>
        <div class="card-body">
          <div class="accordion" id="troubleshootAccordion">
            <div class="card">
              <div class="card-header">
                <h5 class="mb-0">
                  <button class="btn btn-link" data-toggle="collapse" data-target="#issue1">
                    DNS queries not resolving
                  </button>
                </h5>
              </div>
              <div id="issue1" class="collapse show" data-parent="#troubleshootAccordion">
                <div class="card-body">
                  <ol>
                    <li>Check that DNS server is enabled</li>
                    <li>Verify devices are using CrabFlow as DNS server</li>
                    <li>Check upstream DNS servers are reachable</li>
                    <li>Review DNS logs for errors</li>
                  </ol>
                </div>
              </div>
            </div>
            
            <div class="card">
              <div class="card-header">
                <h5 class="mb-0">
                  <button class="btn btn-link collapsed" data-toggle="collapse" data-target="#issue2">
                    Local domains not working
                  </button>
                </h5>
              </div>
              <div id="issue2" class="collapse" data-parent="#troubleshootAccordion">
                <div class="card-body">
                  <ol>
                    <li>Ensure record is added correctly (check typos)</li>
                    <li>Flush DNS cache on client: <code>ipconfig /flushdns</code></li>
                    <li>Try using fully qualified domain name</li>
                    <li>Check record type matches your use case</li>
                  </ol>
                </div>
              </div>
            </div>
            
            <div class="card">
              <div class="card-header">
                <h5 class="mb-0">
                  <button class="btn btn-link collapsed" data-toggle="collapse" data-target="#issue3">
                    Ad blocking not working
                  </button>
                </h5>
              </div>
              <div id="issue3" class="collapse" data-parent="#troubleshootAccordion">
                <div class="card-body">
                  <ol>
                    <li>Verify ad-blocking is enabled</li>
                    <li>Check device is using CrabFlow DNS (not bypassing)</li>
                    <li>Some apps use hardcoded DNS - may need firewall rule</li>
                    <li>Clear browser cache after enabling</li>
                  </ol>
                </div>
              </div>
            </div>
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
