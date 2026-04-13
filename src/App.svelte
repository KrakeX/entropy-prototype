<script>
  let currentView = 'welcome'; // 'welcome' | 'channel'
  let addServerPanel = false;
  let addServerTab = 'public';
  let competitionMode = false;
  let selectedChannel = { id: 't1', name: 'general', type: 'text' };
  let currentServer = null;
  let privateServerIP = '';
  let privateServerPort = '7700';
  let messageInput = '';

  const servers = [
    { id: 1, name: 'Entropy Latam', icon: 'EL', type: 'public', members: 2847, ping: 18, verified: true },
    { id: 2, name: 'OW Ranked Chile', icon: 'OW', type: 'public', members: 412, ping: 12, verified: false },
    { id: 3, name: 'Team Flux', icon: 'TF', type: 'private', members: 14, ping: 2, ip: '192.168.1.50', verified: false },
  ];

  const publicServers = [
    { id: 10, name: 'Entropy NA East', secureId: 'VR-842-X', members: 8420, ping: 145, region: 'Virginia, US' },
    { id: 11, name: 'Entropy EU West', secureId: 'FR-121-Z', members: 12100, ping: 210, region: 'Frankfurt, DE' },
    { id: 12, name: 'FPS Latinoamérica', secureId: 'SP-189-Y', members: 1893, ping: 22, region: 'São Paulo, BR' },
    { id: 13, name: 'Overwatch LATAM Hub', secureId: 'ST-756-A', members: 756, ping: 15, region: 'Santiago, CL' },
    { id: 14, name: 'Valorant Chile', secureId: 'ST-534-B', members: 534, ping: 11, region: 'Santiago, CL' },
  ];

  const textChannels = [
    { id: 't1', name: 'general', unread: 3 },
    { id: 't2', name: 'estrategias', unread: 0 },
    { id: 't3', name: 'scrims', unread: 1 },
    { id: 't4', name: 'off-topic', unread: 0 },
  ];

  const voiceChannels = [
    { id: 'v1', name: 'Competitivo', users: [{ name: 'Durán', ping: '2ms' }, { name: 'Celis', ping: '4ms' }, { name: 'NightOwl', ping: '1ms' }] },
    { id: 'v2', name: 'Práctica', users: [] },
    { id: 'v3', name: 'Reunión Staff', users: [] },
  ];

  const messages = [
    { id: 1, user: 'NightOwl', time: '14:32', text: '¿Alguien para revisar los VODs del scrimmage de anoche? Creo que fallamos en la rotación de B en la ronda 12.', initials: 'NO', highlight: false, userColor: 'primary' },
    { id: 2, user: 'Phantom', time: '14:35', text: 'Esa rotación fue tardía porque Durán se quedó sin utilidad temprano. Tenemos que coordinar mejor los humos para el re-take.', initials: 'PH', highlight: false, userColor: 'on-surface' },
    { id: 3, user: 'Durán', time: '14:36', text: 'Mala mía. El bait de ellos en A me hizo gastar todo. Para la próxima espero el call de Celis antes de soltar el kit completo.', initials: 'DU', highlight: false, userColor: 'secondary' },
    { id: 4, user: 'Celis', time: '14:38', text: 'Entendido. Usemos el server de práctica en 15 min para ajustar eso. NightOwl, trae los clips marcados.', initials: 'CE', highlight: false, userColor: 'on-surface' },
    { id: 5, user: 'NightOwl', time: '14:40', text: 'Listo. Ya los subí a #estrategias. Nos vemos en el canal de voz en 10.', initials: 'NO', highlight: true, userColor: 'primary' },
  ];

  function pingColor(ping) {
    if (ping <= 20) return 'var(--primary)';
    if (ping <= 80) return 'var(--tertiary-container)';
    return 'var(--error)';
  }

  function pingStatus(ping) {
    if (ping <= 20) return 'optimal';
    if (ping <= 80) return 'acceptable';
    return 'critical';
  }

  function enterServer(server) {
    currentServer = server;
    currentView = 'channel';
    selectedChannel = textChannels[0];
    addServerPanel = false;
  }

  function goBack() {
    currentView = 'welcome';
    currentServer = null;
  }

  function toggleCompetition() {
    competitionMode = !competitionMode;
  }
</script>

<!-- =========================================================
     TOP NAV BAR
