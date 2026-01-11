<script>
  let activeSection = 'users'
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0"><i class="fas fa-users mr-2"></i>Users & Groups Guide</h1>
  </div>
  <div class="col-sm-6">
    <ol class="breadcrumb float-sm-right">
      <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
      <li class="breadcrumb-item"><a href="/admin/about/guides">Guides</a></li>
      <li class="breadcrumb-item active">Users & Groups</li>
    </ol>
  </div>
</div>

<div class="row mt-3">
  <!-- Sidebar -->
  <div class="col-md-3">
    <div class="card card-outline card-teal">
      <div class="card-header">
        <h3 class="card-title">Sections</h3>
      </div>
      <div class="card-body p-0">
        <ul class="nav nav-pills flex-column">
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'users' ? 'active' : ''}"
                    on:click={() => activeSection = 'users'}>
              <i class="fas fa-user mr-2"></i> Managing Users
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'groups' ? 'active' : ''}"
                    on:click={() => activeSection = 'groups'}>
              <i class="fas fa-users mr-2"></i> Groups
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'permissions' ? 'active' : ''}"
                    on:click={() => activeSection = 'permissions'}>
              <i class="fas fa-key mr-2"></i> Permissions
            </button>
          </li>
          <li class="nav-item">
            <button class="nav-link text-left w-100 {activeSection === 'scenarios' ? 'active' : ''}"
                    on:click={() => activeSection = 'scenarios'}>
              <i class="fas fa-sitemap mr-2"></i> Scenarios
            </button>
          </li>
        </ul>
      </div>
    </div>
    
    <a href="/admin/users" class="btn btn-info btn-block mt-3">
      <i class="fas fa-users mr-2"></i> Manage Users
    </a>
    <a href="/admin/groups" class="btn btn-outline-info btn-block">
      <i class="fas fa-user-friends mr-2"></i> Manage Groups
    </a>
  </div>
  
  <!-- Content -->
  <div class="col-md-9">
    {#if activeSection === 'users'}
      <div class="card">
        <div class="card-header bg-info">
          <h3 class="card-title"><i class="fas fa-user mr-2"></i>Managing Users</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-info-circle mr-2"></i>User Accounts</h5>
            <p>Users can authenticate to the captive portal, access the admin dashboard (if permitted), and have bandwidth/access rules applied based on their group membership.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-user-plus mr-2 text-info"></i>Adding Users</h5>
          <ol>
            <li>Go to <strong>Users</strong> page</li>
            <li>Click <strong>Add User</strong></li>
            <li>Enter username, email, and password</li>
            <li>Assign to groups</li>
            <li>Set account status (active/disabled)</li>
            <li>Save</li>
          </ol>
          
          <h5 class="mt-4"><i class="fas fa-user-edit mr-2 text-info"></i>User Properties</h5>
          <table class="table table-bordered">
            <thead class="thead-dark">
              <tr>
                <th>Property</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td><strong>Username</strong></td>
                <td>Unique identifier for login</td>
              </tr>
              <tr>
                <td><strong>Email</strong></td>
                <td>Contact and recovery email</td>
              </tr>
              <tr>
                <td><strong>Groups</strong></td>
                <td>Determines permissions and bandwidth limits</td>
              </tr>
              <tr>
                <td><strong>Status</strong></td>
                <td>Active, Disabled, or Pending</td>
              </tr>
              <tr>
                <td><strong>ID Upload</strong></td>
                <td>Optional identity verification document</td>
              </tr>
            </tbody>
          </table>
          
          <div class="callout callout-warning">
            <h5><i class="fas fa-exclamation-triangle mr-2"></i>Default Admin Account</h5>
            <p class="mb-0">The default admin account is created during setup. Make sure to change the default password immediately!</p>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'groups'}
      <div class="card">
        <div class="card-header bg-success">
          <h3 class="card-title"><i class="fas fa-users mr-2"></i>Groups</h3>
        </div>
        <div class="card-body">
          <div class="callout callout-info">
            <h5><i class="fas fa-layer-group mr-2"></i>What are Groups?</h5>
            <p>Groups let you organize users and apply common settings. Users inherit permissions and bandwidth limits from their groups.</p>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-crown mr-2 text-warning"></i>Default Groups</h5>
          <div class="row">
            <div class="col-md-6">
              <div class="card card-outline card-danger">
                <div class="card-header">
                  <i class="fas fa-user-shield mr-2"></i> Administrators
                </div>
                <div class="card-body">
                  <ul class="mb-0 small">
                    <li>Full system access</li>
                    <li>Can manage all settings</li>
                    <li>No bandwidth limits</li>
                  </ul>
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="card card-outline card-primary">
                <div class="card-header">
                  <i class="fas fa-user mr-2"></i> Users
                </div>
                <div class="card-body">
                  <ul class="mb-0 small">
                    <li>Basic portal access</li>
                    <li>Can view own profile</li>
                    <li>Default bandwidth limits</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <h5 class="mt-4"><i class="fas fa-plus-circle mr-2 text-success"></i>Creating Custom Groups</h5>
          <ol>
            <li>Go to <strong>Groups</strong> page</li>
            <li>Click <strong>Add Group</strong></li>
            <li>Enter group name and description</li>
            <li>Select permissions to grant</li>
            <li>Configure bandwidth limits (in <a href="/admin/acl">ACL settings</a>)</li>
            <li>Save</li>
          </ol>
          
          <h5 class="mt-4"><i class="fas fa-sitemap mr-2 text-success"></i>Example Group Structure</h5>
          <div class="card bg-light">
            <div class="card-body">
              <pre class="mb-0">
