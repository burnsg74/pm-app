<script>
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

<main>
  <SystemStatsWidget />
</main>
