<script>
  import { messages } from '../../data/mock.js';
</script>

<div class="messages-area">
  <div class="date-separator">
    <div class="date-line"></div>
    <span class="date-label">Hoy — 21 de Octubre</span>
    <div class="date-line"></div>
  </div>
  {#each messages as msg}
    <div class="message" class:message-highlight={msg.highlight}>
      <div class="message-avatar">{msg.initials}</div>
      <div class="message-body">
        <div class="message-header">
          <span
            class="message-username"
            style="color:{msg.userColor === 'primary' ? 'var(--primary)' : msg.userColor === 'secondary' ? 'var(--secondary)' : 'var(--on-surface)'}"
          >{msg.user}</span>
          <span class="message-time">{msg.time}</span>
        </div>
        <p class="message-text">{msg.text}</p>
      </div>
    </div>
  {/each}
</div>

<style>
  .messages-area { flex: 1; overflow-y: auto; padding: 24px; display: flex; flex-direction: column; gap: 32px; }
  .messages-area::-webkit-scrollbar { width: 4px; }
  .messages-area::-webkit-scrollbar-track { background: var(--surface); }
  .messages-area::-webkit-scrollbar-thumb { background: var(--surface-highest); }
  .messages-area::-webkit-scrollbar-thumb:hover { background: var(--primary); }

  .date-separator { display: flex; align-items: center; gap: 16px; padding: 16px 0; }
  .date-line { flex: 1; height: 1px; background: rgba(59,74,68,0.1); }
  .date-label { font-family: var(--font-label); font-size: 10px; color: rgba(186,202,194,0.4); letter-spacing: 0.3em; text-transform: uppercase; white-space: nowrap; }

  .message { display: flex; gap: 16px; }
  .message-highlight {
    background: rgba(70,241,197,0.05); margin: 0 -24px; padding: 8px 24px;
    border-left: 2px solid var(--primary);
  }
  .message-avatar {
    width: 40px; height: 40px; background: var(--surface-highest);
    display: flex; align-items: center; justify-content: center;
    font-size: 12px; font-weight: 700; color: var(--on-surface); flex-shrink: 0;
  }
  .message-body { display: flex; flex-direction: column; }
  .message-header { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; }
  .message-username { font-family: var(--font-headline); font-weight: 700; font-size: 14px; cursor: pointer; }
  .message-username:hover { text-decoration: underline; }
  .message-time { font-family: var(--font-label); font-size: 10px; color: rgba(186,202,194,0.4); text-transform: uppercase; }
  .message-text { font-size: 14px; color: var(--on-surface); line-height: 1.6; max-width: 640px; }
</style>
