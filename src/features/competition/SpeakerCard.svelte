<script>
  let { name, initials, active = false, muted = false, isYou = false, audioLevel = 0 } = $props();
</script>

<div class="speaker-card" class:speaker-active={active} class:speaker-you={isYou}>
  {#if active}
    <div class="speaker-mic-icon">
      <span class="material-symbols-outlined" style="color:var(--error)">mic</span>
    </div>
  {/if}
  <div class="speaker-avatar" class:speaker-avatar-muted={muted}>{initials}</div>
  <span class="speaker-name" class:muted={muted} style={active ? 'color:var(--error)' : ''}>{name}</span>
  {#if isYou}
    <div class="speaker-controls">
      <span class="material-symbols-outlined" style="font-size:16px">mic_off</span>
      <span class="material-symbols-outlined" style="font-size:16px">volume_up</span>
    </div>
  {:else if active}
    <div class="speaker-bar"><div class="speaker-bar-fill" style="width:{audioLevel}%"></div></div>
  {:else}
    <div class="speaker-bar speaker-bar-empty"></div>
  {/if}
</div>

<style>
  .speaker-card {
    background: var(--surface-high); padding: 24px;
    display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px;
    border: 1px solid rgba(59,74,68,0.2); position: relative;
  }
  .speaker-active { border: 2px solid var(--error); background: var(--surface-high); }
  .speaker-you { border: 1px solid rgba(255,180,171,0.4); }
  .speaker-mic-icon { position: absolute; top: 12px; right: 12px; }
  .speaker-avatar {
    width: 80px; height: 80px; background: var(--surface-highest);
    display: flex; align-items: center; justify-content: center;
    font-size: 20px; font-weight: 700; color: var(--on-surface);
  }
  .speaker-avatar-muted { opacity: 0.5; filter: grayscale(1); }
  .speaker-name { font-family: var(--font-label); font-size: 12px; font-weight: 700; }
  .speaker-name.muted { color: var(--on-surface-variant); }
  .speaker-bar { width: 100%; height: 4px; background: rgba(255,180,171,0.1); margin-top: 8px; }
  .speaker-bar-fill { height: 100%; background: var(--error); }
  .speaker-bar-empty { background: rgba(255,255,255,0.05); }
  .speaker-controls { display: flex; gap: 8px; color: var(--on-surface-variant); }
</style>