========================================================= -->
<nav class="topbar" class:competition-topbar={competitionMode}>
  <div class="topbar-left">
    {#if currentView === 'channel'}
      <button class="icon-btn" on:click={goBack}>
        <span class="material-symbols-outlined">arrow_back</span>
      </button>
    {/if}
    <div class="logo" class:logo-competition={competitionMode}>
      <span class="material-symbols-outlined logo-icon" style="font-variation-settings:'FILL' 1">
        {competitionMode ? 'security' : 'deployed_code'}
      </span>
      <span>ENTROPY</span>
    </div>
    {#if currentView === 'channel' && currentServer}
      <div class="topbar-divider"></div>
      <span class="server-name-header">{currentServer.name}</span>
      <span class="badge-public">{currentServer.type === 'private' ? 'Self-Hosted' : 'Red Pública'}</span>
      <span class="badge-e2e">
        <span class="material-symbols-outlined" style="font-size:12px;font-variation-settings:'FILL' 1">verified_user</span>
        E2E ACTIVE
      </span>
    {/if}
  </div>

  <div class="topbar-right">
    {#if competitionMode}
      <div class="competition-badge">
        <div class="competition-dot"></div>
        <span>MODO COMPETICIÓN</span>
      </div>
    {:else}
      <div class="e2e-badge">
        <span class="material-symbols-outlined" style="font-size:14px">shield</span>
        <span>E2E ENCRYPTED</span>
      </div>
    {/if}
    <div class="toggle-wrap" class:toggle-active={competitionMode} on:click={toggleCompetition}>
      <span class="toggle-label">{competitionMode ? 'MODO COMPETICIÓN' : 'Normal'}</span>
      <span class="material-symbols-outlined toggle-icon">{competitionMode ? 'toggle_on' : 'toggle_off'}</span>
    </div>
    <button class="icon-btn">
      <span class="material-symbols-outlined">settings</span>
    </button>
  </div>
</nav>

<!-- Competition Mode Banner -->
{#if competitionMode}
  <div class="competition-banner">
    Modo Competición activo — Recursos reducidos | Notificaciones silenciadas | Solo voz activa
  </div>
{/if}

<!-- =========================================================
     MAIN LAYOUT
========================================================= -->
<div class="app-shell" class:has-banner={competitionMode}>

  <!-- =====================================================
       SIDEBAR — WELCOME VIEW (Server List)
  ====================================================== -->
  {#if currentView === 'welcome'}
  <aside class="sidebar">
    <div class="sidebar-section-label">Mis Servidores</div>
    <div class="server-list">
      {#each servers as server}
        <button
          class="server-card"
          class:server-card-active={currentServer?.id === server.id}
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
      {/each}
      <button class="add-server-btn" on:click={() => addServerPanel = true}>
        <span class="material-symbols-outlined" style="font-size:16px">add</span>
        <span>AGREGAR SERVIDOR</span>
      </button>
    </div>
    <div class="sidebar-user">
      <div class="user-avatar-wrap">
        <div class="user-avatar">DU</div>
        <div class="user-online-dot"></div>
      </div>
      <div class="user-info">
        <span class="user-name">Durán</span>
        <span class="user-status">En línea | Cifrado</span>
      </div>
      <div class="user-controls">
        <button class="icon-btn"><span class="material-symbols-outlined">mic</span></button>
        <button class="icon-btn"><span class="material-symbols-outlined">volume_up</span></button>
        <button class="icon-btn"><span class="material-symbols-outlined">settings</span></button>
      </div>
    </div>
  </aside>

  <!-- MAIN — WELCOME VIEW -->
  <main class="main-content">
    {#if addServerPanel}
      <!-- ADD SERVER PANEL -->
      <div class="add-server-container">
        <div class="add-server-header">
          <h1 class="panel-title">Conectar a un servidor</h1>
          <p class="panel-subtitle">Explora la infraestructura descentralizada de <strong>ENTROPY</strong>. Selecciona un nodo geográfico para optimizar la latencia táctica.</p>
        </div>

        <div class="panel-card">
          <div class="panel-meta-corner">
            <span>SYS_VERSION: 2.0.4-LTS</span>
            <span>ENCRYPT: AES-256-GCM</span>
          </div>

          <!-- Tabs -->
          <div class="panel-tabs">
            <button
              class="panel-tab"
              class:panel-tab-active={addServerTab === 'public'}
              on:click={() => addServerTab = 'public'}
            >
              <span class="material-symbols-outlined" style="font-size:16px;font-variation-settings:'FILL' 1">public</span>
              Red Pública
            </button>
            <button
              class="panel-tab"
              class:panel-tab-active={addServerTab === 'private'}
              on:click={() => addServerTab = 'private'}
            >
              <span class="material-symbols-outlined" style="font-size:16px">lock</span>
              Servidor Privado
            </button>
          </div>

          {#if addServerTab === 'public'}
            <!-- PUBLIC SERVER LIST -->
            <div class="server-table-header">
              <div style="flex:5">Identificador del Nodo</div>
              <div style="flex:2">Ubicación</div>
              <div style="flex:2;text-align:center">Usuarios</div>
              <div style="flex:1;text-align:right">Ping</div>
              <div style="flex:2"></div>
            </div>
            <div class="server-table-body">
              {#each publicServers as s}
                <div class="server-row">
                  <div class="server-row-name" style="flex:5">
                    <div class="ping-indicator" style="background:{pingColor(s.ping)}"></div>
                    <div>
                      <div class="server-row-title">{s.name}</div>
                      <div class="server-row-id">SECURE_ID: {s.secureId}</div>
                    </div>
                  </div>
                  <div class="server-row-region" style="flex:2">
                    <span class="material-symbols-outlined" style="font-size:16px">location_on</span>
                    {s.region}
                  </div>
                  <div class="server-row-users" style="flex:2">{s.members.toLocaleString()}</div>
                  <div class="server-row-ping" style="flex:1;color:{pingColor(s.ping)}">{s.ping}ms</div>
                  <div style="flex:2;padding-left:16px">
                    <button class="connect-btn" on:click={() => enterServer({id: s.id, name: s.name, type: 'public', ping: s.ping})}>CONECTAR</button>
                  </div>
                </div>
              {/each}
            </div>
            <div class="server-table-footer">
              <div class="legend">
                <div class="legend-item"><div class="legend-dot" style="background:var(--primary)"></div><span>ÓPTIMO</span></div>
                <div class="legend-item"><div class="legend-dot" style="background:var(--tertiary-container)"></div><span>ACEPTABLE</span></div>
                <div class="legend-item"><div class="legend-dot" style="background:var(--error)"></div><span>CRÍTICO</span></div>
              </div>
              <span class="protocol-tag">PROTOCOL_V4_READY</span>
            </div>
          {:else}
            <!-- PRIVATE SERVER FORM -->
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
          {/if}
        </div>

        <!-- Stats grid -->
        {#if addServerTab === 'public'}
          <div class="stats-grid">
            <div class="stat-card">
              <div class="stat-label">Carga Global</div>
              <div class="stat-value">74% <span class="stat-unit">UTILIZACIÓN</span></div>
              <div class="stat-bar"><div class="stat-bar-fill" style="width:74%"></div></div>
            </div>
            <div class="stat-card stat-card-wide">
              <div class="stat-label">Servidor más cercano</div>
              <div class="stat-server-name">SANTIAGO_CL_01</div>
              <span class="stat-subtitle">Recomendado para su ubicación actual</span>
              <button class="auto-connect-btn">AUTO-CONNECT</button>
            </div>
          </div>
        {/if}
      </div>
    {:else}
      <!-- WELCOME PANEL -->
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
            <button class="cta-secondary" on:click={() => addServerPanel = true}>Explorar Nodos</button>
          </div>
        </div>
      </div>
    {/if}
  </main>

  <!-- =====================================================
       CHANNEL VIEW
  ====================================================== -->
  {:else if currentView === 'channel'}
  <aside class="sidebar" class:sidebar-competition={competitionMode}>
    <!-- Channels -->
    <div class="channel-list">
      <div class="channel-section">
        <div class="channel-section-label">Canales de Texto</div>
        {#each textChannels as ch}
          <button
            class="channel-item"
            class:channel-item-active={selectedChannel?.id === ch.id}
            class:channel-item-competition={competitionMode}
            on:click={() => selectedChannel = ch}
          >
            <div class="channel-item-left">
              <span class="material-symbols-outlined" style="font-size:16px">tag</span>
              <span class="channel-name">{ch.name}</span>
            </div>
            {#if ch.unread > 0 && !competitionMode}
              <span class="unread-badge">{ch.unread}</span>
            {/if}
          </button>
        {/each}
      </div>

      <div class="channel-section">
        <div class="channel-section-label">Canales de Voz</div>
        {#each voiceChannels as vc}
          <div class="voice-channel">
            <button class="channel-item voice-channel-btn">
              <div class="channel-item-left">
                <span class="material-symbols-outlined" style="font-size:16px">volume_up</span>
                <span class="channel-name">{vc.name}</span>
              </div>
              {#if vc.users.length > 0}
                <span class="material-symbols-outlined voice-add-icon">person_add</span>
              {/if}
            </button>
            {#if vc.users.length > 0}
              <div class="voice-users">
                {#each vc.users as u}
                  <div class="voice-user">
                    <div class="voice-user-left">
                      <div class="voice-user-avatar-wrap">
                        <div class="voice-user-avatar">{u.name.substring(0,2)}</div>
                        <div class="voice-user-dot"></div>
                      </div>
                      <span class="voice-user-name">{u.name}</span>
                    </div>
                    <span class="voice-user-ping">{u.ping}</span>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>

    <!-- Sidebar Footer -->
    <div class="sidebar-footer" class:sidebar-footer-competition={competitionMode}>
      <div class="sidebar-footer-encryption">
        <div class="footer-enc-icon">
          <span class="material-symbols-outlined" style="font-size:16px;color:var(--primary)">lock</span>
        </div>
        <div>
          <div class="footer-enc-label">CIFRADO ACTIVO</div>
          <div class="footer-enc-sub">Prot: E2E-ENTROPY</div>
        </div>
      </div>
      <div class="sidebar-footer-stats">
        <div>
          <div class="footer-stat-label">Latencia</div>
          <div class="footer-stat-value" style="color:var(--primary)">{currentServer?.ping}ms Stable</div>
        </div>
        <div>
          <div class="footer-stat-label">Network</div>
          <div class="footer-stat-value">{currentServer?.type === 'private' ? 'Self-Hosted' : 'Red Pública'}</div>
        </div>
      </div>
      <button class="add-server-btn-sm" on:click={() => { currentView = 'welcome'; addServerPanel = true; }}>
        NUEVO SERVIDOR
      </button>
      <div class="user-controls-sm">
        <button class="icon-btn" class:icon-btn-competition={competitionMode}><span class="material-symbols-outlined">mic</span></button>
        <button class="icon-btn" class:icon-btn-competition={competitionMode}><span class="material-symbols-outlined">volume_up</span></button>
        <button class="icon-btn" class:icon-btn-competition={competitionMode}><span class="material-symbols-outlined">settings</span></button>
      </div>
    </div>
  </aside>

  <!-- CHANNEL MAIN CONTENT -->
  <main class="main-content" class:main-competition={competitionMode}>
    {#if competitionMode}
      <!-- COMPETITION MODE — Voice Grid -->
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
          <div class="speaker-card speaker-active">
            <div class="speaker-mic-icon">
              <span class="material-symbols-outlined" style="color:var(--error)">mic</span>
            </div>
            <div class="speaker-avatar">SO</div>
            <span class="speaker-name" style="color:var(--error)">SOVA_99</span>
            <div class="speaker-bar"><div class="speaker-bar-fill" style="width:85%"></div></div>
          </div>
          <div class="speaker-card">
            <div class="speaker-avatar speaker-avatar-muted">VI</div>
            <span class="speaker-name muted">VIPER_NET</span>
            <div class="speaker-bar speaker-bar-empty"></div>
          </div>
          <div class="speaker-card">
            <div class="speaker-avatar speaker-avatar-muted">JE</div>
            <span class="speaker-name muted">JETT_DASH</span>
            <div class="speaker-bar speaker-bar-empty"></div>
          </div>
          <div class="speaker-card speaker-you">
            <div class="speaker-avatar">DU</div>
            <span class="speaker-name">DURÁN (YOU)</span>
            <div class="speaker-controls">
              <span class="material-symbols-outlined" style="font-size:16px">mic_off</span>
              <span class="material-symbols-outlined" style="font-size:16px">volume_up</span>
            </div>
          </div>
        </div>

        <!-- Tactical Metrics Panel -->
        <div class="tactical-panel">
          <h3 class="tactical-title">MÉTRICAS TÁCTICAS</h3>
          <div class="tactical-metric">
            <div class="metric-header">
              <span class="metric-label">Latencia de Red</span>
              <span class="metric-value" style="color:var(--error)">14ms</span>
            </div>
            <div class="metric-bars">
              {#each [20, 30, 25, 15, 35, 20, 40] as h}
                <div class="metric-bar" style="height:{h}%"></div>
              {/each}
            </div>
          </div>
          <div class="tactical-metric">
            <div class="metric-header">
              <span class="metric-label">CPU LOAD (ENTROPY)</span>
              <span class="metric-value">2.4%</span>
            </div>
            <div class="cpu-bar">
              <div class="cpu-bar-fill" style="width:12%"></div>
            </div>
          </div>
          <div class="tactical-filtered">
            <span class="tactical-filtered-title">NOTIFICACIONES FILTRADAS</span>
            <div class="filtered-items">
              <div class="filtered-item">
                <span class="material-symbols-outlined" style="font-size:16px">chat_bubble</span>
                <div class="filtered-bar" style="width:128px"></div>
              </div>
              <div class="filtered-item">
                <span class="material-symbols-outlined" style="font-size:16px">notifications</span>
                <div class="filtered-bar" style="width:96px"></div>
              </div>
              <div class="filtered-item">
                <span class="material-symbols-outlined" style="font-size:16px">alternate_email</span>
                <div class="filtered-bar" style="width:160px"></div>
              </div>
            </div>
          </div>
          <div class="competition-note">
            En modo competición, todas las superposiciones y notificaciones externas han sido suspendidas para maximizar el rendimiento de la red y el enfoque táctico.
          </div>
        </div>
      </div>

      <!-- Encryption Log -->
      <div class="enc-log">
        <div class="enc-log-title">Live_Encryption_Log</div>
        <div class="enc-log-lines">
          <p><span class="log-ts">[14:22:01]</span> HANDSHAKE_SUCCESS - NODE_772</p>
          <p><span class="log-ts">[14:22:05]</span> PACKET_BUFFER_CLEARED</p>
          <p><span class="log-ts">[14:22:12]</span> VOICE_ENCODING: OPUS_REDUCED_LATENCY</p>
          <p><span class="log-ts">[14:22:25]</span> PEER_SOVA_99 SPEAKING (SIG_STRENGTH: 0.98)</p>
          <p><span class="log-ts">[14:22:28]</span> SUPPRESSING_NON_CRITICAL_IO</p>
          <p class="log-cursor">_</p>
        </div>
      </div>

      <!-- Floating Action Bar -->
      <div class="floating-bar">
        <button class="float-btn float-btn-danger">
          <span class="material-symbols-outlined">mic</span>
          <span>MUTE</span>
        </button>
        <div class="float-divider"></div>
        <button class="float-btn">
          <span class="material-symbols-outlined">hearing_disabled</span>
          <span>DEAFEN</span>
        </button>
        <div class="float-divider"></div>
        <button class="float-btn float-btn-disconnect">
          <span class="material-symbols-outlined">call_end</span>
          <span>DISCONNECT</span>
        </button>
      </div>

    {:else}
      <!-- NORMAL MODE — Chat -->
      <div class="channel-header">
        <div class="channel-header-left">
          <span class="material-symbols-outlined" style="color:var(--on-surface-variant)">tag</span>
          <h2 class="channel-header-name">{selectedChannel?.name}</h2>
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

      <div class="chat-input-area">
        <div class="chat-input-box">
          <div class="chat-input-inner">
            <button class="icon-btn"><span class="material-symbols-outlined">add_circle</span></button>
            <input
              class="chat-input"
              type="text"
              placeholder="Escribe un mensaje en #{selectedChannel?.name}..."
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
    {/if}
  </main>
  {/if}
</div>

<style>
  /* ── Design Tokens ────────────────────────────────── */
  :global(*), :global(*::before), :global(*::after) { box-sizing: border-box; margin: 0; padding: 0; }
  :global(body) {
    background: #131318;
    color: #e4e1e9;
    font-family: 'Manrope', sans-serif;
    overflow: hidden;
    height: 100vh;
  }
  :global(#app) { height: 100vh; display: flex; flex-direction: column; }

  :global(:root) {
    --surface:                  #131318;
    --surface-lowest:           #0e0e13;
    --surface-low:              #1b1b20;
    --surface-container:        #1f1f25;
    --surface-high:             #2a292f;
    --surface-highest:          #35343a;
    --on-surface:               #e4e1e9;
    --on-surface-variant:       #bacac2;
    --primary:                  #46f1c5;
    --primary-container:        #00d4aa;
    --on-primary:               #00382b;
    --secondary:                #cebdff;
    --secondary-container:      #4f319c;
    --on-secondary-container:   #bea8ff;
    --tertiary-container:       #ffa39c;
    --error:                    #ffb4ab;
    --error-container:          #93000a;
    --outline-variant:          #3b4a44;
    --outline:                  #85948d;
    --font-headline: 'Space Grotesk', sans-serif;
    --font-body: 'Manrope', sans-serif;
    --font-label: 'JetBrains Mono', monospace;
  }

  /* ── Top Nav ──────────────────────────────────────── */
  .topbar {
    position: fixed; top: 0; left: 0; right: 0; z-index: 50;
    height: 64px; display: flex; align-items: center;
    justify-content: space-between; padding: 0 24px;
    background: var(--surface);
  }
  .competition-topbar { border-bottom: 1px solid rgba(255,180,171,0.2); }

  .topbar-left { display: flex; align-items: center; gap: 12px; }
  .topbar-right { display: flex; align-items: center; gap: 12px; }

  .logo {
    display: flex; align-items: center; gap: 8px;
    font-family: var(--font-label); font-size: 18px;
    font-weight: 900; color: var(--primary-container);
    text-transform: uppercase; letter-spacing: -0.04em;
  }
  .logo-competition { color: var(--error); }
  .logo-icon { font-size: 28px; }

  .topbar-divider { width: 1px; height: 24px; background: var(--outline-variant); opacity: 0.3; }
  .server-name-header { font-family: var(--font-headline); font-size: 16px; font-weight: 700; }

  .badge-public {
    font-family: var(--font-label); font-size: 9px;
    padding: 2px 6px; border: 1px solid rgba(0,212,170,0.4);
    color: var(--primary-container); text-transform: uppercase; letter-spacing: 0.05em;
  }
  .badge-e2e {
    display: flex; align-items: center; gap: 4px;
    font-family: var(--font-label); font-size: 9px;
    padding: 2px 8px; background: rgba(70,241,197,0.1);
    color: var(--primary); text-transform: uppercase;
  }

  .e2e-badge {
    display: flex; align-items: center; gap: 6px;
    padding: 4px 12px; border: 1px solid rgba(70,241,197,0.3);
    background: rgba(70,241,197,0.05); color: var(--primary);
    font-family: var(--font-label); font-size: 10px; letter-spacing: 0.15em;
  }
  .competition-badge {
    display: flex; align-items: center; gap: 8px;
    padding: 4px 12px; background: rgba(147,0,10,0.2);
    border: 1px solid rgba(255,180,171,0.3);
    font-family: var(--font-label); font-size: 10px;
    color: var(--error); font-weight: 700; text-transform: uppercase;
  }
  .competition-dot {
    width: 8px; height: 8px; border-radius: 50%;
    background: var(--error); animation: pulse 1.5s infinite;
  }

  .toggle-wrap {
    display: flex; align-items: center; gap: 6px;
    padding: 4px 12px; background: var(--surface-high);
    cursor: pointer; transition: background 0.15s;
    font-family: var(--font-label); font-size: 10px;
    color: var(--on-surface-variant); text-transform: uppercase; letter-spacing: 0.1em;
  }
  .toggle-wrap:hover { background: var(--surface-highest); }
  .toggle-active { background: rgba(255,180,171,0.1); color: var(--error); }
  .toggle-icon { font-size: 26px; color: var(--primary); }
  .toggle-active .toggle-icon { color: var(--error); }

  .icon-btn {
    padding: 8px; background: none; border: none; cursor: pointer;
    color: var(--on-surface-variant); transition: background 0.15s;
  }
  .icon-btn:hover { background: var(--surface-highest); }
  .icon-btn-competition:hover { color: var(--error); }

  /* ── Competition Banner ───────────────────────────── */
  .competition-banner {
    position: fixed; top: 64px; left: 0; right: 0; z-index: 40;
    height: 32px; display: flex; align-items: center; justify-content: center;
    background: linear-gradient(90deg, rgba(255,180,171,0.3), rgba(255,180,171,0.1), rgba(255,180,171,0.3));
    border-bottom: 1px solid rgba(255,180,171,0.4);
    font-family: var(--font-label); font-size: 10px;
    color: var(--error); text-transform: uppercase; letter-spacing: 0.2em; font-weight: 700;
  }

  /* ── App Shell ────────────────────────────────────── */
  .app-shell {
    display: flex; margin-top: 64px; height: calc(100vh - 64px);
    overflow: hidden;
  }
  .has-banner { margin-top: 96px; height: calc(100vh - 96px); }

  /* ── Sidebar ──────────────────────────────────────── */
  .sidebar {
    width: 280px; flex-shrink: 0; background: var(--surface-low);
    display: flex; flex-direction: column; overflow: hidden;
  }
  .sidebar-competition { /* nothing extra needed, colors cascade */ }

  .sidebar-section-label {
    padding: 16px 24px 8px;
    font-family: var(--font-label); font-size: 11px;
    text-transform: uppercase; letter-spacing: 0.2em;
    color: var(--on-surface-variant); opacity: 0.7;
  }

  .server-list { flex: 1; overflow-y: auto; padding: 0 12px; display: flex; flex-direction: column; gap: 8px; }
  .server-list::-webkit-scrollbar { display: none; }

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
  .ping-dot { width: 6px; height: 6px; border-radius: 50%; }
  .server-ip { opacity: 0.4; }
  .badge-private-sm {
    font-family: var(--font-label); font-size: 8px; padding: 1px 4px;
    background: rgba(79,49,156,0.2); color: var(--secondary);
    border: 1px solid rgba(206,189,255,0.2);
  }

  .add-server-btn {
    width: 100%; margin-top: 8px; border: 1px dashed var(--outline-variant);
    background: none; padding: 12px; display: flex; align-items: center;
    justify-content: center; gap: 8px; color: var(--on-surface-variant);
    cursor: pointer; transition: background 0.15s;
    font-family: var(--font-label); font-size: 10px; text-transform: uppercase; letter-spacing: 0.15em;
  }
  .add-server-btn:hover { background: var(--surface-highest); }

  .sidebar-user {
    margin-top: auto; padding: 12px 16px;
    display: flex; align-items: center; gap: 12px;
    background: rgba(42,41,47,0.5);
    border-top: 1px solid rgba(59,74,68,0.1);
  }
  .user-avatar-wrap { position: relative; }
  .user-avatar {
    width: 40px; height: 40px; display: flex; align-items: center;
    justify-content: center; font-weight: 700; font-size: 13px;
    background: var(--surface-highest); color: var(--on-surface);
    border: 1px solid rgba(70,241,197,0.2);
  }
  .user-online-dot {
    position: absolute; bottom: -1px; right: -1px;
    width: 12px; height: 12px; background: var(--primary);
    border: 2px solid var(--surface-low);
  }
  .user-info { flex: 1; }
  .user-name { display: block; font-size: 14px; font-weight: 700; font-family: var(--font-headline); color: var(--on-surface); }
  .user-status { display: block; font-family: var(--font-label); font-size: 10px; color: var(--primary); text-transform: uppercase; letter-spacing: -0.02em; }
  .user-controls { display: flex; gap: 2px; }

  /* ── Channel Sidebar ──────────────────────────────── */
  .channel-list { flex: 1; overflow-y: auto; padding: 24px 0; display: flex; flex-direction: column; gap: 32px; }
  .channel-list::-webkit-scrollbar { display: none; }
  .channel-section { padding: 0 16px; }
  .channel-section-label {
    padding: 0 8px 12px;
    font-family: var(--font-label); font-size: 10px; font-weight: 700;
    color: var(--on-surface-variant); text-transform: uppercase;
    letter-spacing: 0.2em; opacity: 0.6;
  }
  .channel-item {
    width: 100%; display: flex; align-items: center; justify-content: space-between;
    padding: 8px 12px; background: none; border: none; cursor: pointer;
    color: var(--on-surface-variant); opacity: 0.7; transition: all 0.15s;
    text-align: left;
  }
  .channel-item:hover { background: var(--surface-high); opacity: 1; }
  .channel-item-active { background: var(--surface-high); color: var(--primary-container); border-left: 2px solid var(--primary-container); opacity: 1; font-weight: 700; }
  .channel-item-competition.channel-item-active { color: var(--error); border-left-color: var(--error); }
  .channel-item-left { display: flex; align-items: center; gap: 12px; }
  .channel-name { font-size: 14px; font-family: var(--font-body); }
  .unread-badge {
    background: var(--primary); color: var(--on-primary);
    font-family: var(--font-label); font-size: 10px; font-weight: 700;
    padding: 1px 6px;
  }

  .voice-channel { }
  .voice-channel-btn { }
  .voice-add-icon { font-size: 16px; opacity: 0; }
  .voice-channel-btn:hover .voice-add-icon { opacity: 1; }
  .voice-users { margin-left: 36px; margin-top: 4px; display: flex; flex-direction: column; gap: 8px; }
  .voice-user { display: flex; align-items: center; justify-content: space-between; padding-right: 8px; }
  .voice-user-left { display: flex; align-items: center; gap: 8px; }
  .voice-user-avatar-wrap { position: relative; }
  .voice-user-avatar {
    width: 24px; height: 24px; background: var(--surface-highest);
    display: flex; align-items: center; justify-content: center;
    font-size: 9px; font-weight: 700; color: var(--on-surface);
  }
  .voice-user-dot {
    position: absolute; bottom: -2px; right: -2px;
    width: 8px; height: 8px; background: var(--primary);
    border: 1.5px solid var(--surface-low); border-radius: 50%;
    animation: pulse 2s infinite;
  }
  .voice-user-name { font-size: 12px; color: var(--on-surface); }
  .voice-user-ping { font-family: var(--font-label); font-size: 9px; color: var(--primary); }

  .sidebar-footer { padding: 16px; border-top: 1px solid rgba(59,74,68,0.1); }
  .sidebar-footer-encryption { display: flex; align-items: center; gap: 8px; margin-bottom: 12px; }
  .footer-enc-icon {
    width: 32px; height: 32px; background: var(--surface-highest);
    display: flex; align-items: center; justify-content: center;
  }
  .footer-enc-label { font-family: var(--font-label); font-size: 10px; color: var(--primary); }
  .footer-enc-sub { font-family: var(--font-label); font-size: 9px; color: var(--on-surface-variant); opacity: 0.6; text-transform: uppercase; }
  .sidebar-footer-stats { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 12px; }
  .footer-stat-label { font-family: var(--font-label); font-size: 8px; color: var(--on-surface-variant); opacity: 0.5; text-transform: uppercase; }
  .footer-stat-value { font-family: var(--font-label); font-size: 11px; color: var(--on-surface); }

  .add-server-btn-sm {
    width: 100%; padding: 10px;
    background: var(--primary-container); color: var(--on-primary);
    border: none; cursor: pointer; font-family: var(--font-label);
    font-size: 11px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.1em;
    transition: filter 0.15s; margin-bottom: 12px;
  }
  .add-server-btn-sm:hover { filter: brightness(1.1); }
  .sidebar-footer-competition .add-server-btn-sm { background: var(--error); color: var(--error-container); }
  .user-controls-sm { display: flex; justify-content: space-around; padding-top: 12px; border-top: 1px solid rgba(59,74,68,0.1); }

  /* ── Main Content ─────────────────────────────────── */
  .main-content {
    flex: 1; background: var(--surface); overflow: hidden;
    display: flex; flex-direction: column; position: relative;
  }
  .main-competition { background: var(--surface-lowest); }

  /* ── Welcome Panel ────────────────────────────────── */
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

  /* ── Add Server Panel ─────────────────────────────── */
  .add-server-container {
    flex: 1; overflow-y: auto; padding: 32px; display: flex; flex-direction: column; gap: 32px;
  }
  .add-server-container::-webkit-scrollbar { width: 4px; }
  .add-server-container::-webkit-scrollbar-track { background: var(--surface); }
  .add-server-container::-webkit-scrollbar-thumb { background: var(--surface-highest); }

  .add-server-header { }
  .panel-title {
    font-family: var(--font-headline); font-size: 36px; font-weight: 900;
    letter-spacing: -0.04em; text-transform: uppercase; margin-bottom: 8px;
  }
  .panel-subtitle { font-size: 13px; color: var(--on-surface-variant); max-width: 560px; }
  .panel-subtitle strong { color: var(--primary); font-family: var(--font-label); }

  .panel-card {
    background: #12121a; border: 1px solid rgba(59,74,68,0.1);
    position: relative; overflow: hidden;
  }
  .panel-meta-corner {
    position: absolute; top: 12px; right: 16px;
    font-family: var(--font-label); font-size: 10px; color: rgba(70,241,197,0.3);
    display: flex; flex-direction: column; align-items: flex-end; gap: 4px;
    pointer-events: none;
  }

  .panel-tabs { display: flex; border-bottom: 1px solid rgba(59,74,68,0.1); }
  .panel-tab {
    padding: 20px 32px; background: none; border: none; cursor: pointer;
    display: flex; align-items: center; gap: 8px;
    font-family: var(--font-headline); font-size: 12px; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.15em;
    color: var(--on-surface-variant); transition: background 0.15s;
  }
  .panel-tab:hover { background: var(--surface-high); }
  .panel-tab-active { background: var(--primary-container); color: var(--on-primary); }

  .server-table-header {
    display: flex; padding: 12px 32px;
    background: rgba(27,27,32,0.5);
    font-family: var(--font-label); font-size: 10px;
    color: var(--outline); text-transform: uppercase; letter-spacing: 0.1em;
  }
  .server-table-body { }
  .server-row {
    display: flex; align-items: center; padding: 20px 32px;
    border-bottom: 1px solid rgba(59,74,68,0.05);
    transition: background 0.15s;
  }
  .server-row:hover { background: rgba(42,41,47,0.3); }
  .server-row-name { display: flex; align-items: center; gap: 16px; }
  .ping-indicator { width: 8px; height: 8px; border-radius: 50%; flex-shrink: 0; }
  .server-row-title { font-weight: 700; font-family: var(--font-body); color: var(--on-surface); font-size: 14px; }
  .server-row-id { font-family: var(--font-label); font-size: 10px; color: var(--outline); margin-top: 2px; }
  .server-row-region { display: flex; align-items: center; gap: 4px; font-size: 12px; color: var(--on-surface-variant); }
  .server-row-users { text-align: center; font-family: var(--font-label); font-size: 12px; color: var(--on-surface); }
  .server-row-ping { text-align: right; font-family: var(--font-label); font-size: 12px; font-weight: 700; }

  .connect-btn {
    width: 100%; padding: 8px;
    background: var(--primary-container); color: var(--on-primary);
    border: none; cursor: pointer; font-family: var(--font-label);
    font-size: 10px; font-weight: 700; text-transform: uppercase;
    transition: filter 0.15s;
  }
  .connect-btn:hover { filter: brightness(1.1); }

  .server-table-footer {
    padding: 16px 32px; background: var(--surface-lowest);
    display: flex; justify-content: space-between; align-items: center;
    border-top: 1px solid rgba(59,74,68,0.1);
  }
  .legend { display: flex; gap: 24px; }
  .legend-item { display: flex; align-items: center; gap: 8px; font-family: var(--font-label); font-size: 10px; color: var(--outline); text-transform: uppercase; }
  .legend-dot { width: 8px; height: 8px; }
  .protocol-tag { font-family: var(--font-label); font-size: 10px; color: rgba(70,241,197,0.5); letter-spacing: 0.15em; }

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

  .stats-grid { display: grid; grid-template-columns: 1fr 2fr; gap: 24px; }
  .stat-card {
    background: var(--surface-low); padding: 24px;
    display: flex; flex-direction: column; gap: 16px;
  }
  .stat-card-wide { flex-direction: row; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 16px; }
  .stat-label { font-family: var(--font-label); font-size: 11px; color: var(--primary); text-transform: uppercase; letter-spacing: 0.15em; }
  .stat-value { font-family: var(--font-headline); font-size: 36px; font-weight: 900; }
  .stat-unit { font-family: var(--font-label); font-size: 10px; color: var(--outline); vertical-align: super; }
  .stat-bar { width: 100%; height: 4px; background: var(--surface-highest); }
  .stat-bar-fill { height: 100%; background: var(--primary); }
  .stat-server-name { font-family: var(--font-headline); font-size: 22px; font-weight: 700; }
  .stat-subtitle { font-size: 12px; color: var(--on-surface-variant); }
  .auto-connect-btn {
    padding: 10px 24px; background: none; border: 1px solid rgba(70,241,197,0.3);
    color: var(--primary); font-family: var(--font-label); font-size: 12px;
    cursor: pointer; transition: background 0.15s; text-transform: uppercase;
  }
  .auto-connect-btn:hover { background: rgba(70,241,197,0.1); }

  /* ── Channel Header ───────────────────────────────── */
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

  /* ── Messages ─────────────────────────────────────── */
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

  /* ── Chat Input ───────────────────────────────────── */
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

  /* ── Competition Voice View ───────────────────────── */
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
  .speaker-card {
    background: var(--surface-high); padding: 24px;
    display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px;
    border: 1px solid rgba(59,74,68,0.2); position: relative;
  }
  .speaker-active { border: 2px solid var(--error); background: var(--surface-high); }
  .speaker-you { border: 1px solid rgba(255,180,171,0.4); }
  .speaker-mic-icon { position: absolute; top: 12px; right: 12px; }
  .speaker-avatar {
    width: 80px; height: 80px; background: var(--surface-highest);
    display: flex; align-items: center; justify-content: center;
    font-size: 20px; font-weight: 700; color: var(--on-surface);
  }
  .speaker-avatar-muted { opacity: 0.5; filter: grayscale(1); }
  .speaker-name { font-family: var(--font-label); font-size: 12px; font-weight: 700; }
  .speaker-name.muted { color: var(--on-surface-variant); }
  .speaker-bar { width: 100%; height: 4px; background: rgba(255,180,171,0.1); margin-top: 8px; }
  .speaker-bar-fill { height: 100%; background: var(--error); }
  .speaker-bar-empty { background: rgba(255,255,255,0.05); }
  .speaker-controls { display: flex; gap: 8px; color: var(--on-surface-variant); }

  .tactical-panel {
    background: var(--surface-high); padding: 24px;
    border-left: 1px solid rgba(255,180,171,0.1); overflow-y: auto;
    display: flex; flex-direction: column; gap: 32px;
  }
  .tactical-title { font-family: var(--font-headline); font-size: 11px; font-weight: 700; color: var(--error); text-transform: uppercase; letter-spacing: 0.15em; }
  .tactical-metric { display: flex; flex-direction: column; gap: 8px; }
  .metric-header { display: flex; justify-content: space-between; align-items: flex-end; }
  .metric-label { font-family: var(--font-label); font-size: 10px; color: var(--on-surface-variant); text-transform: uppercase; }
  .metric-value { font-family: var(--font-label); font-size: 14px; font-weight: 700; }
  .metric-bars { display: flex; align-items: flex-end; gap: 3px; height: 48px; }
  .metric-bar { flex: 1; background: rgba(255,180,171,0.3); min-height: 4px; }
  .metric-bar:last-child { background: var(--error); }
  .cpu-bar { width: 100%; height: 4px; background: var(--surface-highest); }
  .cpu-bar-fill { height: 100%; background: var(--primary-container); }

  .tactical-filtered { display: flex; flex-direction: column; gap: 12px; padding-top: 24px; border-top: 1px solid rgba(59,74,68,0.1); }
  .tactical-filtered-title { font-family: var(--font-headline); font-size: 10px; font-weight: 700; color: var(--on-surface-variant); text-transform: uppercase; letter-spacing: 0.2em; }
  .filtered-items { display: flex; flex-direction: column; gap: 12px; opacity: 0.3; }
  .filtered-item { display: flex; align-items: center; gap: 12px; }
  .filtered-bar { height: 8px; background: var(--outline-variant); }

  .competition-note {
    margin-top: auto; padding: 16px;
    background: rgba(255,180,171,0.05); border: 1px solid rgba(255,180,171,0.2);
    font-family: var(--font-label); font-size: 10px; color: rgba(255,180,171,0.8);
    text-transform: uppercase; line-height: 1.6;
  }

  .enc-log {
    background: var(--surface-lowest); padding: 16px 32px;
    font-family: var(--font-label); font-size: 10px; position: relative; flex-shrink: 0;
    max-height: 120px; overflow: hidden;
  }
  .enc-log-title { position: absolute; top: 8px; right: 16px; color: rgba(255,180,171,0.3); text-transform: uppercase; }
  .enc-log-lines { color: var(--on-surface-variant); display: flex; flex-direction: column; gap: 4px; }
  .log-ts { color: var(--error); }
  .log-cursor { animation: pulse 1s infinite; }

  .floating-bar {
    position: absolute; bottom: 32px; left: 50%; transform: translateX(-50%);
    display: flex; align-items: center; gap: 16px;
    background: rgba(42,41,47,0.9); backdrop-filter: blur(12px);
    padding: 12px 24px; border: 1px solid rgba(255,180,171,0.2);
    box-shadow: 0 0 20px rgba(255,180,171,0.1);
  }
  .float-btn {
    display: flex; align-items: center; gap: 8px;
    background: none; border: none; cursor: pointer;
    font-family: var(--font-label); font-size: 11px; font-weight: 700;
    color: var(--on-surface); text-transform: uppercase; padding: 8px 16px;
    transition: background 0.15s;
  }
  .float-btn:hover { background: rgba(255,255,255,0.05); }
  .float-btn-danger { color: var(--error); }
  .float-btn-danger:hover { background: rgba(255,180,171,0.1); }
  .float-btn-disconnect { color: #ff5252; }
  .float-btn-disconnect:hover { background: rgba(255,82,82,0.1); }
  .float-divider { width: 1px; height: 24px; background: rgba(59,74,68,0.3); }

  /* ── Animations ───────────────────────────────────── */
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }
</style>
