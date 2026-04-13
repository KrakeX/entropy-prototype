<script>
  import { app } from '../../stores/app.svelte.js';
  import MessageList from './MessageList.svelte';
  import ChatInput from './ChatInput.svelte';
</script>

<!-- Channel Header -->
<div class="channel-header">
  <div class="channel-header-left">
    <span class="material-symbols-outlined" style="color:var(--on-surface-variant)">tag</span>
    <h2 class="channel-header-name">{app.selectedChannel?.name}</h2>
    <span class="channel-header-desc">Chat principal de la comunidad Latam</span>
  </div>
  <div class="channel-header-right">
    <button class="icon-btn"><span class="material-symbols-outlined">notifications</span></button>
    <button class="icon-btn"><span class="material-symbols-outlined">push_pin</span></button>
    <button class="icon-btn"><span class="material-symbols-outlined">group</span></button>
    <div class="search-wrap">
      <span class="material-symbols-outlined search-icon">search</span>
      <input class="search-input" type="text" placeholder="Buscar..." />
    </div>
  </div>
</div>

<MessageList />
<ChatInput />

<style>
  .channel-header {
    height: 48px; display: flex; align-items: center; justify-content: space-between;
    padding: 0 24px; background: var(--surface-low);
    border-bottom: 1px solid rgba(59,74,68,0.05); flex-shrink: 0;
  }
  .channel-header-left { display: flex; align-items: center; gap: 12px; }
  .channel-header-name {
    font-family: var(--font-headline); font-size: 14px; font-weight: 700; text-transform: uppercase; letter-spacing: -0.01em;
  }
  .channel-header-desc { font-size: 12px; color: var(--on-surface-variant); opacity: 0.4; }
  .channel-header-right { display: flex; align-items: center; gap: 8px; }
  .search-wrap { position: relative; }
  .search-icon { position: absolute; left: 8px; top: 50%; transform: translateY(-50%); font-size: 16px; color: var(--on-surface-variant); }
  .search-input {
    background: var(--surface-high); border: none; padding: 4px 8px 4px 28px;
    font-size: 12px; color: var(--on-surface); outline: none; width: 160px;
  }
  .search-input::placeholder { color: var(--on-surface-variant); opacity: 0.4; }
  .search-input:focus { box-shadow: 0 0 0 1px var(--primary); }
</style>
