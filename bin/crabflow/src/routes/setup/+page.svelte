<script>
  export const ssr = false;

import { invoke } from '@tauri-apps/api/core'
import { goto } from '$app/navigation'

let step = 1 // ← this was missing

let hostname = ""
let adminEmail = ""
let adminUser = ""
let adminPass = ""
let telemetry = false

function next() {
  step += 1
}

async function finish() {
  try {
    await invoke("save_setup", {
      config: {
        hostname,
        admin_email: adminEmail,
        admin_user: adminUser,
        admin_pass: adminPass,
        telemetry,
        first_run: false
      }
    })
    goto("/admin/dashboard")
  } catch (e) {
    console.error("Setup failed:", e)
  }
}

</script>

{#if step === 1}
  <h2>Welcome to CrabFlow</h2>
  <p>This wizard will guide you through initial setup.</p>
  <button on:click={next}>Begin Setup</button>
{/if}

{#if step === 2}
  <h2>Network Basics</h2>
  <label>Hostname</label>
  <input bind:value={hostname} />
  <label>Admin Email</label>
  <input bind:value={adminEmail} />
  <button on:click={next}>Next</button>
{/if}

{#if step === 3}
  <h2>Create Admin Account</h2>
  <label>Username</label>
  <input bind:value={adminUser} />
  <label>Password</label>
  <input type="password" bind:value={adminPass} />
  <button on:click={next}>Next</button>
{/if}

{#if step === 4}
  <h2>Privacy & Consent</h2>
  <label>
    <input type="checkbox" bind:checked={telemetry} />
    Enable telemetry & audit logs
  </label>
  <button on:click={finish}>Finish Setup</button>
{/if}
