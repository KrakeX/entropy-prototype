<script>
  import './app.css';

  import { app } from './stores/app.svelte.js';

  import TopNav from './components/TopNav.svelte';
  import CompetitionBanner from './components/CompetitionBanner.svelte';

  // Welcome view
  import ServerListSidebar from './features/servers/ServerListSidebar.svelte';
  import WelcomePanel from './features/welcome/WelcomePanel.svelte';
  import AddServerPanel from './features/servers/AddServerPanel.svelte';

  // Channel view
  import ChannelSidebar from './features/channels/ChannelSidebar.svelte';
  import ChatView from './features/channels/ChatView.svelte';
  import VoiceGrid from './features/competition/VoiceGrid.svelte';
</script>

<!-- Top Navigation Bar -->
<TopNav />

<!-- Competition Mode Banner (fixed, debajo del topbar) -->
{#if app.competitionMode}
  <CompetitionBanner />
{/if}

<!-- Main App Shell -->
<div class="app-shell" class:has-banner={app.competitionMode}>

  <!-- ── WELCOME VIEW ── -->
  {#if app.currentView === 'welcome'}
    <ServerListSidebar />
    <main class="main-content">
      {#if app.addServerPanel}
        <AddServerPanel />
      {:else}
        <WelcomePanel />
      {/if}
    </main>

  <!-- ── CHANNEL VIEW ── -->
  {:else if app.currentView === 'channel'}
    <ChannelSidebar />
    <main class="main-content" class:main-competition={app.competitionMode}>
      {#if app.competitionMode}
        <VoiceGrid />
      {:else}
        <ChatView />
      {/if}
    </main>
  {/if}

</div>

<style>
  /* ── App Shell ────────────────────────────────────── */
  .app-shell {
    display: flex; margin-top: 64px; height: calc(100vh - 64px);
    overflow: hidden;
  }
  .has-banner { margin-top: 96px; height: calc(100vh - 96px); }

  /* ── Main Content ─────────────────────────────────── */
  .main-content {
    flex: 1; background: var(--surface); overflow: hidden;
    display: flex; flex-direction: column; position: relative;
  }
  .main-competition { background: var(--surface-lowest); }
</style>
