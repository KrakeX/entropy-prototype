<script>
  // Botones de acción del modo competición
</script>

<div class="floating-bar">
  <button class="float-btn float-btn-danger">
    <span class="material-symbols-outlined">mic</span>
    <span>MUTE</span>
  </button>
  <div class="float-divider"></div>
  <button class="float-btn">
    <span class="material-symbols-outlined">hearing_disabled</span>
    <span>DEAFEN</span>
  </button>
  <div class="float-divider"></div>
  <button class="float-btn float-btn-disconnect">
    <span class="material-symbols-outlined">call_end</span>
    <span>DISCONNECT</span>
  </button>
</div>

<style>
  .floating-bar {
    position: absolute; bottom: 32px; left: 50%; transform: translateX(-50%);
    display: flex; align-items: center; gap: 16px;
    background: rgba(42,41,47,0.9); backdrop-filter: blur(12px);
    padding: 12px 24px; border: 1px solid rgba(255,180,171,0.2);
    box-shadow: 0 0 20px rgba(255,180,171,0.1);
  }
  .float-btn {
    display: flex; align-items: center; gap: 8px;
    background: none; border: none; cursor: pointer;
    font-family: var(--font-label); font-size: 11px; font-weight: 700;
    color: var(--on-surface); text-transform: uppercase; padding: 8px 16px;
    transition: background 0.15s;
  }
  .float-btn:hover { background: rgba(255,255,255,0.05); }
  .float-btn-danger { color: var(--error); }
  .float-btn-danger:hover { background: rgba(255,180,171,0.1); }
  .float-btn-disconnect { color: #ff5252; }
  .float-btn-disconnect:hover { background: rgba(255,82,82,0.1); }
  .float-divider { width: 1px; height: 24px; background: rgba(59,74,68,0.3); }
</style>
