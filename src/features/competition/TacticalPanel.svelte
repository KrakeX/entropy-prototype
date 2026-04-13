<script>
  const latencyBars = [20, 30, 25, 15, 35, 20, 40];
</script>

<div class="tactical-panel">
  <h3 class="tactical-title">MÉTRICAS TÁCTICAS</h3>
  <div class="tactical-metric">
    <div class="metric-header">
      <span class="metric-label">Latencia de Red</span>
      <span class="metric-value" style="color:var(--error)">14ms</span>
    </div>
    <div class="metric-bars">
      {#each latencyBars as h}
        <div class="metric-bar" style="height:{h}%"></div>
      {/each}
    </div>
  </div>
  <div class="tactical-metric">
    <div class="metric-header">
      <span class="metric-label">CPU LOAD (ENTROPY)</span>
      <span class="metric-value">2.4%</span>
    </div>
    <div class="cpu-bar">
      <div class="cpu-bar-fill" style="width:12%"></div>
    </div>
  </div>
  <div class="tactical-filtered">
    <span class="tactical-filtered-title">NOTIFICACIONES FILTRADAS</span>
    <div class="filtered-items">
      <div class="filtered-item">
        <span class="material-symbols-outlined" style="font-size:16px">chat_bubble</span>
        <div class="filtered-bar" style="width:128px"></div>
      </div>
      <div class="filtered-item">
        <span class="material-symbols-outlined" style="font-size:16px">notifications</span>
        <div class="filtered-bar" style="width:96px"></div>
      </div>
      <div class="filtered-item">
        <span class="material-symbols-outlined" style="font-size:16px">alternate_email</span>
        <div class="filtered-bar" style="width:160px"></div>
      </div>
    </div>
  </div>
  <div class="competition-note">
    En modo competición, todas las superposiciones y notificaciones externas han sido suspendidas para maximizar el rendimiento de la red y el enfoque táctico.
  </div>
</div>

<style>
  .tactical-panel {
    background: var(--surface-high); padding: 24px;
    border-left: 1px solid rgba(255,180,171,0.1); overflow-y: auto;
    display: flex; flex-direction: column; gap: 32px;
  }
  .tactical-title { font-family: var(--font-headline); font-size: 11px; font-weight: 700; color: var(--error); text-transform: uppercase; letter-spacing: 0.15em; }
  .tactical-metric { display: flex; flex-direction: column; gap: 8px; }
  .metric-header { display: flex; justify-content: space-between; align-items: flex-end; }
  .metric-label { font-family: var(--font-label); font-size: 10px; color: var(--on-surface-variant); text-transform: uppercase; }
  .metric-value { font-family: var(--font-label); font-size: 14px; font-weight: 700; }
  .metric-bars { display: flex; align-items: flex-end; gap: 3px; height: 48px; }
  .metric-bar { flex: 1; background: rgba(255,180,171,0.3); min-height: 4px; }
  .metric-bar:last-child { background: var(--error); }
  .cpu-bar { width: 100%; height: 4px; background: var(--surface-highest); }
  .cpu-bar-fill { height: 100%; background: var(--primary-container); }

  .tactical-filtered { display: flex; flex-direction: column; gap: 12px; padding-top: 24px; border-top: 1px solid rgba(59,74,68,0.1); }
  .tactical-filtered-title { font-family: var(--font-headline); font-size: 10px; font-weight: 700; color: var(--on-surface-variant); text-transform: uppercase; letter-spacing: 0.2em; }
  .filtered-items { display: flex; flex-direction: column; gap: 12px; opacity: 0.3; }
  .filtered-item { display: flex; align-items: center; gap: 12px; }
  .filtered-bar { height: 8px; background: var(--outline-variant); }

  .competition-note {
    margin-top: auto; padding: 16px;
    background: rgba(255,180,171,0.05); border: 1px solid rgba(255,180,171,0.2);
    font-family: var(--font-label); font-size: 10px; color: rgba(255,180,171,0.8);
    text-transform: uppercase; line-height: 1.6;
  }
</style>
