<script>
  import { invoke } from '@tauri-apps/api/core'
  let leases = []

  async function refresh() {
    leases = await invoke("list_leases")
  }

  refresh()
</script>

<h2>DHCP Leases</h2>
<table>
  <thead>
    <tr><th>IP</th><th>MAC</th><th>Hostname</th><th>Expires</th></tr>
  </thead>
  <tbody>
    {#each leases as l}
      <tr>
        <td>{l.ip}</td>
        <td>{l.mac}</td>
        <td>{l.hostname}</td>
        <td>{l.expires_at}</td>
      </tr>
    {/each}
  </tbody>
</table>
