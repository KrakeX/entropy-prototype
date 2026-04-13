<script>
  import { app } from '../../stores/app.svelte.js';

  let messageInput = $state('');
</script>

<div class="chat-input-area">
  <div class="chat-input-box">
    <div class="chat-input-inner">
      <button class="icon-btn"><span class="material-symbols-outlined">add_circle</span></button>
      <input
        class="chat-input"
        type="text"
        placeholder="Escribe un mensaje en #{app.selectedChannel?.name}..."
        bind:value={messageInput}
      />
      <div class="chat-input-actions">
        <button class="icon-btn"><span class="material-symbols-outlined">gif_box</span></button>
        <button class="icon-btn"><span class="material-symbols-outlined">sentiment_satisfied</span></button>
        <div class="chat-input-divider"></div>
        <button class="icon-btn send-btn"><span class="material-symbols-outlined" style="font-variation-settings:'FILL' 1">send</span></button>
      </div>
    </div>
    <div class="chat-input-progress"></div>
  </div>
  <div class="chat-input-meta">
    <div class="chat-enc-info">
      <span class="enc-label">Encryption: AES-256 GCM</span>
      <span class="enc-dot">●</span>
      <span class="enc-tunnel">Secure Tunnel Active</span>
    </div>
    <span class="enc-shortcut">Press <kbd>ESC</kbd> to exit focus</span>
  </div>
</div>

<style>
  .chat-input-area { padding: 0 24px 24px; flex-shrink: 0; }
  .chat-input-box { background: var(--surface-highest); }
  .chat-input-inner { display: flex; align-items: center; padding: 12px 16px; gap: 16px; }
  .chat-input { flex: 1; background: none; border: none; font-size: 14px; color: var(--on-surface); outline: none; }
  .chat-input::placeholder { color: rgba(186,202,194,0.4); }
  .chat-input-actions { display: flex; align-items: center; gap: 12px; color: var(--on-surface-variant); }
  .chat-input-divider { width: 1px; height: 24px; background: rgba(59,74,68,0.2); }
  .send-btn { color: var(--primary); }
  .chat-input-progress { height: 2px; background: var(--surface-low); position: relative; }
  .chat-input-progress::after { content: ''; position: absolute; left: 0; top: 0; width: 30%; height: 100%; background: var(--primary); }

  .chat-input-meta { display: flex; justify-content: space-between; align-items: center; padding: 8px 4px 0; }
  .chat-enc-info { display: flex; align-items: center; gap: 8px; }
  .enc-label { font-family: var(--font-label); font-size: 9px; color: var(--primary); text-transform: uppercase; }
  .enc-dot { font-size: 9px; color: rgba(186,202,194,0.4); }
  .enc-tunnel { font-family: var(--font-label); font-size: 9px; color: rgba(186,202,194,0.4); text-transform: uppercase; }
  .enc-shortcut { font-family: var(--font-label); font-size: 9px; color: rgba(186,202,194,0.4); text-transform: uppercase; }
  kbd { background: var(--surface-high); padding: 1px 4px; font-size: 8px; }
</style>
