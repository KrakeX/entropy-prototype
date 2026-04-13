<script>
  import { app } from '../../stores/app.svelte.js';
  import PublicServerList from './PublicServerList.svelte';
  import PrivateServerForm from './PrivateServerForm.svelte';
</script>

<div class="add-server-container">
  <div class="add-server-header">
    <h1 class="panel-title">Conectar a un servidor</h1>
    <p class="panel-subtitle">Explora la infraestructura descentralizada de <strong>ENTROPY</strong>. Selecciona un nodo geográfico para optimizar la latencia táctica.</p>
  </div>

  <div class="panel-card">
    <div class="panel-meta-corner">
      <span>SYS_VERSION: 2.0.4-LTS</span>
      <span>ENCRYPT: AES-256-GCM</span>
    </div>

    <!-- Tabs -->
    <div class="panel-tabs">
      <button
        class="panel-tab"
        class:panel-tab-active={app.addServerTab === 'public'}
        on:click={() => app.addServerTab = 'public'}
      >
        <span class="material-symbols-outlined" style="font-size:16px;font-variation-settings:'FILL' 1">public</span>
        Red Pública
      </button>
      <button
        class="panel-tab"
        class:panel-tab-active={app.addServerTab === 'private'}
        on:click={() => app.addServerTab = 'private'}
      >
        <span class="material-symbols-outlined" style="font-size:16px">lock</span>
        Servidor Privado
      </button>
    </div>

    {#if app.addServerTab === 'public'}
      <PublicServerList />
    {:else}
      <PrivateServerForm />
    {/if}
  </div>

  <!-- Stats grid — solo en pestaña pública -->
  {#if app.addServerTab === 'public'}
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">Carga Global</div>
        <div class="stat-value">74% <span class="stat-unit">UTILIZACIÓN</span></div>
        <div class="stat-bar"><div class="stat-bar-fill" style="width:74%"></div></div>
      </div>
      <div class="stat-card stat-card-wide">
        <div class="stat-label">Servidor más cercano</div>
        <div class="stat-server-name">SANTIAGO_CL_01</div>
        <span class="stat-subtitle">Recomendado para su ubicación actual</span>
        <button class="auto-connect-btn">AUTO-CONNECT</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .add-server-container {
    flex: 1; overflow-y: auto; padding: 32px; display: flex; flex-direction: column; gap: 32px;
  }
  .add-server-container::-webkit-scrollbar { width: 4px; }
  .add-server-container::-webkit-scrollbar-track { background: var(--surface); }
  .add-server-container::-webkit-scrollbar-thumb { background: var(--surface-highest); }

  .add-server-header { }
  .panel-title {
    font-family: var(--font-headline); font-size: 36px; font-weight: 900;
    letter-spacing: -0.04em; text-transform: uppercase; margin-bottom: 8px;
  }
  .panel-subtitle { font-size: 13px; color: var(--on-surface-variant); max-width: 560px; }
  .panel-subtitle :global(strong) { color: var(--primary); font-family: var(--font-label); }

  .panel-card {
    background: #12121a; border: 1px solid rgba(59,74,68,0.1);
    position: relative; overflow: hidden;
  }
  .panel-meta-corner {
    position: absolute; top: 12px; right: 16px;
    font-family: var(--font-label); font-size: 10px; color: rgba(70,241,197,0.3);
    display: flex; flex-direction: column; align-items: flex-end; gap: 4px;
    pointer-events: none;
  }

  .panel-tabs { display: flex; border-bottom: 1px solid rgba(59,74,68,0.1); }
  .panel-tab {
    padding: 20px 32px; background: none; border: none; cursor: pointer;
    display: flex; align-items: center; gap: 8px;
    font-family: var(--font-headline); font-size: 12px; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.15em;
    color: var(--on-surface-variant); transition: background 0.15s;
  }
  .panel-tab:hover { background: var(--surface-high); }
  .panel-tab-active { background: var(--primary-container); color: var(--on-primary); }

  .stats-grid { display: grid; grid-template-columns: 1fr 2fr; gap: 24px; }
  .stat-card {
    background: var(--surface-low); padding: 24px;
    display: flex; flex-direction: column; gap: 16px;
  }
  .stat-card-wide { flex-direction: row; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 16px; }
  .stat-label { font-family: var(--font-label); font-size: 11px; color: var(--primary); text-transform: uppercase; letter-spacing: 0.15em; }
  .stat-value { font-family: var(--font-headline); font-size: 36px; font-weight: 900; }
  .stat-unit { font-family: var(--font-label); font-size: 10px; color: var(--outline); vertical-align: super; }
  .stat-bar { width: 100%; height: 4px; background: var(--surface-highest); }
  .stat-bar-fill { height: 100%; background: var(--primary); }
  .stat-server-name { font-family: var(--font-headline); font-size: 22px; font-weight: 700; }
  .stat-subtitle { font-size: 12px; color: var(--on-surface-variant); }
  .auto-connect-btn {
    padding: 10px 24px; background: none; border: 1px solid rgba(70,241,197,0.3);
    color: var(--primary); font-family: var(--font-label); font-size: 12px;
    cursor: pointer; transition: background 0.15s; text-transform: uppercase;
  }
  .auto-connect-btn:hover { background: rgba(70,241,197,0.1); }
</style>
