import { textChannels } from '../data/mock.js';

export const app = $state({
  currentView: 'welcome',   // 'welcome' | 'channel'
  currentServer: null,
  competitionMode: false,
  addServerPanel: false,    // en welcome, mostrar panel add server
  selectedChannel: { id: 't1', name: 'general', type: 'text' },
  addServerTab: 'public',
});

export function enterServer(server) {
  app.currentServer = server;
  app.currentView = 'channel';
  app.selectedChannel = textChannels[0];
  app.addServerPanel = false;
}

export function goBack() {
  app.currentView = 'welcome';
  app.currentServer = null;
}

export function toggleCompetition() {
  app.competitionMode = !app.competitionMode;
}
