<script>
  // No props needed — siempre se renderiza cuando competitionMode es true
</script>

<div class="competition-banner">
  Modo Competición activo — Recursos reducidos | Notificaciones silenciadas | Solo voz activa
</div>

<style>
  .competition-banner {
    position: fixed; top: 64px; left: 0; right: 0; z-index: 40;
    height: 32px; display: flex; align-items: center; justify-content: center;
    background: linear-gradient(90deg, rgba(255,180,171,0.3), rgba(255,180,171,0.1), rgba(255,180,171,0.3));
    border-bottom: 1px solid rgba(255,180,171,0.4);
    font-family: var(--font-label); font-size: 10px;
    color: var(--error); text-transform: uppercase; letter-spacing: 0.2em; font-weight: 700;
  }
</style>
