<script lang="ts">
  import BottleIcon from './BottleIcon.svelte';

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

  $: sorted = [...messages].sort((a, b) => a.timestamp - b.timestamp);
</script>

<div class="bottle-list-container">
  <div class="list-header">
    <button class="back-btn" onclick={onBack}>←</button>
    <h3>{from}</h3>
    <div class="count">{messages.length}</div>
  </div>

  <div class="bottles-scroll">
    {#each sorted as msg (msg.id)}
      <button class="bottle-btn" onclick={() => onSelect(msg.id)} title={new Date(msg.timestamp * 1000).toLocaleString()}>
        <BottleIcon
          bodyLength={msg.body.length}
          encrypted={msg.encrypted}
          direction={msg.direction}
          maxLength={500}
        />
        <span class="time-label">{new Date(msg.timestamp * 1000).toLocaleDateString()}</span>
      </button>
    {/each}
  </div>
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
</style>
