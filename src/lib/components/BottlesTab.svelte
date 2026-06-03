<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api';
  import { bottles, type BottleContent, type BottleMeta } from '../store';
  import { translator } from '$lib/i18n';
  import MessageBottleList from './MessageBottleList.svelte';

  interface BottleGroup {
    from: string;
    count: number;
    latest: BottleMeta;
    encryptedCount: number;
  }

  let loading = false;
  let replies: Record<string, string> = {};
  let replying: Record<string, boolean> = {};
  let replyStatus: Record<string, string> = {};
  let selectedSender: string | null = null;
  let activeBottle: BottleContent | null = null;
  let activeBottleId: string | null = null;
  let openingBottleId: string | null = null;

  $: groups = groupBottles($bottles);
  $: selectedMessages = selectedSender
    ? $bottles
        .filter((b) => b.from === selectedSender || (b.direction === 'sent' && b.to === selectedSender))
        .map((b) => ({
          id: b.id,
          body: '',
          timestamp: b.timestamp,
          encrypted: b.encrypted,
          direction: (b.direction || 'received') as 'sent' | 'received',
        }))
    : [];

  onMount(() => {
    refresh();
  });

  async function refresh() {
    loading = true;
    try {
      const [received, sent] = await Promise.all([
        api.fetchBottles(),
        api.fetchSentBottles(),
      ]);
      const allBottles = [
        ...received,
        ...sent.map((b) => ({ ...b, direction: 'sent' as const })),
      ];
      bottles.set(allBottles);
    } catch { /* ignore */ }
    loading = false;
  }

  function groupBottles(items: BottleMeta[]): BottleGroup[] {
    const bySender = new Map<string, BottleMeta[]>();
    for (const bottle of items) {
      const key = bottle.direction === 'sent' ? bottle.to || bottle.from : bottle.from;
      const list = bySender.get(key) || [];
      list.push(bottle);
      bySender.set(key, list);
    }

    return [...bySender.entries()]
      .map(([from, list]) => {
        const sorted = [...list].sort((a, b) => b.timestamp - a.timestamp);
        return {
          from,
          count: list.length,
          latest: sorted[0],
          encryptedCount: list.filter((b) => b.encrypted).length,
        };
      })
      .sort((a, b) => b.latest.timestamp - a.latest.timestamp);
  }

  async function open(id: string) {
    openingBottleId = id;
    try {
      const content = await api.openBottle(id);
      activeBottle = content;
      activeBottleId = id;
    } catch (e: unknown) {
      alert(typeof e === 'string' ? e : 'Failed to open');
    } finally {
      openingBottleId = null;
    }
  }

  function openSender(from: string) {
    selectedSender = from;
    activeBottle = null;
    activeBottleId = null;
  }

  function closeSenderView() {
    selectedSender = null;
    activeBottle = null;
    activeBottleId = null;
  }

  async function quickReply(to: string) {
    const body = replies[to]?.trim();
    if (!body) return;

    replying = { ...replying, [to]: true };
    replyStatus = { ...replyStatus, [to]: '' };
    try {
      await api.throwBottle(to, body);
      replies = { ...replies, [to]: '' };
      replyStatus = { ...replyStatus, [to]: $translator('bottleSent') };
    } catch (e: unknown) {
      replyStatus = { ...replyStatus, [to]: typeof e === 'string' ? e : 'Failed to send' };
    } finally {
      replying = { ...replying, [to]: false };
    }
  }

  async function deleteBottle() {
    if (!activeBottle) return;
    if (!confirm($translator('confirmDelete') || 'Delete this message?')) return;

    try {
      await api.deleteBottle(activeBottle.id);
      activeBottle = null;
      activeBottleId = null;
      await refresh();
    } catch (e: unknown) {
      alert(typeof e === 'string' ? e : 'Failed to delete');
    }
  }

  function formatTime(ts: number): string {
    return new Date(ts * 1000).toLocaleString();
  }
</script>

