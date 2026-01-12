<script>
  import { onMount } from 'svelte'
  import { api } from '$lib/tauri'
  import { session } from '$lib/stores/session'
  import { get } from 'svelte/store'
  // convertFileSrc loaded dynamically in Tauri context

  let user = {
    username: 'Admin',
    email: 'admin@example.com',
    nickname: 'Administrator',
    role: 'Administrator',
    id_document_path: null
  }
  let newPassword = ''
  let history = []
  let loading = true
  let fileInput;
  let profileImageUrl = null;

  onMount(async () => {
    try {
      const s = get(session);
      let config = null;

      if (s && s.user) {
        user.username = s.user.username;
      } else {
        config = await api.loadSetup()
        if (config) {
            // Fallback to config if user DB doesn't have it yet, but ideally we fetch from DB
            user.username = config.admin_user || 'Admin'
        }
      }
      
      // Fetch actual user data from DB
      const users = await api.listUsers()
      const dbUser = users.find(u => u.username === user.username)
      if (dbUser) {
        user = { ...user, ...dbUser }
        // Ensure defaults if missing in DB
        if (!user.email && config) user.email = config.admin_email
        if (!user.nickname) user.nickname = 'Administrator'
        // Load profile image if available
        if (user.id_document_path) {
          try {
            if (api.isTauri()) {
              const { convertFileSrc } = await import('@tauri-apps/api/core')
              profileImageUrl = convertFileSrc(user.id_document_path)
            } else {
              profileImageUrl = user.id_document_path
            }
          } catch (e) {
            console.warn('Could not load profile image:', e)
          }
        }
      }

      try {
          history = await api.getUserHistory(user.username)
      } catch (err) {
          console.warn("Could not fetch history:", err)
      }
    } catch (e) {
      console.error(e)
    } finally {
      loading = false
    }
  })

  async function saveProfile() {
    try {
      await api.updateUserProfile(user.username, user.nickname, user.email)
      
      if (newPassword) {
        await api.changePassword(user.username, newPassword)
        newPassword = '' // Clear after save
      }
      
      alert("Profile updated successfully!")
    } catch (e) {
      alert("Failed to update profile: " + e)
    }
  }

  async function handleIdUpload(event) {
    const file = event.target.files[0];
    if (!file) return;

    if (history.length === 0) {
        // alert("You must be logged in via the portal (tagged) to upload an ID.");
        // return;
    }

    const reader = new FileReader();
    reader.onload = async (e) => {
        const base64 = e.target.result;
        try {
            await api.uploadId(user.username, base64);
            // Refresh user data to get new image path
            const users = await api.listUsers();
            const dbUser = users.find(u => u.username === user.username);
            if (dbUser && dbUser.id_document_path) {
              profileImageUrl = convertFileSrc(dbUser.id_document_path);
            }
            alert("Profile picture uploaded successfully!");
        } catch (err) {
            alert("Upload failed: " + err);
        }
    };
    reader.readAsDataURL(file);
  }

  function triggerFileUpload() {
    fileInput.click();
  }
</script>

<style>
  .profile-img-container {
    position: relative;
    width: 100px;
    height: 100px;
    margin: 0 auto;
    cursor: pointer;
  }
  
  .profile-img-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    opacity: 0;
    transition: opacity 0.3s;
    color: white;
    font-size: 1.5rem;
  }

  .profile-img-container:hover .profile-img-overlay {
    opacity: 1;
  }
</style>

<section class="content-header">
  <div class="container-fluid">
    <div class="row mb-2">
      <div class="col-sm-6">
        <h1>Profile</h1>
      </div>
      <div class="col-sm-6">
        <ol class="breadcrumb float-sm-right">
          <li class="breadcrumb-item"><a href="/admin/dashboard">Home</a></li>
          <li class="breadcrumb-item active">User Profile</li>
        </ol>
      </div>
    </div>
  </div>
</section>

