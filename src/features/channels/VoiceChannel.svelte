<script>
  let { vc } = $props();
</script>

<div class="voice-channel">
  <button class="channel-item voice-channel-btn">
    <div class="channel-item-left">
      <span class="material-symbols-outlined" style="font-size:16px">volume_up</span>
      <span class="channel-name">{vc.name}</span>
    </div>
    {#if vc.users.length > 0}
      <span class="material-symbols-outlined voice-add-icon">person_add</span>
    {/if}
  </button>
  {#if vc.users.length > 0}
    <div class="voice-users">
      {#each vc.users as u}
        <div class="voice-user">
          <div class="voice-user-left">
            <div class="voice-user-avatar-wrap">
              <div class="voice-user-avatar">{u.name.substring(0,2)}</div>
              <div class="voice-user-dot"></div>
            </div>
            <span class="voice-user-name">{u.name}</span>
          </div>
          <span class="voice-user-ping">{u.ping}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .voice-channel { }
  .voice-channel-btn { }
  .voice-add-icon { font-size: 16px; opacity: 0; }
  .voice-channel-btn:hover :global(.voice-add-icon) { opacity: 1; }
  .voice-users { margin-left: 36px; margin-top: 4px; display: flex; flex-direction: column; gap: 8px; }
  .voice-user { display: flex; align-items: center; justify-content: space-between; padding-right: 8px; }
  .voice-user-left { display: flex; align-items: center; gap: 8px; }
  .voice-user-avatar-wrap { position: relative; }
  .voice-user-avatar {
    width: 24px; height: 24px; background: var(--surface-highest);
    display: flex; align-items: center; justify-content: center;
    font-size: 9px; font-weight: 700; color: var(--on-surface);
  }
  .voice-user-dot {
    position: absolute; bottom: -2px; right: -2px;
    width: 8px; height: 8px; background: var(--primary);
    border: 1.5px solid var(--surface-low); border-radius: 50%;
    animation: pulse 2s infinite;
  }
  .voice-user-name { font-size: 12px; color: var(--on-surface); }
  .voice-user-ping { font-family: var(--font-label); font-size: 9px; color: var(--primary); }

  /* channel-item styles needed locally because this renders inside channel-item context */
  :global(.channel-item) {
    width: 100%; display: flex; align-items: center; justify-content: space-between;
    padding: 8px 12px; background: none; border: none; cursor: pointer;
    color: var(--on-surface-variant); opacity: 0.7; transition: all 0.15s;
    text-align: left;
  }
  :global(.channel-item:hover) { background: var(--surface-high); opacity: 1; }
  :global(.channel-item-left) { display: flex; align-items: center; gap: 12px; }
  :global(.channel-name) { font-size: 14px; font-family: var(--font-body); }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }
</style>
