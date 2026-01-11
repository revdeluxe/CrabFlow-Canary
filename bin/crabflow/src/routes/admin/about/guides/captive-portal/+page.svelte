<script>
  let activeSection = 'overview'
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0"><i class="fas fa-sign-in-alt mr-2"></i>Captive Portal Guide</h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
      <li class="breadcrumb-item"><a href="/admin/about/guides">Guides</a></li>
      <li class="breadcrumb-item active">Captive Portal</li>
    </ol>
  </div>
</div>

<div class="row mt-3">
  <!-- Sidebar -->
  <div class="col-md-3">
    <div class="card card-outline card-orange">
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
            <button class="nav-link text-left w-100 {activeSection === 'setup' ? 'active' : ''}"
                    on:click={() => activeSection = 'setup'}>
              <i class="fas fa-cogs mr-2"></i> Setup
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'customize' ? 'active' : ''}"
                    on:click={() => activeSection = 'customize'}>
              <i class="fas fa-paint-brush mr-2"></i> Customization
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'scenarios' ? 'active' : ''}"
                    on:click={() => activeSection = 'scenarios'}>
              <i class="fas fa-lightbulb mr-2"></i> Use Cases
            </button>
          </li>
        </ul>
      </div>
    </div>
    
    <a href="/admin/acl" class="btn btn-warning btn-block mt-3">
      <i class="fas fa-cog mr-2"></i> Portal Settings
    </a>
    <a href="/admin/portal-editor" class="btn btn-outline-warning btn-block">
      <i class="fas fa-edit mr-2"></i> Portal Editor
    </a>
  </div>
  
  <!-- Content -->
  <div class="col-md-9">
    {#if activeSection === 'overview'}
      <div class="card">
        <div class="card-header bg-warning">
          <h3 class="card-title text-dark"><i class="fas fa-sign-in-alt mr-2"></i>Captive Portal Overview</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-question-circle mr-2"></i>What is a Captive Portal?</h5>
            <p>A captive portal intercepts network access and redirects users to a login page. Users must authenticate or accept terms before accessing the internet. This is commonly seen in hotels, airports, cafes, and businesses.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-flow-chart mr-2 text-warning"></i>How It Works</h5>
          <div class="row text-center">
            <div class="col-md-2">
              <div class="card bg-light">
                <div class="card-body p-2">
                  <i class="fas fa-wifi fa-2x text-primary mb-2"></i>
                  <small class="d-block">User connects to WiFi</small>
                </div>
              </div>
            </div>
            <div class="col-md-1 d-flex align-items-center justify-content-center">
              <i class="fas fa-arrow-right text-muted"></i>
            </div>
            <div class="col-md-2">
              <div class="card bg-light">
                <div class="card-body p-2">
                  <i class="fas fa-globe fa-2x text-info mb-2"></i>
                  <small class="d-block">Device checks connectivity</small>
                </div>
              </div>
            </div>
            <div class="col-md-1 d-flex align-items-center justify-content-center">
              <i class="fas fa-arrow-right text-muted"></i>
            </div>
            <div class="col-md-2">
              <div class="card bg-light">
                <div class="card-body p-2">
                  <i class="fas fa-directions fa-2x text-warning mb-2"></i>
                  <small class="d-block">CrabFlow redirects</small>
                </div>
              </div>
            </div>
            <div class="col-md-1 d-flex align-items-center justify-content-center">
              <i class="fas fa-arrow-right text-muted"></i>
            </div>
            <div class="col-md-2">
              <div class="card bg-light">
                <div class="card-body p-2">
                  <i class="fas fa-user-check fa-2x text-success mb-2"></i>
                  <small class="d-block">User logs in</small>
                </div>
              </div>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-star mr-2 text-warning"></i>Features</h5>
          <div class="row">
            <div class="col-md-6">
              <ul class="list-group">
                <li class="list-group-item"><i class="fas fa-check text-success mr-2"></i>Username/password authentication</li>
                <li class="list-group-item"><i class="fas fa-check text-success mr-2"></i>Terms & conditions acceptance</li>
                <li class="list-group-item"><i class="fas fa-check text-success mr-2"></i>Session timeout control</li>
              </ul>
            </div>
            <div class="col-md-6">
              <ul class="list-group">
                <li class="list-group-item"><i class="fas fa-check text-success mr-2"></i>Custom branding</li>
                <li class="list-group-item"><i class="fas fa-check text-success mr-2"></i>User session tracking</li>
                <li class="list-group-item"><i class="fas fa-check text-success mr-2"></i>Walled garden (allowed domains)</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'setup'}
      <div class="card">
        <div class="card-header bg-primary">
          <h3 class="card-title"><i class="fas fa-cogs mr-2"></i>Portal Setup</h3>
        </div>
        <div class="card-body">
          <h5><i class="fas fa-clipboard-check mr-2 text-primary"></i>Prerequisites</h5>
          <ul>
            <li><strong>DHCP enabled</strong> - Devices need IP addresses from CrabFlow</li>
            <li><strong>DNS enabled</strong> - CrabFlow must handle DNS for redirection</li>
            <li><strong>Network routing</strong> - CrabFlow must be the gateway</li>
          </ul>
          
          <hr>
          
          <h5><i class="fas fa-list-ol mr-2 text-primary"></i>Step-by-Step Setup</h5>
          
          <div class="card bg-light mb-3">
            <div class="card-body">
              <h6><span class="badge badge-primary mr-2">1</span>Enable Captive Portal</h6>
              <ol>
                <li>Go to <strong>ACL & Permissions</strong></li>
                <li>Select <strong>Captive Portal</strong> tab</li>
                <li>Toggle <strong>Enable Captive Portal</strong> ON</li>
              </ol>
            </div>
          </div>
          
          <div class="card bg-light mb-3">
            <div class="card-body">
              <h6><span class="badge badge-primary mr-2">2</span>Configure Authentication</h6>
              <ul>
                <li><strong>Require Authentication:</strong> Enable if users must log in</li>
                <li><strong>Session Timeout:</strong> How long until re-authentication (minutes)</li>
                <li><strong>Portal URL:</strong> Where users are redirected (default works for most)</li>
              </ul>
            </div>
          </div>
          
          <div class="card bg-light mb-3">
            <div class="card-body">
              <h6><span class="badge badge-primary mr-2">3</span>Set Allowed Domains (Walled Garden)</h6>
              <p>Domains accessible BEFORE authentication:</p>
              <ul>
                <li>Your portal domain (automatic)</li>
                <li>Payment processors if using paid WiFi</li>
                <li>Any external authentication services</li>
              </ul>
            </div>
          </div>
          
          <div class="card bg-light mb-3">
            <div class="card-body">
              <h6><span class="badge badge-primary mr-2">4</span>Create User Accounts</h6>
              <p>Go to <a href="/admin/users">Users</a> to create accounts for portal authentication, or allow self-registration.</p>
            </div>
          </div>
          
          <div class="callout callout-success">
            <h5><i class="fas fa-check-circle mr-2"></i>Test Your Portal</h5>
            <p class="mb-0">Connect a device to your network. You should see "Sign in to network" notification. If not, check that DHCP/DNS point to CrabFlow.</p>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'customize'}
      <div class="card">
        <div class="card-header bg-info">
          <h3 class="card-title"><i class="fas fa-paint-brush mr-2"></i>Portal Customization</h3>
        </div>
        <div class="card-body">
          <p>Customize your portal appearance using the <strong>Portal Editor</strong>.</p>
          
          <div class="row">
            <div class="col-md-6">
              <div class="card card-outline card-info">
                <div class="card-header">
                  <h5 class="mb-0"><i class="fas fa-image mr-2"></i>Branding</h5>
                </div>
                <div class="card-body">
                  <ul>
                    <li>Upload your logo</li>
                    <li>Set brand colors</li>
                    <li>Custom background image</li>
                    <li>Company name & tagline</li>
                  </ul>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card card-outline card-success">
                <div class="card-header">
                  <h5 class="mb-0"><i class="fas fa-file-alt mr-2"></i>Content</h5>
                </div>
                <div class="card-body">
                  <ul>
                    <li>Welcome message</li>
                    <li>Terms of service</li>
                    <li>Privacy policy link</li>
                    <li>Support contact info</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-code mr-2 text-info"></i>Advanced Customization</h5>
          <p>For complete control, you can edit the portal HTML/CSS directly:</p>
          <ol>
            <li>Go to <strong>Portal Editor</strong></li>
            <li>Click <strong>Advanced Mode</strong></li>
            <li>Edit HTML template directly</li>
            <li>Preview changes before saving</li>
          </ol>
          
          <div class="callout callout-warning">
            <h5><i class="fas fa-exclamation-triangle mr-2"></i>Important Variables</h5>
            <p class="mb-0">Keep these placeholders in your template:<br>
            <code>{{REDIRECT_URL}}</code> - Where to send user after login<br>
            <code>{{ERROR_MESSAGE}}</code> - Display login errors<br>
            <code>{{USERNAME}}</code> - Pre-fill username if known</p>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'scenarios'}
      <div class="card">
        <div class="card-header bg-success">
          <h3 class="card-title"><i class="fas fa-lightbulb mr-2"></i>Common Use Cases</h3>
        </div>
        <div class="card-body">
          <!-- Scenario 1 -->
          <div class="card card-outline card-primary mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-hotel mr-2"></i>Hotel / Guest WiFi</h5>
            </div>
            <div class="card-body">
              <p><strong>Goal:</strong> Provide internet access to guests with room number verification</p>
              <ul>
                <li>Enable captive portal with authentication</li>
                <li>Create accounts per room number</li>
                <li>Set 24-hour session timeout</li>
                <li>Customize with hotel branding</li>
                <li>Add terms of service acceptance</li>
              </ul>
            </div>
          </div>
          
          <!-- Scenario 2 -->
          <div class="card card-outline card-warning mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-coffee mr-2"></i>Café / Restaurant</h5>
            </div>
            <div class="card-body">
              <p><strong>Goal:</strong> Free WiFi with terms acceptance</p>
              <ul>
                <li>Enable captive portal without authentication</li>
                <li>Require terms acceptance only</li>
                <li>Set 2-hour session timeout</li>
                <li>Apply bandwidth limits (e.g., 5 Mbps)</li>
                <li>Block inappropriate content</li>
              </ul>
            </div>
          </div>
          
          <!-- Scenario 3 -->
          <div class="card card-outline card-info mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-building mr-2"></i>Office Guest Network</h5>
            </div>
            <div class="card-body">
              <p><strong>Goal:</strong> Provide temporary access to visitors</p>
              <ul>
                <li>Enable authentication</li>
                <li>Create temporary accounts for visitors</li>
                <li>Isolate from main network (see Firewall guide)</li>
                <li>8-hour session timeout</li>
                <li>Log user activity for compliance</li>
              </ul>
            </div>
          </div>
          
          <!-- Scenario 4 -->
          <div class="card card-outline card-danger mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-graduation-cap mr-2"></i>School / Library</h5>
            </div>
            <div class="card-body">
              <p><strong>Goal:</strong> Controlled access with content filtering</p>
              <ul>
                <li>Enable authentication with student accounts</li>
                <li>Integrate with existing user directory</li>
                <li>Enable ad-blocking and content filtering</li>
                <li>Set appropriate bandwidth limits</li>
                <li>Block social media during school hours</li>
              </ul>
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