<section class="content">
  <div class="container-fluid">
    <div class="row">
      
      <!-- Left Column: Profile Info (View Container) -->
      <div class="col-12">
        <div class="card card-primary card-outline">
          <div class="card-body box-profile">
            <div class="text-center">
              <div class="profile-img-container" on:click={triggerFileUpload}>
                {#if profileImageUrl}
                  <img src={profileImageUrl} alt="Profile" class="img-circle elevation-2" style="width: 100%; height: 100%; object-fit: cover;">
                {:else}
                  <div class="img-circle elevation-2 d-flex justify-content-center align-items-center bg-light" style="width: 100%; height: 100%; font-size: 3rem;">
                    <i class="fas fa-user"></i>
                  </div>
                {/if}
                <div class="profile-img-overlay">
                  <i class="fas fa-camera"></i>
                </div>
              </div>
            </div>
            <h3 class="profile-username text-center mt-3">{user.username}</h3>
            <p class="text-muted text-center">{user.role}</p>
            
            <ul class="list-group list-group-unbordered mb-3">
              <li class="list-group-item">
                <b>Email</b> <a class="float-right">{user.email}</a>
              </li>
            </ul>

            <!-- Hidden File Input -->
            <input type="file" bind:this={fileInput} style="display: none;" on:change={handleIdUpload}>
          </div>
        </div>
      </div>

      <!-- Right Column: Settings Form (Edit Container) -->
      <div class="col-12">
        <div class="card">
          <div class="card-header p-2">
            <ul class="nav nav-pills">
              <li class="nav-item"><a class="nav-link active" href="#settings" data-toggle="tab">Settings</a></li>
            </ul>
          </div>
          <div class="card-body">
            <div class="tab-content">
              <div class="active tab-pane" id="settings">
                <form on:submit|preventDefault={saveProfile}>
                  <div class="form-group">
                    <label for="inputName">Username</label>
                    <input type="text" class="form-control" id="inputName" placeholder="Name" bind:value={user.username} readonly>
                  </div>
                  <div class="form-group">
                    <label for="inputNickname">Nickname</label>
                    <input type="text" class="form-control" id="inputNickname" placeholder="Nickname" bind:value={user.nickname}>
                  </div>
                  <div class="form-group">
                    <label for="inputEmail">Email</label>
                    <input type="email" class="form-control" id="inputEmail" placeholder="Email" bind:value={user.email}>
                  </div>

                  <div class="form-group">
                    <label for="inputPassword">New Password</label>
                    <input type="password" class="form-control" id="inputPassword" placeholder="Leave blank to keep current" bind:value={newPassword}>
                  </div>
                  
                  <!-- Explicit Upload Button in Edit Container -->
                  <div class="form-group">
                    <label>ID Document</label>
                    <div>
                      <button type="button" class="btn btn-default" on:click={triggerFileUpload}>
                        <i class="fas fa-upload"></i> Upload ID
                      </button>
                      <span class="text-muted ml-2 small">Or click profile picture</span>
                    </div>
                  </div>

                  <div class="form-group">
                    <button type="submit" class="btn btn-danger">Save Changes</button>
                  </div>
                </form>
              </div>
            </div>
          </div>
        </div>
      </div>

    </div>

    <!-- Login History Card (Full Width Below) -->
    <div class="row">
      <div class="col-12">
        <div class="card">
          <div class="card-header">
            <h3 class="card-title">Login History</h3>
          </div>
          <div class="card-body table-responsive p-0">
            <table class="table table-hover text-nowrap">
              <thead>
                <tr>
                  <th>IP</th>
                  <th>MAC</th>
                  <th>Device</th>
                  <th>Time</th>
                </tr>
              </thead>
              <tbody>
                {#each history as record}
                  <tr>
                    <td>{record.ip}</td>
                    <td>{record.mac}</td>
                    <td>{record.device_name || 'Unknown'}</td>
                    <td>{new Date(record.timestamp).toLocaleString()}</td>
                  </tr>
                {/each}
                {#if history.length === 0}
                  <tr><td colspan="4" class="text-center">No login history found.</td></tr>
                {/if}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>

  </div>
</section>
