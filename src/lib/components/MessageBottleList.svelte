<script lang="ts">
  import BottleIcon from './BottleIcon.svelte';
  import { translator } from '$lib/i18n';
  import type { BottleContent } from '$lib/store';

  interface Message {
    id: string;
    body: string;
    timestamp: number;
    encrypted: boolean;
    direction: 'sent' | 'received';
  }

  export let from: string;
  export let messages: Message[] = [];
  export let onSelect: (id: string) => void = () => {};
  export let onBack: () => void = () => {};
  export let onDelete: () => void = () => {};
  export let activeMessage: BottleContent | null = null;
  export let activeMessageId: string | null = null;
  export let openingMessageId: string | null = null;

  $: sorted = [...messages].sort((a, b) => a.timestamp - b.timestamp);
  $: activeMeta = activeMessageId ? messages.find((message) => message.id === activeMessageId) ?? null : null;

  function formatTime(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleString();
  }
</script>

<div class="bottle-list-container">
  <div class="list-header">
    <button class="back-btn" onclick={onBack}>←</button>
    <h3>{from}</h3>
    <div class="count">{messages.length}</div>
  </div>

  <div class="bottles-scroll">
    {#each sorted as msg (msg.id)}
      <button
        class="bottle-btn"
        class:active={activeMessageId === msg.id}
        onclick={() => onSelect(msg.id)}
        title={new Date(msg.timestamp * 1000).toLocaleString()}
      >
        <BottleIcon
          bodyLength={msg.body.length}
          encrypted={msg.encrypted}
          direction={msg.direction}
          maxLength={500}
        />
        <span class="time-label">
          {openingMessageId === msg.id ? '...' : new Date(msg.timestamp * 1000).toLocaleDateString()}
        </span>
      </button>
    {/each}
  </div>

  {#if activeMessage && activeMeta}
    <article class="glass message-panel" class:sent={activeMeta.direction === 'sent'}>
      <div class="message-header">
        <div>
          <span class="direction-chip">{activeMeta.direction === 'sent' ? $translator('sent') : $translator('from')}</span>
          <strong>{activeMeta.direction === 'sent' ? from : activeMessage.from}</strong>
        </div>
        <div class="message-actions">
          <span class="message-time">{formatTime(activeMessage.timestamp)}</span>
          <button class="delete-btn" type="button" onclick={onDelete}>{$translator('delete')}</button>
        </div>
      </div>
      <pre class="message-body">{activeMessage.body}</pre>
    </article>
  {:else}
    <div class="glass empty-state">
      <p>{$translator('selectBottle')}</p>
    </div>
  {/if}
</div>

<style>
  .bottle-list-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.5rem;
    max-width: 680px;
    margin: 2rem auto;
  }

  .list-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 0.5rem;
  }

  .back-btn {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    color: #fff;
    padding: 0.4rem 0.8rem;
    cursor: pointer;
    font-size: 1rem;
  }

  .back-btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  h3 {
    margin: 0;
    color: #fff;
    font-size: 1.1rem;
    flex: 1;
  }

  .count {
    color: rgba(255, 255, 255, 0.62);
    font-size: 0.9rem;
    padding: 0.4rem 0.8rem;
    background: rgba(255, 255, 255, 0.08);
    border-radius: 8px;
  }

  .bottles-scroll {
    display: flex;
    gap: 1rem;
    overflow-x: auto;
    padding: 1rem 0;
    scroll-behavior: smooth;
  }

  .bottles-scroll::-webkit-scrollbar {
    height: 6px;
  }

  .bottles-scroll::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 3px;
  }

  .bottles-scroll::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    border-radius: 3px;
  }

  .bottle-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 8px;
    transition: background 0.2s;
    flex-shrink: 0;
  }

  .bottle-btn.active {
    background: rgba(255, 255, 255, 0.12);
    box-shadow: inset 0 0 0 1px rgba(123, 218, 255, 0.36);
  }

  .bottle-btn:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .time-label {
    color: rgba(255, 255, 255, 0.62);
    font-size: 0.7rem;
    text-align: center;
    max-width: 70px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .message-panel,
  .empty-state {
    padding: 1rem 1.1rem;
    min-height: 220px;
  }

  .message-panel.sent {
    background: linear-gradient(180deg, rgba(123, 218, 255, 0.18), rgba(255, 255, 255, 0.08));
  }

  .message-header {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    align-items: flex-start;
    margin-bottom: 0.9rem;
  }

  .message-header strong {
    display: block;
    margin-top: 0.25rem;
    color: #fff;
    font-size: 1rem;
  }

  .direction-chip {
    display: inline-flex;
    align-items: center;
    padding: 0.18rem 0.5rem;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.72);
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .message-actions {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.55rem;
  }

  .message-time {
    color: rgba(255, 255, 255, 0.62);
    font-size: 0.78rem;
  }

  .delete-btn {
    border: 1px solid rgba(255, 255, 255, 0.16);
    border-radius: 999px;
    background: rgba(255, 96, 96, 0.15);
    color: #fff;
    cursor: pointer;
    padding: 0.45rem 0.8rem;
  }

  .message-body {
    margin: 0;
    white-space: pre-wrap;
    word-break: break-word;
    color: rgba(255, 255, 255, 0.94);
    font-size: 0.95rem;
    line-height: 1.55;
  }

  .empty-state {
    display: grid;
    place-items: center;
    text-align: center;
  }

  .empty-state p {
    margin: 0;
    color: rgba(255, 255, 255, 0.62);
  }

  @media (max-width: 640px) {
    .message-header {
      flex-direction: column;
    }

    .message-actions {
      align-items: flex-start;
    }
  }
</style>
