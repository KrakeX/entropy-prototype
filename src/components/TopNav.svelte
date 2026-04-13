<script>
  import { app, goBack, toggleCompetition } from '../stores/app.svelte.js';
</script>

<nav class="topbar" class:competition-topbar={app.competitionMode}>
  <div class="topbar-left">
    {#if app.currentView === 'channel'}
      <button class="icon-btn" on:click={goBack}>
        <span class="material-symbols-outlined">arrow_back</span>
      </button>
    {/if}
    <div class="logo" class:logo-competition={app.competitionMode}>
      <span class="material-symbols-outlined logo-icon" style="font-variation-settings:'FILL' 1">
        {app.competitionMode ? 'security' : 'deployed_code'}
      </span>
      <span>ENTROPY</span>
    </div>
    {#if app.currentView === 'channel' && app.currentServer}
      <div class="topbar-divider"></div>
      <span class="server-name-header">{app.currentServer.name}</span>
      <span class="badge-public">{app.currentServer.type === 'private' ? 'Self-Hosted' : 'Red Pública'}</span>
      <span class="badge-e2e">
        <span class="material-symbols-outlined" style="font-size:12px;font-variation-settings:'FILL' 1">verified_user</span>
        E2E ACTIVE
      </span>
    {/if}
  </div>

  <div class="topbar-right">
    {#if app.competitionMode}
      <div class="competition-badge">
        <div class="competition-dot"></div>
        <span>MODO COMPETICIÓN</span>
      </div>
    {:else}
      <div class="e2e-badge">
        <span class="material-symbols-outlined" style="font-size:14px">shield</span>
        <span>E2E ENCRYPTED</span>
      </div>
    {/if}
    <div class="toggle-wrap" class:toggle-active={app.competitionMode} on:click={toggleCompetition}>
      <span class="toggle-label">{app.competitionMode ? 'MODO COMPETICIÓN' : 'Normal'}</span>
      <span class="material-symbols-outlined toggle-icon">{app.competitionMode ? 'toggle_on' : 'toggle_off'}</span>
    </div>
    <button class="icon-btn">
      <span class="material-symbols-outlined">settings</span>
    </button>
  </div>
</nav>

<style>
  .topbar {
    position: fixed; top: 0; left: 0; right: 0; z-index: 50;
    height: 64px; display: flex; align-items: center;
    justify-content: space-between; padding: 0 24px;
    background: var(--surface);
  }
  .competition-topbar { border-bottom: 1px solid rgba(255,180,171,0.2); }

  .topbar-left { display: flex; align-items: center; gap: 12px; }
  .topbar-right { display: flex; align-items: center; gap: 12px; }

  .logo {
    display: flex; align-items: center; gap: 8px;
    font-family: var(--font-label); font-size: 18px;
    font-weight: 900; color: var(--primary-container);
    text-transform: uppercase; letter-spacing: -0.04em;
  }
  .logo-competition { color: var(--error); }
  .logo-icon { font-size: 28px; }

  .topbar-divider { width: 1px; height: 24px; background: var(--outline-variant); opacity: 0.3; }
  .server-name-header { font-family: var(--font-headline); font-size: 16px; font-weight: 700; }

  .badge-public {
    font-family: var(--font-label); font-size: 9px;
    padding: 2px 6px; border: 1px solid rgba(0,212,170,0.4);
    color: var(--primary-container); text-transform: uppercase; letter-spacing: 0.05em;
  }
  .badge-e2e {
    display: flex; align-items: center; gap: 4px;
    font-family: var(--font-label); font-size: 9px;
    padding: 2px 8px; background: rgba(70,241,197,0.1);
    color: var(--primary); text-transform: uppercase;
  }

  .e2e-badge {
    display: flex; align-items: center; gap: 6px;
    padding: 4px 12px; border: 1px solid rgba(70,241,197,0.3);
    background: rgba(70,241,197,0.05); color: var(--primary);
    font-family: var(--font-label); font-size: 10px; letter-spacing: 0.15em;
  }
  .competition-badge {
    display: flex; align-items: center; gap: 8px;
    padding: 4px 12px; background: rgba(147,0,10,0.2);
    border: 1px solid rgba(255,180,171,0.3);
    font-family: var(--font-label); font-size: 10px;
    color: var(--error); font-weight: 700; text-transform: uppercase;
  }
  .competition-dot {
    width: 8px; height: 8px; border-radius: 50%;
    background: var(--error); animation: pulse 1.5s infinite;
  }

  .toggle-wrap {
    display: flex; align-items: center; gap: 6px;
    padding: 4px 12px; background: var(--surface-high);
    cursor: pointer; transition: background 0.15s;
    font-family: var(--font-label); font-size: 10px;
    color: var(--on-surface-variant); text-transform: uppercase; letter-spacing: 0.1em;
  }
  .toggle-wrap:hover { background: var(--surface-highest); }
  .toggle-active { background: rgba(255,180,171,0.1); color: var(--error); }
  .toggle-icon { font-size: 26px; color: var(--primary); }
  .toggle-active .toggle-icon { color: var(--error); }

  /* icon-btn viene de app.css como global */
  :global(.icon-btn) {
    padding: 8px; background: none; border: none; cursor: pointer;
    color: var(--on-surface-variant); transition: background 0.15s;
  }
  :global(.icon-btn:hover) { background: var(--surface-highest); }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }
</style>