├── Administrators (full access)
├── Staff
│   ├── IT Department (network management)
│   ├── Management (reports access)
│   └── Employees (basic access)
├── Guests
│   ├── VIP Guests (higher bandwidth)
│   └── Regular Guests (limited bandwidth)
└── Restricted (minimal access)</pre>
            </div>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'permissions'}
      <div class="card">
        <div class="card-header bg-warning">
          <h3 class="card-title text-dark"><i class="fas fa-key mr-2"></i>Permissions</h3>
        </div>
        <div class="card-body">
          <p>Permissions control what users can see and do in CrabFlow.</p>
          
          <h5><i class="fas fa-shield-alt mr-2 text-warning"></i>Permission Categories</h5>
          
          <table class="table table-bordered">
            <thead class="thead-dark">
              <tr>
                <th>Category</th>
                <th>Permissions</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td rowspan="3"><strong>Dashboard</strong></td>
                <td><code>view_dashboard</code></td>
                <td>Access main dashboard</td>
              </tr>
              <tr>
                <td><code>view_stats</code></td>
                <td>View network statistics</td>
              </tr>
              <tr>
                <td><code>view_logs</code></td>
                <td>View system logs</td>
              </tr>
              <tr>
                <td rowspan="2"><strong>Users</strong></td>
                <td><code>manage_users</code></td>
                <td>Create/edit/delete users</td>
              </tr>
              <tr>
                <td><code>manage_groups</code></td>
                <td>Create/edit/delete groups</td>
              </tr>
              <tr>
                <td rowspan="4"><strong>Network</strong></td>
                <td><code>manage_dhcp</code></td>
                <td>Configure DHCP settings</td>
              </tr>
              <tr>
                <td><code>manage_dns</code></td>
                <td>Configure DNS settings</td>
              </tr>
              <tr>
                <td><code>manage_firewall</code></td>
                <td>Configure firewall rules</td>
              </tr>
              <tr>
                <td><code>manage_acl</code></td>
                <td>Configure ACL & portal</td>
              </tr>
              <tr>
                <td rowspan="2"><strong>System</strong></td>
                <td><code>manage_settings</code></td>
                <td>System configuration</td>
              </tr>
              <tr>
                <td><code>manage_power</code></td>
                <td>Restart/shutdown system</td>
              </tr>
            </tbody>
          </table>
          
          <div class="callout callout-danger">
            <h5><i class="fas fa-exclamation-circle mr-2"></i>Security Note</h5>
            <p class="mb-0">Grant permissions carefully. Users with <code>manage_users</code> can escalate privileges by adding themselves to admin groups.</p>
          </div>
        </div>
      </div>
      
    {:else if activeSection === 'scenarios'}
      <div class="card">
        <div class="card-header bg-purple">
          <h3 class="card-title"><i class="fas fa-sitemap mr-2"></i>Organization Scenarios</h3>
        </div>
        <div class="card-body">
          <!-- Scenario 1 -->
          <div class="card card-outline card-primary mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-home mr-2"></i>Home Network</h5>
            </div>
            <div class="card-body">
              <p><strong>Simple setup for home use:</strong></p>
              <ul>
                <li><strong>Admin</strong> (1-2 people) - Full access</li>
                <li><strong>Family</strong> - Normal bandwidth, no admin access</li>
                <li><strong>Kids</strong> - Content filtering, time limits</li>
                <li><strong>Guests</strong> - Limited bandwidth, isolated network</li>
              </ul>
            </div>
          </div>
          
          <!-- Scenario 2 -->
          <div class="card card-outline card-info mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-building mr-2"></i>Small Business</h5>
            </div>
            <div class="card-body">
              <p><strong>Organized access for business:</strong></p>
              <ul>
                <li><strong>IT Admin</strong> - Full system access</li>
                <li><strong>Management</strong> - View reports, no config changes</li>
                <li><strong>Employees</strong> - Normal access, content policy</li>
                <li><strong>Contractors</strong> - Time-limited accounts</li>
                <li><strong>Visitors</strong> - Guest WiFi only</li>
              </ul>
            </div>
          </div>
          
          <!-- Scenario 3 -->
          <div class="card card-outline card-success mb-3">
            <div class="card-header">
              <h5 class="mb-0"><i class="fas fa-university mr-2"></i>Educational Institution</h5>
            </div>
            <div class="card-body">
              <p><strong>Complex hierarchy for schools:</strong></p>
              <ul>
                <li><strong>IT Staff</strong> - Full access</li>
                <li><strong>Faculty</strong> - Higher bandwidth, fewer restrictions</li>
                <li><strong>Staff</strong> - Standard access</li>
                <li><strong>Students</strong> - Content filtered, bandwidth limited</li>
                <li><strong>Library</strong> - Shared accounts for public terminals</li>
                <li><strong>Events</strong> - Temporary accounts for visitors</li>
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
