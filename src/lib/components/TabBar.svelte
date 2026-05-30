<script lang="ts">
  let { activeTab = 'throw', onchange }: {
    activeTab?: 'throw' | 'bottles';
    onchange?: (tab: 'throw' | 'bottles') => void;
  } = $props();
</script>

<div class="tab-bar">
  <button
    class="tab"
    class:active={activeTab === 'throw'}
    onclick={() => onchange?.('throw')}
  >
    Throw
  </button>
  <button
    class="tab"
    class:active={activeTab === 'bottles'}
    onclick={() => onchange?.('bottles')}
  >
    My Bottles
  </button>
</div>

<style>
  .tab-bar {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
    padding: 1.5rem 2rem 0;
  }

  .tab {
    padding: 0.6rem 2rem;
    border: 1px solid rgba(255,255,255,0.15);
    border-radius: 100px;
    background: rgba(255,255,255,0.05);
    color: rgba(255,255,255,0.6);
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tab.active {
    background: rgba(255,255,255,0.15);
    color: #fff;
    border-color: rgba(255,255,255,0.3);
  }

  .tab:hover {
    background: rgba(255,255,255,0.1);
  }
</style>
