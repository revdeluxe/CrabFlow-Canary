<script>
  import { onMount } from 'svelte'

  // Check if running in Tauri
  const isTauri = typeof window !== 'undefined' && window.__TAURI_INTERNALS__;

  let updates = []

  onMount(async () => {
    if (!isTauri) return;
    
    try {
      const { listen } = await import('@tauri-apps/api/event');
      await listen("crabflow://render", (event) => {
        console.log("Render event:", event.payload)
        updates.push(event.payload)
      })
    } catch (e) {
      console.warn("Failed to setup Tauri listener:", e);
    }
  })
</script>

<ul>
  {#each updates as u}
    <li>{u.kind}: {JSON.stringify(u.data)}</li>
  {/each}
</ul>
