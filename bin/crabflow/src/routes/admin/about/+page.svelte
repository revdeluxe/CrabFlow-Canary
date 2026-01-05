<script>
</script>

<div class="row mb-2">
  <div class="col-sm-6">
    <h1 class="m-0">About</h1>
  </div>
</div>

<div class="row">
  <div class="col-md-6">
    <div class="card card-primary card-outline">
      <div class="card-body box-profile">
        <div class="text-center">
          <img class="profile-user-img img-fluid img-circle"
               src="/favicon.png"
               alt="User profile picture"
               on:error={(e) => e.currentTarget.style.display = 'none'}>
        </div>

        <h3 class="profile-username text-center">CrabFlow Canary</h3>

        <p class="text-muted text-center">Network Management System</p>

        <ul class="list-group list-group-unbordered mb-3">
          <li class="list-group-item">
            <b>Version</b> <a class="float-right">0.1.0 (Canary)</a>
          </li>
          <li class="list-group-item">
            <b>Lead Programmer</b> <a class="float-right">Richard Villareal</a>
          </li>
        </ul>
      </div>
      <!-- /.card-body -->
    </div>
    <!-- /.card -->
  </div>
  
  <div class="col-md-6">
    <div class="card card-info">
      <div class="card-header">
        <h3 class="card-title">Credits</h3>
      </div>
      <div class="card-body">
        <p>Special thanks to the open source community.</p>
        <ul>
            <li>Tauri</li>
            <li>SvelteKit</li>
            <li>Rust</li>
            <li>AdminLTE</li>
        </ul>
      </div>
    </div>
  </div>
</div>
