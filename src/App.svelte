<script>
	import Board from './Board.svelte';
	import SystemStatsWidget from './SystemStatsWidget.svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	async function revealApp() {
		const appWindow = getCurrentWindow();
		await appWindow.maximize();
		await appWindow.show();
	}

	window.addEventListener('DOMContentLoaded', () => {
		revealApp();
	});
</script>

<div class="shell">
	<Board />
	<SystemStatsWidget />
</div>

<style>
	.shell {
		display: flex;
		flex-direction: column;
		height: 100%;
		min-height: 0;
	}
</style>
