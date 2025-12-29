<script>
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'

  let stats = {
    users: 0,
    sessions: 0,
    logs: 0
  }

  function formatNumber(n) {
    if (n >= 1_000_000) return (n / 1_000_000).toFixed(1) + "M"
    if (n >= 1_000) return (n / 1_000).toFixed(1) + "K"
    return n.toString()
  }

  async function refreshStats() {
    try {
      stats.users = await invoke("count_users")
      stats.sessions = await invoke("count_sessions")
      stats.logs = await invoke("count_logs")
    } catch (e) {
      console.error("Failed to refresh stats:", e)
    }
  }

  onMount(() => {
    refreshStats()
    const interval = setInterval(refreshStats, 30_000) // refresh every 30s
    return () => clearInterval(interval)
  })
</script>

<div class="tracker-card">
  <h3>Overview</h3>
  <ul>
    <li><strong>Users:</strong> {formatNumber(stats.users)}</li>
    <li><strong>Sessions:</strong> {formatNumber(stats.sessions)}</li>
    <li><strong>Logs:</strong> {formatNumber(stats.logs)}</li>
  </ul>
</div>

<style>
  .tracker-card {
    background: #f9f9f9;
    padding: 0.75rem;
    border-radius: 6px;
    margin: 0.5rem 0;
    font-size: 0.9rem;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  }
  .tracker-card h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1rem;
  }
  .tracker-card ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  .tracker-card li {
    margin: 0.25rem 0;
  }
</style>
