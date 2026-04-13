<script>
  import { app } from '../../stores/app.svelte.js';
</script>

<div class="welcome-panel">
  <div class="welcome-bg-icon">
    <span class="material-symbols-outlined" style="font-size:600px;font-variation-settings:'FILL' 1">deployed_code</span>
  </div>
  <div class="welcome-content">
    <div class="welcome-logo-wrap">
      <div class="welcome-logo-box">
        <span class="material-symbols-outlined" style="font-size:56px;color:var(--primary);font-variation-settings:'FILL' 1">deployed_code</span>
      </div>
      <h1 class="welcome-title">BIENVENIDO A <span style="color:var(--primary)">ENTROPY</span></h1>
      <p class="welcome-subtitle">Comunicación privada por diseño</p>
    </div>
    <div class="features-grid">
      <div class="feature-card">
        <span class="material-symbols-outlined feature-icon">shield_lock</span>
        <h4 class="feature-title">Privacy-First</h4>
        <p class="feature-desc">Cifrado de extremo a extremo gestionado por llaves asimétricas locales. Tus datos nunca tocan el disco sin protección.</p>
      </div>
      <div class="feature-card">
        <span class="material-symbols-outlined feature-icon">bar_chart_4_bars</span>
        <h4 class="feature-title">Baja Latencia</h4>
        <p class="feature-desc">Optimizado para routing de alto rendimiento. Protocolos de túnel personalizados para gaming y tiempo real.</p>
      </div>
      <div class="feature-card">
        <span class="material-symbols-outlined feature-icon">dns</span>
        <h4 class="feature-title">Self-Hosted</h4>
        <p class="feature-desc">Levanta tu propio nodo en segundos. Control total sobre tu infraestructura y visibilidad de usuarios.</p>
      </div>
      <div class="feature-card">
        <span class="material-symbols-outlined feature-icon">sensors</span>
        <h4 class="feature-title">Modo Competición</h4>
        <p class="feature-desc">Aislamiento de recursos para torneos. Monitoreo constante de integridad de conexión y anticheat visual.</p>
      </div>
    </div>
    <div class="welcome-ctas">
      <button class="cta-primary">Iniciar Sesión</button>
      <button class="cta-secondary" on:click={() => app.addServerPanel = true}>Explorar Nodos</button>
    </div>
  </div>
</div>

<style>
  .welcome-panel {
    flex: 1; display: flex; align-items: center; justify-content: center;
    padding: 32px; position: relative;
  }
  .welcome-bg-icon {
    position: absolute; inset: 0; display: flex; align-items: center; justify-content: center;
    pointer-events: none; opacity: 0.03; overflow: hidden;
  }
  .welcome-content { z-index: 1; text-align: center; max-width: 640px; width: 100%; }
  .welcome-logo-wrap { display: flex; flex-direction: column; align-items: center; margin-bottom: 48px; }
  .welcome-logo-box {
    width: 96px; height: 96px; background: rgba(70,241,197,0.1);
    border: 1px solid rgba(70,241,197,0.2);
    display: flex; align-items: center; justify-content: center; margin-bottom: 24px;
  }
  .welcome-title {
    font-family: var(--font-headline); font-size: 52px; font-weight: 900;
    letter-spacing: -0.04em; color: var(--on-surface); line-height: 1;
  }
  .welcome-subtitle {
    font-family: var(--font-body); font-size: 18px; font-weight: 300;
    color: var(--on-surface-variant); text-transform: uppercase; letter-spacing: 0.1em; margin-top: 8px;
  }

  .features-grid {
    display: grid; grid-template-columns: 1fr 1fr;
    gap: 1px; background: rgba(59,74,68,0.2);
    border: 1px solid rgba(59,74,68,0.2); margin-bottom: 48px;
  }
  .feature-card {
    background: var(--surface-low); padding: 32px;
    display: flex; flex-direction: column; align-items: flex-start; text-align: left;
    transition: background 0.15s;
  }
  .feature-card:hover { background: var(--surface-highest); }
  .feature-icon { font-size: 28px; color: var(--primary); margin-bottom: 16px; }
  .feature-title {
    font-family: var(--font-headline); font-size: 16px; font-weight: 700;
    color: var(--on-surface); text-transform: uppercase; letter-spacing: -0.01em; margin-bottom: 8px;
  }
  .feature-desc { font-size: 13px; color: var(--on-surface-variant); line-height: 1.6; }

  .welcome-ctas { display: flex; justify-content: center; gap: 24px; }
  .cta-primary {
    padding: 12px 32px; background: var(--primary-container); color: var(--on-primary);
    border: 1px solid var(--primary); font-family: var(--font-headline); font-weight: 700;
    font-size: 13px; text-transform: uppercase; letter-spacing: 0.15em;
    cursor: pointer; transition: all 0.15s;
  }
  .cta-primary:hover { background: var(--primary); }
  .cta-secondary {
    padding: 12px 32px; background: none; color: var(--on-surface);
    border: 1px solid var(--outline-variant); font-family: var(--font-headline); font-weight: 700;
    font-size: 13px; text-transform: uppercase; letter-spacing: 0.15em;
    cursor: pointer; transition: background 0.15s;
  }
  .cta-secondary:hover { background: var(--surface-highest); }
</style>
