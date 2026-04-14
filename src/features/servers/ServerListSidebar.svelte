<script>
  import { onMount } from 'svelte';
  import { app } from '../../stores/app.svelte.js';
  import { servers as mockServers } from '../../data/mock.js';
  import { invoke } from '../../utils/tauri.js';
  import ServerCard from './ServerCard.svelte';

  let servers = $state(mockServers);
  let loading = $state(false);
  let error = $state(null);

  onMount(async () => {
    loading = true;
    error = null;
    try {
      const result = await invoke('fetch_server_list', {}, null);
      if (result && result.length > 0) {
        // Mapear campos del backend al shape que usa ServerCard
        servers = result.map(s => ({
          id: s.id,
          name: s.name,
          icon: s.region.toUpperCase().slice(0, 2),
          type: s.server_type === 'Private' ? 'private' : 'public',
          verified: s.server_type === 'Public',
          members: s.player_count,
          ping: s.ping_ms,
          ip: s.ip,
          port_udp: s.port_udp,
          port_quic: s.port_quic,
        }));
      }
      // Si result es null (modo web) o vacío, se mantiene mockServers
    } catch (e) {
      error = e?.message ?? String(e);
    } finally {
      loading = false;
    }
  });
</script>

<aside class="sidebar">
  <div class="sidebar-section-label">Mis Servidores</div>
  <div class="server-list">
    {#if loading}
      <div class="list-status">
        <span class="material-symbols-outlined spin">sync</span>
        <span>CONECTANDO...</span>
      </div>
    {:else if error}
      <div class="list-status list-status-error">
        <span class="material-symbols-outlined">cloud_off</span>
        <span>SIN CONEXIÓN</span>
      </div>
    {/if}
    {#each servers as server}
      <ServerCard {server} isActive={app.currentServer?.id === server.id} />
    {/each}
    <button class="add-server-btn" on:click={() => app.addServerPanel = true}>
      <span class="material-symbols-outlined" style="font-size:16px">add</span>
      <span>AGREGAR SERVIDOR</span>
    </button>
  </div>
  <div class="sidebar-user">
    <div class="user-avatar-wrap">
      <div class="user-avatar">DU</div>
      <div class="user-online-dot"></div>
    </div>
    <div class="user-info">
      <span class="user-name">Durán</span>
      <span class="user-status">En línea | Cifrado</span>
    </div>
    <div class="user-controls">
      <button class="icon-btn"><span class="material-symbols-outlined">mic</span></button>
      <button class="icon-btn"><span class="material-symbols-outlined">volume_up</span></button>
      <button class="icon-btn"><span class="material-symbols-outlined">settings</span></button>
    </div>
  </div>
</aside>

<style>
  .sidebar {
    width: 280px; flex-shrink: 0; background: var(--surface-low);
    display: flex; flex-direction: column; overflow: hidden;
  }

  .sidebar-section-label {
    padding: 16px 24px 8px;
    font-family: var(--font-label); font-size: 11px;
    text-transform: uppercase; letter-spacing: 0.2em;
    color: var(--on-surface-variant); opacity: 0.7;
  }

  .server-list { flex: 1; overflow-y: auto; padding: 0 12px; display: flex; flex-direction: column; gap: 8px; }
  .server-list::-webkit-scrollbar { display: none; }

  .add-server-btn {
    width: 100%; margin-top: 8px; border: 1px dashed var(--outline-variant);
    background: none; padding: 12px; display: flex; align-items: center;
    justify-content: center; gap: 8px; color: var(--on-surface-variant);
    cursor: pointer; transition: background 0.15s;
    font-family: var(--font-label); font-size: 10px; text-transform: uppercase; letter-spacing: 0.15em;
  }
  .add-server-btn:hover { background: var(--surface-highest); }

  .list-status {
    display: flex; align-items: center; gap: 6px; padding: 10px 12px;
    font-family: var(--font-label); font-size: 10px; letter-spacing: 0.1em;
    color: var(--on-surface-variant); opacity: 0.5;
  }
  .list-status-error { color: var(--error); opacity: 0.7; }
  @keyframes spin { to { transform: rotate(360deg); } }
  .spin { display: inline-block; animation: spin 1s linear infinite; font-size: 16px; }

  .sidebar-user {
    margin-top: auto; padding: 12px 16px;
    display: flex; align-items: center; gap: 12px;
    background: rgba(42,41,47,0.5);
    border-top: 1px solid rgba(59,74,68,0.1);
  }
  .user-avatar-wrap { position: relative; }
  .user-avatar {
    width: 40px; height: 40px; display: flex; align-items: center;
    justify-content: center; font-weight: 700; font-size: 13px;
    background: var(--surface-highest); color: var(--on-surface);
    border: 1px solid rgba(70,241,197,0.2);
  }
  .user-online-dot {
    position: absolute; bottom: -1px; right: -1px;
    width: 12px; height: 12px; background: var(--primary);
    border: 2px solid var(--surface-low);
  }
  .user-info { flex: 1; }
  .user-name { display: block; font-size: 14px; font-weight: 700; font-family: var(--font-headline); color: var(--on-surface); }
  .user-status { display: block; font-family: var(--font-label); font-size: 10px; color: var(--primary); text-transform: uppercase; letter-spacing: -0.02em; }
  .user-controls { display: flex; gap: 2px; }
</style>
