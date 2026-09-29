<script>
	import { onDestroy, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	let cpuUsage = $state(0);
	let memoryUsage = $state(0);

	let intervalId;

	async function refreshStats() {
		try {
			const stats = await invoke('get_system_stats');
			cpuUsage = stats.cpu_usage;
			memoryUsage = stats.memory_usage;
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

<footer class="status-footer" aria-label="System status">
	<span class="dim">Project Manager</span>
	<span class="metrics">
		<span>CPU {cpuUsage.toFixed(0)}%</span>
		<span>MEM {memoryUsage.toFixed(0)}%</span>
	</span>
</footer>

<style>
	.status-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-shrink: 0;
		height: 28px;
		padding: 0 0.75rem;
		border-top: 1px solid var(--terminal-border);
		background: var(--terminal-bg);
		font-size: 12px;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}

	.metrics {
		display: flex;
		gap: 1rem;
	}
</style>
