<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api';
  import { bottles, currentBottle } from '../store';

  let loading = false;

  onMount(() => {
    refresh();
  });

  async function refresh() {
    loading = true;
    try {
      bottles.set(await api.fetchBottles());
    } catch { /* ignore */ }
    loading = false;
  }

  async function open(id: string) {
    try {
      const content = await api.openBottle(id);
      currentBottle.set(content);
    } catch (e: unknown) {
      alert(typeof e === 'string' ? e : 'Failed to open');
    }
  }

  async function remove(id: string) {
    try {
      await api.deleteBottle(id);
      bottles.update((b) => b.filter((x) => x.id !== id));
      currentBottle.update((c) => (c?.id === id ? null : c));
    } catch (e: unknown) {
      alert(typeof e === 'string' ? e : 'Failed to delete');
    }
  }

  function closeModal() {
    currentBottle.set(null);
  }

  function formatTime(ts: number): string {
    return new Date(ts).toLocaleString();
  }
</script>

<div class="bottles-container">
  <div class="glass bottles-card">
    <div class="header">
      <h2>My Bottles</h2>
      <button class="icon-btn" onclick={refresh} disabled={loading}>
        {loading ? '...' : '\u21BB'}
      </button>
    </div>

    {#if $bottles.length === 0}
      <p class="empty">No bottles yet. Waiting for the tide...</p>
    {:else}
      <div class="list">
        {#each $bottles as b (b.id)}
          <div class="bottle-item glass" role="button" tabindex="0" onkeydown={(e) => { if (e.key === 'Enter') open(b.id); }} onclick={() => open(b.id)}>
            <div class="bottle-info">
              <span class="from">{b.from}</span>
              <span class="time">{formatTime(b.timestamp)}</span>
            </div>
            <div class="bottle-meta">
              {#if b.encrypted}
                <span class="badge locked">Encrypted</span>
              {:else}
                <span class="badge">Plain</span>
              {/if}
              <button class="delete-btn" onclick={(e) => { e.stopPropagation(); remove(b.id); }}>
                &times;
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

{#if $currentBottle}
  <div class="modal-overlay" role="presentation" onclick={closeModal}>
    <div class="glass modal" role="dialog" tabindex="-1" onkeydown={(e) => { if (e.key === 'Escape') closeModal(); }} onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <span class="from-label">From: {$currentBottle.from}</span>
        <span class="time-label">{formatTime($currentBottle.timestamp)}</span>
        <button class="icon-btn" onclick={closeModal}>&times;</button>
      </div>
      <pre class="modal-body">{$currentBottle.body}</pre>
    </div>
  </div>
{/if}

<style>
  .bottles-container {
    max-width: 600px;
    margin: 2rem auto;
  }

  .bottles-card {
    padding: 2rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  h2 {
    margin: 0;
    color: #fff;
    font-size: 1.3rem;
  }

  .icon-btn {
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 8px;
    color: #fff;
    padding: 0.4rem 0.8rem;
    cursor: pointer;
    font-size: 1rem;
  }

  .icon-btn:disabled {
    opacity: 0.5;
  }

  .empty {
    color: rgba(255,255,255,0.4);
    text-align: center;
    padding: 3rem 0;
  }

  .list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .bottle-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .bottle-item:hover {
    background: rgba(255,255,255,0.08);
  }

  .bottle-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .from {
    color: #fff;
    font-weight: 500;
  }

  .time {
    color: rgba(255,255,255,0.4);
    font-size: 0.8rem;
  }

  .bottle-meta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .badge {
    font-size: 0.75rem;
    padding: 0.2rem 0.5rem;
    border-radius: 6px;
    background: rgba(255,255,255,0.1);
    color: rgba(255,255,255,0.6);
  }

  .badge.locked {
    background: rgba(107, 255, 184, 0.15);
    color: #6bffb8;
  }

  .delete-btn {
    background: none;
    border: none;
    color: rgba(255,255,255,0.3);
    cursor: pointer;
    font-size: 1.2rem;
    padding: 0.25rem;
  }

  .delete-btn:hover {
    color: #ff6b6b;
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 100;
    padding: 2rem;
    backdrop-filter: blur(4px);
  }

  .modal {
    width: 100%;
    max-width: 600px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    padding: 1.5rem;
  }

  .modal-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .from-label {
    color: #fff;
    font-weight: 500;
  }

  .time-label {
    color: rgba(255,255,255,0.4);
    font-size: 0.8rem;
    flex: 1;
  }

  .modal-body {
    overflow-y: auto;
    color: rgba(255,255,255,0.9);
    white-space: pre-wrap;
    word-break: break-word;
    font-family: inherit;
    margin: 0;
    line-height: 1.6;
  }
</style>
