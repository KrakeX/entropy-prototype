<script>
  import { publicServers } from '../../data/mock.js';
  import { pingColor } from '../../utils/ping.js';
  import { enterServer } from '../../stores/app.svelte.js';
</script>

<div class="server-table-header">
  <div style="flex:5">Identificador del Nodo</div>
  <div style="flex:2">Ubicación</div>
  <div style="flex:2;text-align:center">Usuarios</div>
  <div style="flex:1;text-align:right">Ping</div>
  <div style="flex:2"></div>
</div>
<div class="server-table-body">
  {#each publicServers as s}
    <div class="server-row">
      <div class="server-row-name" style="flex:5">
        <div class="ping-indicator" style="background:{pingColor(s.ping)}"></div>
        <div>
          <div class="server-row-title">{s.name}</div>
          <div class="server-row-id">SECURE_ID: {s.secureId}</div>
        </div>
      </div>
      <div class="server-row-region" style="flex:2">
        <span class="material-symbols-outlined" style="font-size:16px">location_on</span>
        {s.region}
      </div>
      <div class="server-row-users" style="flex:2">{s.members.toLocaleString()}</div>
      <div class="server-row-ping" style="flex:1;color:{pingColor(s.ping)}">{s.ping}ms</div>
      <div style="flex:2;padding-left:16px">
        <button class="connect-btn" on:click={() => enterServer({id: s.id, name: s.name, type: 'public', ping: s.ping})}>CONECTAR</button>
      </div>
    </div>
  {/each}
</div>
<div class="server-table-footer">
  <div class="legend">
    <div class="legend-item"><div class="legend-dot" style="background:var(--primary)"></div><span>ÓPTIMO</span></div>
    <div class="legend-item"><div class="legend-dot" style="background:var(--tertiary-container)"></div><span>ACEPTABLE</span></div>
    <div class="legend-item"><div class="legend-dot" style="background:var(--error)"></div><span>CRÍTICO</span></div>
  </div>
  <span class="protocol-tag">PROTOCOL_V4_READY</span>
</div>

<style>
  .server-table-header {
    display: flex; padding: 12px 32px;
    background: rgba(27,27,32,0.5);
    font-family: var(--font-label); font-size: 10px;
    color: var(--outline); text-transform: uppercase; letter-spacing: 0.1em;
  }
  .server-table-body { }
  .server-row {
    display: flex; align-items: center; padding: 20px 32px;
    border-bottom: 1px solid rgba(59,74,68,0.05);
    transition: background 0.15s;
  }
  .server-row:hover { background: rgba(42,41,47,0.3); }
  .server-row-name { display: flex; align-items: center; gap: 16px; }
  .ping-indicator { width: 8px; height: 8px; border-radius: 50%; flex-shrink: 0; }
  .server-row-title { font-weight: 700; font-family: var(--font-body); color: var(--on-surface); font-size: 14px; }
  .server-row-id { font-family: var(--font-label); font-size: 10px; color: var(--outline); margin-top: 2px; }
  .server-row-region { display: flex; align-items: center; gap: 4px; font-size: 12px; color: var(--on-surface-variant); }
  .server-row-users { text-align: center; font-family: var(--font-label); font-size: 12px; color: var(--on-surface); }
  .server-row-ping { text-align: right; font-family: var(--font-label); font-size: 12px; font-weight: 700; }

  .connect-btn {
    width: 100%; padding: 8px;
    background: var(--primary-container); color: var(--on-primary);
    border: none; cursor: pointer; font-family: var(--font-label);
    font-size: 10px; font-weight: 700; text-transform: uppercase;
    transition: filter 0.15s;
  }
  .connect-btn:hover { filter: brightness(1.1); }

  .server-table-footer {
    padding: 16px 32px; background: var(--surface-lowest);
    display: flex; justify-content: space-between; align-items: center;
    border-top: 1px solid rgba(59,74,68,0.1);
  }
  .legend { display: flex; gap: 24px; }
  .legend-item { display: flex; align-items: center; gap: 8px; font-family: var(--font-label); font-size: 10px; color: var(--outline); text-transform: uppercase; }
  .legend-dot { width: 8px; height: 8px; }
  .protocol-tag { font-family: var(--font-label); font-size: 10px; color: rgba(70,241,197,0.5); letter-spacing: 0.15em; }
</style>
