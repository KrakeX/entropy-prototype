<script>
  let privateServerIP = $state('');
  let privateServerPort = $state('7700');
</script>

<div class="private-form">
  <p class="form-description">Conecta a un servidor self-hosted de Entropy. La conexión se establece directamente sin pasar por la red pública.</p>
  <div class="form-row">
    <div class="form-group" style="flex:3">
      <label class="form-label">Dirección del Nodo (IP/Host)</label>
      <div class="input-wrap">
        <input class="form-input" type="text" placeholder="192.168.1.50 o dominio.ejemplo.com" bind:value={privateServerIP} />
        <div class="input-underline"></div>
      </div>
    </div>
    <div class="form-group" style="flex:1">
      <label class="form-label">Puerto</label>
      <div class="input-wrap">
        <input class="form-input text-center" type="text" bind:value={privateServerPort} />
        <div class="input-underline"></div>
      </div>
    </div>
  </div>
  <div class="info-banner">
    <div class="info-banner-icon">
      <span class="material-symbols-outlined" style="font-variation-settings:'FILL' 1">shield</span>
    </div>
    <p class="info-banner-text">Conexión directa cifrada E2E — Sin intermediarios, sin telemetría, sin datos biométricos.</p>
  </div>
  <button class="connect-btn-full">CONECTAR AL SERVIDOR</button>
  <div class="form-footer">
    <div class="form-footer-badges">
      <div class="footer-badge"><div class="footer-dot"></div><span>RSA-4096 Active</span></div>
      <div class="footer-badge"><div class="footer-dot"></div><span>AES-256-GCM</span></div>
    </div>
    <span class="footer-version">V.2.4.0-STABLE</span>
  </div>
</div>

<style>
  .private-form { padding: 32px; }
  .form-description { font-size: 13px; color: var(--on-surface-variant); margin-bottom: 24px; max-width: 480px; }
  .form-row { display: flex; gap: 16px; margin-bottom: 24px; }
  .form-group { display: flex; flex-direction: column; gap: 8px; }
  .form-label {
    font-family: var(--font-label); font-size: 10px; text-transform: uppercase;
    letter-spacing: 0.15em; color: var(--on-surface-variant);
  }
  .input-wrap { position: relative; }
  .form-input {
    width: 100%; background: var(--surface-highest); border: none;
    padding: 16px; font-family: var(--font-label); color: var(--primary);
    font-size: 13px; outline: none;
  }
  .form-input::placeholder { color: var(--on-surface-variant); opacity: 0.3; }
  .form-input.text-center { text-align: center; }
  .input-underline { position: absolute; bottom: 0; left: 0; right: 0; height: 2px; background: var(--secondary-container); }

  .info-banner {
    display: flex; align-items: center; gap: 16px; padding: 16px;
    background: rgba(70,241,197,0.05); border: 1px solid rgba(70,241,197,0.1);
    margin-bottom: 24px;
  }
  .info-banner-icon {
    width: 40px; height: 40px; background: rgba(70,241,197,0.1);
    display: flex; align-items: center; justify-content: center; flex-shrink: 0;
    color: var(--primary);
  }
  .info-banner-text {
    font-family: var(--font-label); font-size: 11px; color: var(--primary-fixed);
    text-transform: uppercase; letter-spacing: 0.05em; line-height: 1.6;
  }
  .connect-btn-full {
    width: 100%; padding: 20px; background: var(--primary-container); color: var(--on-primary);
    border: none; cursor: pointer; font-family: var(--font-headline); font-size: 18px;
    font-weight: 900; letter-spacing: 0.15em; text-transform: uppercase;
    transition: all 0.15s;
  }
  .connect-btn-full:hover { background: var(--primary); }

  .form-footer {
    display: flex; justify-content: space-between; align-items: center;
    padding-top: 16px; margin-top: 24px;
    border-top: 1px solid rgba(59,74,68,0.1);
  }
  .form-footer-badges { display: flex; gap: 16px; }
  .footer-badge { display: flex; align-items: center; gap: 8px; font-family: var(--font-label); font-size: 10px; color: var(--on-surface-variant); text-transform: uppercase; }
  .footer-dot { width: 6px; height: 6px; background: var(--primary); }
  .footer-version { font-family: var(--font-label); font-size: 10px; color: rgba(186,202,194,0.4); }
</style>
