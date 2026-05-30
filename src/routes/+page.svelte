<script lang="ts">
  import '../app.css';
  import { session } from '$lib/store';
  import AuthPanel from '$lib/components/AuthPanel.svelte';
  import TabBar from '$lib/components/TabBar.svelte';
  import ThrowTab from '$lib/components/ThrowTab.svelte';
  import BottlesTab from '$lib/components/BottlesTab.svelte';

  let activeTab: 'throw' | 'bottles' = 'throw';
</script>

<svelte:head>
  <title>Bottles - 漂流瓶</title>
</svelte:head>

{#if !$session}
  <AuthPanel />
{:else}
  <div class="app">
    <TabBar bind:activeTab />
    {#if activeTab === 'throw'}
      <ThrowTab />
    {:else}
      <BottlesTab />
    {/if}
  </div>
{/if}

<style>
  .app {
    max-width: 800px;
    margin: 0 auto;
  }
</style>
