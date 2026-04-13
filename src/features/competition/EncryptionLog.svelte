<script>
  const logLines = [
    { ts: '[14:22:01]', msg: 'HANDSHAKE_SUCCESS - NODE_772' },
    { ts: '[14:22:05]', msg: 'PACKET_BUFFER_CLEARED' },
    { ts: '[14:22:12]', msg: 'VOICE_ENCODING: OPUS_REDUCED_LATENCY' },
    { ts: '[14:22:25]', msg: 'PEER_SOVA_99 SPEAKING (SIG_STRENGTH: 0.98)' },
    { ts: '[14:22:28]', msg: 'SUPPRESSING_NON_CRITICAL_IO' },
  ];
</script>

<div class="enc-log">
  <div class="enc-log-title">Live_Encryption_Log</div>
  <div class="enc-log-lines">
    {#each logLines as line}
      <p><span class="log-ts">{line.ts}</span> {line.msg}</p>
    {/each}
    <p class="log-cursor">_</p>
  </div>
</div>

<style>
  .enc-log {
    background: var(--surface-lowest); padding: 16px 32px;
    font-family: var(--font-label); font-size: 10px; position: relative; flex-shrink: 0;
    max-height: 120px; overflow: hidden;
  }
  .enc-log-title { position: absolute; top: 8px; right: 16px; color: rgba(255,180,171,0.3); text-transform: uppercase; }
  .enc-log-lines { color: var(--on-surface-variant); display: flex; flex-direction: column; gap: 4px; }
  .log-ts { color: var(--error); }
  .log-cursor { animation: pulse 1s infinite; }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }
</style>
