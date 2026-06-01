<script lang="ts">
  export let bodyLength: number = 0;
  export let encrypted: boolean = false;
  export let direction: 'sent' | 'received' = 'received';
  export let maxLength: number = 500;

  const waterLevel = Math.min((bodyLength / maxLength) * 100, 100);
</script>

<div class="bottle-container" class:encrypted>
  <svg viewBox="0 0 60 100" class="bottle">
    <!-- 瓶身 -->
    <path d="M 15 20 L 12 35 Q 10 50 15 70 Q 15 85 30 90 Q 45 85 45 70 Q 50 50 48 35 L 45 20 Z"
          fill="none" stroke="rgba(255,255,255,0.3)" stroke-width="1.5"/>

    <!-- 水位 -->
    <defs>
      <clipPath id="bottle-clip">
        <path d="M 15 20 L 12 35 Q 10 50 15 70 Q 15 85 30 90 Q 45 85 45 70 Q 50 50 48 35 L 45 20 Z"/>
      </clipPath>
    </defs>

    <rect x="12" y={90 - waterLevel * 0.7} width="36" height={waterLevel * 0.7}
          fill="rgba(123, 218, 255, 0.4)" clip-path="url(#bottle-clip)"/>

    <!-- 瓶盖 -->
    <rect x="24" y="8" width="12" height="12" rx="2" fill="rgba(255,255,255,0.2)" stroke="rgba(255,255,255,0.3)" stroke-width="1"/>

    <!-- 方向指示 -->
    <text x="30" y="55" text-anchor="middle" font-size="20" fill="rgba(255,255,255,0.5)">
      {direction === 'sent' ? '→' : '←'}
    </text>
  </svg>

  {#if encrypted}
    <div class="lock-icon">🔒</div>
  {/if}
</div>

<style>
  .bottle-container {
    position: relative;
    width: 60px;
    height: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .bottle {
    width: 100%;
    height: 100%;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
  }

  .lock-icon {
    position: absolute;
    bottom: 8px;
    right: -8px;
    font-size: 16px;
    background: rgba(107, 255, 184, 0.2);
    border-radius: 50%;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid rgba(107, 255, 184, 0.4);
  }
</style>
