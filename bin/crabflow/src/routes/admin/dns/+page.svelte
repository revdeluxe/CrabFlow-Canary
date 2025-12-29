<script>
  import { invoke } from '@tauri-apps/api/core'
  let records = []

  async function refresh() {
    records = await invoke("list_records")
  }

  refresh()
</script>

<h2>DNS Records</h2>
<table>
  <thead>
    <tr><th>Name</th><th>Type</th><th>Value</th><th>TTL</th></tr>
  </thead>
  <tbody>
    {#each records as r}
      <tr>
        <td>{r.name}</td>
        <td>{r.rtype}</td>
        <td>{r.value}</td>
        <td>{r.ttl}</td>
      </tr>
    {/each}
  </tbody>
</table>
