<script lang="ts">
  import { api } from '../api';
  import { bottles } from '../store';
  import { translator } from '$lib/i18n';

  let to = '';
  let message = '';
  let sending = false;
  let status = '';
  let isError = false;

  async function send() {
    if (!to.trim() || !message.trim()) return;
    sending = true;
    status = '';
    try {
      await api.throwBottle(to.trim(), message);
      status = $translator('bottleSent');
      isError = false;
      message = '';
      to = '';
      bottles.set(await api.fetchBottles());
    } catch (e: unknown) {
      status = typeof e === 'string' ? e : 'Failed to send';
      isError = true;
    } finally {
      sending = false;
    }
  }
</script>

<div class="glass throw-card">
  <h2>{$translator('throwBottle')}</h2>
  <form onsubmit={(e) => { e.preventDefault(); send(); }}>
    <input
      bind:value={to}
      placeholder={$translator('recipient')}
      maxlength={32}
      required
    />
    <textarea
      bind:value={message}
      placeholder={$translator('message')}
      maxlength={131072}
      required
    ></textarea>
    <div class="char-count">{message.length} / 131072</div>
    <button type="submit" disabled={sending}>
      {sending ? $translator('throwing') : $translator('throwIntoOcean')}
    </button>
  </form>
  {#if status}
    <p class:error={isError}>{status}</p>
  {/if}
</div>

<style>
  .throw-card {
    padding: 2rem;
    max-width: 600px;
    margin: 2rem auto;
  }

  h2 {
    margin: 0 0 1.5rem;
    color: #fff;
    font-size: 1.3rem;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  input, textarea {
    padding: 0.75rem 1rem;
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 12px;
    background: rgba(255,255,255,0.08);
    color: #fff;
    font-size: 1rem;
    outline: none;
    font-family: inherit;
    transition: border-color 0.2s;
  }

  textarea {
    min-height: 200px;
    resize: vertical;
  }

  input:focus, textarea:focus {
    border-color: rgba(255,255,255,0.5);
  }

  input::placeholder, textarea::placeholder {
    color: rgba(255,255,255,0.4);
  }

  .char-count {
    text-align: right;
    font-size: 0.8rem;
    color: rgba(255,255,255,0.4);
  }

  button {
    padding: 0.85rem;
    border: none;
    border-radius: 12px;
    background: rgba(255,255,255,0.15);
    color: #fff;
    font-size: 1rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  button:hover:not(:disabled) {
    background: rgba(255,255,255,0.25);
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  p {
    margin: 1rem 0 0;
    color: #6bffb8;
    font-size: 0.9rem;
    text-align: center;
  }

  .error {
    color: #ff6b6b;
  }
</style>
