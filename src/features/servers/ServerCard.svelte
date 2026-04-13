<script>
  import { pingColor } from '../../utils/ping.js';
  import { enterServer } from '../../stores/app.svelte.js';

  let { server, isActive } = $props();
</script>

<button
  class="server-card"
  class:server-card-active={isActive}
  class:server-card-private={server.type === 'private'}
  on:click={() => enterServer(server)}
>
  <div class="server-icon" class:server-icon-private={server.type === 'private'}>
    {server.icon}
    {#if server.type === 'private'}
      <span class="server-icon-lock">
        <span class="material-symbols-outlined" style="font-size:10px">lock</span>
      </span>
    {/if}
  </div>
  <div class="server-info">
    <div class="server-info-row">
      <span class="server-card-name">{server.name}</span>
      {#if server.verified}
        <span class="material-symbols-outlined" style="font-size:14px;color:var(--primary);font-variation-settings:'FILL' 1">verified</span>
      {/if}
      {#if server.type === 'private'}
        <span class="badge-private-sm">PRIVATE</span>
      {/if}
    </div>
    <div class="server-meta">
      <span>{server.members.toLocaleString()} USERS</span>
      <span class="ping-dot" style="background:{pingColor(server.ping)}"></span>
      <span style="color:{pingColor(server.ping)}">{server.ping}MS</span>
      {#if server.ip}<span class="server-ip">{server.ip}</span>{/if}
    </div>
  </div>
</button>

<style>
  .server-card {
    display: flex; align-items: center; gap: 12px; padding: 12px;
    background: none; border: none; cursor: pointer; width: 100%;
    color: var(--on-surface-variant); opacity: 0.7;
    transition: all 0.15s; text-align: left;
  }
  .server-card:hover { background: var(--surface-high); opacity: 1; }
  .server-card-active {
    background: var(--surface-high); opacity: 1;
    border-left: 2px solid var(--primary-container);
  }

  .server-icon {
    width: 40px; height: 40px; background: var(--surface-highest);
    display: flex; align-items: center; justify-content: center;
    font-weight: 700; font-size: 13px; position: relative; flex-shrink: 0;
    color: var(--on-surface);
  }
  .server-icon-private { background: var(--secondary-container); color: var(--on-secondary-container); }
  .server-icon-lock {
    position: absolute; top: -4px; right: -4px;
    background: var(--surface-lowest); padding: 2px;
    color: var(--secondary);
  }

  .server-info { flex: 1; overflow: hidden; }
  .server-info-row { display: flex; align-items: center; gap: 4px; }
  .server-card-name { font-size: 14px; font-weight: 600; color: var(--on-surface); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .server-meta { display: flex; align-items: center; gap: 6px; font-family: var(--font-label); font-size: 10px; color: var(--on-surface-variant); opacity: 0.6; margin-top: 2px; }
  /* .ping-dot viene de app.css */
  .server-ip { opacity: 0.4; }
  .badge-private-sm {
    font-family: var(--font-label); font-size: 8px; padding: 1px 4px;
    background: rgba(79,49,156,0.2); color: var(--secondary);
    border: 1px solid rgba(206,189,255,0.2);
  }
</style>
