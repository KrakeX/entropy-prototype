<script>
  import SpeakerCard from './SpeakerCard.svelte';
  import TacticalPanel from './TacticalPanel.svelte';
  import EncryptionLog from './EncryptionLog.svelte';
  import FloatingActionBar from './FloatingActionBar.svelte';
</script>

<!-- Voice Header -->
<div class="voice-header">
  <div class="voice-header-left">
    <span class="material-symbols-outlined" style="color:var(--error);font-variation-settings:'FILL' 1">graphic_eq</span>
    <h2 class="voice-title">Canal de Voz: Alfa-Zulu</h2>
    <div class="voice-divider"></div>
    <span class="voice-bitrate">BITRATE: 128KBPS (LIMITED)</span>
  </div>
  <div class="voice-header-right">
    <span class="voice-enc-badge">
      <span class="material-symbols-outlined" style="font-size:14px">lock</span>
      AES-256 ACTIVE
    </span>
  </div>
</div>

<div class="voice-grid-layout">
  <!-- Speaker Grid -->
  <div class="speaker-grid">
    <SpeakerCard name="SOVA_99" initials="SO" active={true} audioLevel={85} />
    <SpeakerCard name="VIPER_NET" initials="VI" muted={true} />
    <SpeakerCard name="JETT_DASH" initials="JE" muted={true} />
    <SpeakerCard name="DURÁN (YOU)" initials="DU" isYou={true} />
  </div>

  <!-- Tactical Metrics Panel -->
  <TacticalPanel />
</div>

<EncryptionLog />
<FloatingActionBar />

<style>
  .voice-header {
    height: 56px; display: flex; align-items: center; justify-content: space-between;
    padding: 0 32px; background: var(--surface-low);
    border-bottom: 1px solid rgba(255,180,171,0.1); flex-shrink: 0;
  }
  .voice-header-left { display: flex; align-items: center; gap: 16px; }
  .voice-title { font-family: var(--font-headline); font-size: 18px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; }
  .voice-divider { width: 1px; height: 16px; background: rgba(59,74,68,0.3); }
  .voice-bitrate { font-family: var(--font-label); font-size: 10px; color: var(--error); }
  .voice-enc-badge {
    display: flex; align-items: center; gap: 8px; padding: 4px 12px;
    background: var(--surface-high); font-family: var(--font-label); font-size: 10px;
  }

  .voice-grid-layout {
    flex: 1; display: grid; grid-template-columns: 8fr 4fr; gap: 1px;
    overflow: hidden; min-height: 0;
  }

  .speaker-grid {
    padding: 16px; display: grid; grid-template-columns: 1fr 1fr; gap: 4px;
    overflow: hidden;
  }
</style>
