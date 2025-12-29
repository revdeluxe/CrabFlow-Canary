<script>
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { goto } from '$app/navigation'
  import { session } from '$lib/stores/session'

  let username = ""
  let password = ""
  let error = null

  onMount(async () => {
    const status = await invoke("check_first_run")
    if (status.first_run) {
      goto("/setup")
    }
  })

  async function doLogin() {
    try {
      const result = await invoke("login", { req: { username, password } })
      session.set(result)
      error = null

      if (result.role === "admin") {
        goto("/admin/dashboard")
      } else {
        goto("/portal/dashboard")
      }
    } catch (e) {
      error = e
    }
  }
</script>
