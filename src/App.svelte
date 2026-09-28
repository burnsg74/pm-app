<script>
  import SystemStatsWidget from './SystemStatsWidget.svelte';

  import { getCurrentWindow } from '@tauri-apps/api/window';

  // Run this function after your app is fully loaded
  // (e.g., inside useEffect in React, onMounted in Vue, or DOMContentLoaded)
  async function revealApp() {
    const appWindow = getCurrentWindow();

    // Ensures the layout is fully maximized right before showing
    await appWindow.maximize();

    // Instantly reveal the window
    await appWindow.show();
  }

  // Example using raw JS:
  window.addEventListener('DOMContentLoaded', () => {
    revealApp();
  });
</script>

<main>
  App.svelte Page
  <!-- <svelte:component this={routes[currentPage]} {navigate} /> -->

  <SystemStatsWidget />
</main>
