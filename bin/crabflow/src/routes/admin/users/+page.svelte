<script>
  import { invoke } from '@tauri-apps/api/core'
  let users = []
  let filter = "student"

  async function loadUsers() {
    users = await invoke("list_users")
  }

  $: filtered = users.filter(u => u.groups.includes(filter))
  loadUsers()
</script>

<select bind:value={filter}>
  <option value="student">Students</option>
  <option value="staff">Staff</option>
  <option value="admin">Admins</option>
  <option value="custom">Custom</option>
</select>

<ul>
  {#each filtered as u}
    <li>{u.username} ({u.role}) → {u.groups.join(", ")}</li>
  {/each}
</ul>
