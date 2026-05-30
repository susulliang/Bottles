<script lang="ts">
  import '../app.css';
  import { session } from '$lib/store';
  import AuthPanel from '$lib/components/AuthPanel.svelte';
  import TabBar from '$lib/components/TabBar.svelte';
  import ThrowTab from '$lib/components/ThrowTab.svelte';
  import BottlesTab from '$lib/components/BottlesTab.svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let activeTab: 'throw' | 'bottles' = 'throw';

  function handleTabChange(tab: 'throw' | 'bottles') {
    activeTab = tab;
  }

  async function closeWindow() {
    const window = getCurrentWindow();
    await window.close();
  }
</script>

<svelte:head>
  <title>Bottles - 漂流瓶</title>
</svelte:head>

<!-- Close Button -->
<button class="close-btn" onclick={closeWindow} aria-label="Close">&times;</button>

{#if !$session}
  <AuthPanel />
{:else}
  <div class="app">
    <TabBar activeTab={activeTab} onchange={handleTabChange} />
    {#if activeTab === 'throw'}
      <ThrowTab />
    {:else}
      <BottlesTab />
    {/if}
  </div>
{/if}

<style>
  .close-btn {
    position: fixed;
    top: 8px;
    right: 8px;
    background: rgba(255, 255, 255, 0.08);
    border: none;
    border-radius: 4px;
    color: rgba(255, 255, 255, 0.4);
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-size: 14px;
    line-height: 1;
    z-index: 1000;
    transition: all 0.2s;
    padding: 0;
  }

  .close-btn:hover {
    background: rgba(255, 95, 87, 0.3);
    color: rgba(255, 255, 255, 0.8);
  }

  .app {
    max-width: 800px;
    margin: 0 auto;
  }
</style>
