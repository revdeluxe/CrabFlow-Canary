<script>
  import { onMount } from 'svelte'
  import { listen } from '@tauri-apps/api/event'

  let updates = []

  onMount(async () => {
    await listen("crabflow://render", (event) => {
      console.log("Render event:", event.payload)
      updates.push(event.payload)
    })
  })
</script>

<ul>
  {#each updates as u}
    <li>{u.kind}: {JSON.stringify(u.data)}</li>
  {/each}
</ul>
