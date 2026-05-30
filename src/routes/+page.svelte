<script lang="ts">
  import '../app.css';
  import { session, welcomeMessage } from '$lib/store';
  import { api } from '$lib/api';
  import { translator } from '$lib/i18n';
  import AuthPanel from '$lib/components/AuthPanel.svelte';
  import TabBar from '$lib/components/TabBar.svelte';
  import ThrowTab from '$lib/components/ThrowTab.svelte';
  import BottlesTab from '$lib/components/BottlesTab.svelte';

  let activeTab: 'throw' | 'bottles' = 'throw';
  let welcomeTimer: ReturnType<typeof setTimeout> | null = null;

  $: if ($welcomeMessage) {
    if (welcomeTimer) clearTimeout(welcomeTimer);
    welcomeTimer = setTimeout(() => {
      welcomeMessage.set(null);
      welcomeTimer = null;
    }, 10000);
  }

  function handleTabChange(tab: 'throw' | 'bottles') {
    activeTab = tab;
  }

  async function minimizeWindow() {
    await api.minimizeApp();
  }

  async function closeWindow() {
    await api.exitApp();
  }
</script>

<svelte:head>
  <title>Bottles - 漂流瓶</title>
</svelte:head>

<div class="drag-region" data-tauri-drag-region></div>

<div class="window-controls">
  <button class="window-btn minimize-btn" onclick={minimizeWindow} aria-label={$translator('minimize')}>−</button>
  <button class="window-btn close-btn" onclick={closeWindow} aria-label={$translator('close')}>&times;</button>
</div>

{#if $welcomeMessage}
  <div class="welcome-bubble" role="status">
    {$welcomeMessage}
  </div>
{/if}

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
  .welcome-bubble {
    position: fixed;
    top: 2.25rem;
    left: 50%;
    z-index: 900;
    width: min(90vw, 560px);
    transform: translateX(-50%);
    padding: 1rem 1.25rem;
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: 999px;
    background: rgba(3, 67, 129, 0.72);
    box-shadow: 0 14px 40px rgba(0, 18, 46, 0.28), inset 0 1px 0 rgba(255, 255, 255, 0.22);
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    color: rgba(255, 255, 255, 0.96);
    text-align: center;
    line-height: 1.35;
    animation: bubble-float 10s ease-in-out forwards;
  }

  @keyframes bubble-float {
    0% {
      opacity: 0;
      transform: translate(-50%, -12px) scale(0.96);
    }

    8%, 86% {
      opacity: 1;
      transform: translate(-50%, 0) scale(1);
    }

    100% {
      opacity: 0;
      transform: translate(-50%, -16px) scale(0.98);
    }
  }

  .drag-region {
    position: fixed;
    inset: 0;
    z-index: 0;
  }

  .window-controls {
    position: fixed;
    top: 8px;
    right: 8px;
    z-index: 1000;
    display: flex;
    gap: 6px;
  }

  .window-btn {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: none;
    border-radius: 5px;
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.55);
    cursor: pointer;
    font-size: 15px;
    line-height: 1;
    transition: all 0.2s;
  }

  .window-btn:hover {
    background: rgba(255, 255, 255, 0.2);
    color: rgba(255, 255, 255, 0.9);
  }

  .minimize-btn {
    font-size: 18px;
  }

  .close-btn:hover {
    background: rgba(255, 95, 87, 0.42);
  }

  .app {
    position: relative;
    z-index: 1;
    max-width: 800px;
    margin: 0 auto;
  }
</style>
