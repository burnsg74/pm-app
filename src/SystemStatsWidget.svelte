<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let cpuUsage = $state(0);
  let memoryUsage = $state(0);
  let memoryUsedMb = $state(0);
  let memoryTotalMb = $state(0);

  let intervalId;

  async function refreshStats() {
    try {
      const stats = await invoke('get_system_stats');
      cpuUsage = stats.cpu_usage;
      memoryUsage = stats.memory_usage;
      memoryUsedMb = stats.memory_used / 1024 / 1024;
      memoryTotalMb = stats.memory_total / 1024 / 1024;
    } catch (err) {
      console.error('Failed to fetch system stats:', err);
    }
  }

  onMount(() => {
    refreshStats();
    intervalId = setInterval(refreshStats, 1000);
  });

  onDestroy(() => {
    clearInterval(intervalId);
  });
</script>

<fieldset class="widget">
  <legend class="widget-title">System Monitor</legend>
  <div class="widget-row">
    <span class="widget-label">CPU:</span>
    <span class="glow widget-value">{cpuUsage.toFixed(1)}%</span>
  </div>
  <div class="widget-row">
    <span class="widget-label">Memory:</span>
    <span class="glow widget-value"
      >{memoryUsage.toFixed(1)}% </span
    >
  </div>
</fieldset>

<style>
  .widget {
    border: 2px solid var(--terminal-border);
    padding: 0.75rem 1rem 1rem;
    max-width: 320px;
    margin: 1rem 0;
    font-family: var(--terminal-font);
    color: var(--terminal-fg);
  }

  .widget-title {
    padding: 0 0.5em;
    text-transform: uppercase;
    font-weight: bold;
  }

  .widget-row {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.25rem 0;
  }

  .widget-label {
    color: var(--terminal-fg);
  }

  .widget-value {
    font-weight: bold;
  }
</style>