<div class="bottles-container">
  {#if selectedSender}
    <MessageBottleList
      from={selectedSender}
      messages={selectedMessages}
      onSelect={open}
      onBack={closeSenderView}
      activeMessage={activeBottle}
      activeMessageId={activeBottleId}
      openingMessageId={openingBottleId}
      onDelete={deleteBottle}
    />
  {:else}
    <div class="glass bottles-card">
      <div class="header">
        <h2>{$translator('myBottles')}</h2>
        <button class="icon-btn" onclick={refresh} disabled={loading}>
          {loading ? '...' : '↻'}
        </button>
      </div>

      {#if groups.length === 0}
        <p class="empty">{$translator('noBottles')}</p>
      {:else}
        <div class="list">
          {#each groups as group (group.from)}
            <section class="sender-row glass">
              <button class="sender-main" onclick={() => openSender(group.from)} aria-label={$translator('openLatest')}>
                <div class="sender-icon">✉</div>
                <div class="sender-info">
                  <span class="from">{group.from}</span>
                  <span class="time">{$translator('latest')}: {formatTime(group.latest.timestamp)}</span>
                </div>
                <div class="count-pill">
                  <span>×{group.count}</span>
                  <small>{$translator('bottles')}</small>
                </div>
              </button>

              <div class="row-meta">
                {#if group.encryptedCount > 0}
                  <span class="badge locked">🔒 {group.encryptedCount}</span>
                {/if}
                <span class="badge">{group.count - group.encryptedCount} {$translator('plain')}</span>
              </div>

              <form class="quick-reply" onsubmit={(e) => { e.preventDefault(); quickReply(group.from); }}>
                <input bind:value={replies[group.from]} placeholder={$translator('replyPlaceholder')} />
                <button type="submit" disabled={replying[group.from] || !replies[group.from]?.trim()}>
                  {replying[group.from] ? $translator('replying') : $translator('reply')}
                </button>
              </form>

              {#if replyStatus[group.from]}
                <p class="reply-status">{replyStatus[group.from]}</p>
              {/if}
            </section>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .bottles-container {
    max-width: 680px;
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
    color: rgba(255,255,255,0.62);
    text-align: center;
    padding: 3rem 0;
  }

  .list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .sender-row {
    padding: 1rem;
  }

  .sender-main {
    width: 100%;
    display: grid;
    grid-template-columns: auto 1fr auto;
    gap: 0.85rem;
    align-items: center;
    padding: 0;
    border: none;
    background: transparent;
    color: inherit;
    text-align: left;
    cursor: pointer;
  }

  .sender-icon {
    width: 38px;
    height: 38px;
    display: grid;
    place-items: center;
    border-radius: 50%;
    background: rgba(255,255,255,0.12);
    border: 1px solid rgba(255,255,255,0.18);
  }

  .sender-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    min-width: 0;
  }

  .from {
    color: #fff;
    font-weight: 700;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .time {
    color: rgba(255,255,255,0.62);
    font-size: 0.8rem;
  }

  .count-pill {
    display: flex;
    flex-direction: column;
    align-items: center;
    min-width: 64px;
    padding: 0.35rem 0.65rem;
    border-radius: 14px;
    background: rgba(123, 218, 255, 0.16);
    border: 1px solid rgba(255,255,255,0.18);
  }

  .count-pill span {
    font-size: 1.05rem;
    font-weight: 700;
  }

  .count-pill small {
    color: rgba(255,255,255,0.62);
    font-size: 0.7rem;
  }

  .row-meta {
    display: flex;
    gap: 0.4rem;
    margin: 0.75rem 0;
  }

  .badge {
    font-size: 0.75rem;
    padding: 0.2rem 0.5rem;
    border-radius: 999px;
    background: rgba(255,255,255,0.1);
    color: rgba(255,255,255,0.72);
  }

  .badge.locked {
    background: rgba(107, 255, 184, 0.15);
    color: #6bffb8;
  }

  .quick-reply {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0.5rem;
  }

  .quick-reply input {
    min-width: 0;
    padding: 0.65rem 0.85rem;
    border: 1px solid rgba(255,255,255,0.18);
    border-radius: 12px;
    background: rgba(255,255,255,0.08);
    color: #fff;
    outline: none;
  }

  .quick-reply input::placeholder {
    color: rgba(255,255,255,0.45);
  }

  .quick-reply button {
    padding: 0.65rem 0.9rem;
    border: none;
    border-radius: 12px;
    background: rgba(255,255,255,0.16);
    color: #fff;
    cursor: pointer;
  }

  .quick-reply button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .reply-status {
    margin: 0.6rem 0 0;
    color: #6bffb8;
    font-size: 0.85rem;
  }
</style>
