<script>
  import { app } from '../../stores/app.svelte.js';
  import { textChannels, voiceChannels } from '../../data/mock.js';
  import VoiceChannel from './VoiceChannel.svelte';
</script>

<aside class="sidebar" class:sidebar-competition={app.competitionMode}>
  <!-- Channels -->
  <div class="channel-list">
    <div class="channel-section">
      <div class="channel-section-label">Canales de Texto</div>
      {#each textChannels as ch}
        <button
          class="channel-item"
          class:channel-item-active={app.selectedChannel?.id === ch.id}
          class:channel-item-competition={app.competitionMode}
          on:click={() => app.selectedChannel = ch}
        >
          <div class="channel-item-left">
            <span class="material-symbols-outlined" style="font-size:16px">tag</span>
            <span class="channel-name">{ch.name}</span>
          </div>
          {#if ch.unread > 0 && !app.competitionMode}
            <span class="unread-badge">{ch.unread}</span>
          {/if}
        </button>
      {/each}
    </div>

    <div class="channel-section">
      <div class="channel-section-label">Canales de Voz</div>
      {#each voiceChannels as vc}
        <VoiceChannel {vc} />
      {/each}
    </div>
  </div>

  <!-- Sidebar Footer -->
  <div class="sidebar-footer" class:sidebar-footer-competition={app.competitionMode}>
    <div class="sidebar-footer-encryption">
      <div class="footer-enc-icon">
        <span class="material-symbols-outlined" style="font-size:16px;color:var(--primary)">lock</span>
      </div>
      <div>
        <div class="footer-enc-label">CIFRADO ACTIVO</div>
        <div class="footer-enc-sub">Prot: E2E-ENTROPY</div>
      </div>
    </div>
    <div class="sidebar-footer-stats">
      <div>
        <div class="footer-stat-label">Latencia</div>
        <div class="footer-stat-value" style="color:var(--primary)">{app.currentServer?.ping}ms Stable</div>
      </div>
      <div>
        <div class="footer-stat-label">Network</div>
        <div class="footer-stat-value">{app.currentServer?.type === 'private' ? 'Self-Hosted' : 'Red Pública'}</div>
      </div>
    </div>
    <button class="add-server-btn-sm" on:click={() => { app.currentView = 'welcome'; app.addServerPanel = true; }}>
      NUEVO SERVIDOR
    </button>
    <div class="user-controls-sm">
      <button class="icon-btn" class:icon-btn-competition={app.competitionMode}><span class="material-symbols-outlined">mic</span></button>
      <button class="icon-btn" class:icon-btn-competition={app.competitionMode}><span class="material-symbols-outlined">volume_up</span></button>
      <button class="icon-btn" class:icon-btn-competition={app.competitionMode}><span class="material-symbols-outlined">settings</span></button>
    </div>
  </div>
</aside>

<style>
  .sidebar {
    width: 280px; flex-shrink: 0; background: var(--surface-low);
    display: flex; flex-direction: column; overflow: hidden;
  }
  .sidebar-competition { /* colors cascade */ }

  .channel-list { flex: 1; overflow-y: auto; padding: 24px 0; display: flex; flex-direction: column; gap: 32px; }
  .channel-list::-webkit-scrollbar { display: none; }
  .channel-section { padding: 0 16px; }
  .channel-section-label {
    padding: 0 8px 12px;
    font-family: var(--font-label); font-size: 10px; font-weight: 700;
    color: var(--on-surface-variant); text-transform: uppercase;
    letter-spacing: 0.2em; opacity: 0.6;
  }
  .channel-item {
    width: 100%; display: flex; align-items: center; justify-content: space-between;
    padding: 8px 12px; background: none; border: none; cursor: pointer;
    color: var(--on-surface-variant); opacity: 0.7; transition: all 0.15s;
    text-align: left;
  }
  .channel-item:hover { background: var(--surface-high); opacity: 1; }
  .channel-item-active { background: var(--surface-high); color: var(--primary-container); border-left: 2px solid var(--primary-container); opacity: 1; font-weight: 700; }
  .channel-item-competition.channel-item-active { color: var(--error); border-left-color: var(--error); }
  .channel-item-left { display: flex; align-items: center; gap: 12px; }
  .channel-name { font-size: 14px; font-family: var(--font-body); }
  .unread-badge {
    background: var(--primary); color: var(--on-primary);
    font-family: var(--font-label); font-size: 10px; font-weight: 700;
    padding: 1px 6px;
  }

  .sidebar-footer { padding: 16px; border-top: 1px solid rgba(59,74,68,0.1); }
  .sidebar-footer-encryption { display: flex; align-items: center; gap: 8px; margin-bottom: 12px; }
  .footer-enc-icon {
    width: 32px; height: 32px; background: var(--surface-highest);
    display: flex; align-items: center; justify-content: center;
  }
  .footer-enc-label { font-family: var(--font-label); font-size: 10px; color: var(--primary); }
  .footer-enc-sub { font-family: var(--font-label); font-size: 9px; color: var(--on-surface-variant); opacity: 0.6; text-transform: uppercase; }
  .sidebar-footer-stats { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 12px; }
  .footer-stat-label { font-family: var(--font-label); font-size: 8px; color: var(--on-surface-variant); opacity: 0.5; text-transform: uppercase; }
  .footer-stat-value { font-family: var(--font-label); font-size: 11px; color: var(--on-surface); }

  .add-server-btn-sm {
    width: 100%; padding: 10px;
    background: var(--primary-container); color: var(--on-primary);
    border: none; cursor: pointer; font-family: var(--font-label);
    font-size: 11px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.1em;
    transition: filter 0.15s; margin-bottom: 12px;
  }
  .add-server-btn-sm:hover { filter: brightness(1.1); }
  .sidebar-footer-competition .add-server-btn-sm { background: var(--error); color: var(--error-container); }
  .user-controls-sm { display: flex; justify-content: space-around; padding-top: 12px; border-top: 1px solid rgba(59,74,68,0.1); }
</style>
