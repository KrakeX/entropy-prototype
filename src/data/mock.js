export const servers = [
  { id: 1, name: 'Entropy Latam', icon: 'EL', type: 'public', members: 2847, ping: 18, verified: true },
  { id: 2, name: 'OW Ranked Chile', icon: 'OW', type: 'public', members: 412, ping: 12, verified: false },
  { id: 3, name: 'Team Flux', icon: 'TF', type: 'private', members: 14, ping: 2, ip: '192.168.1.50', verified: false },
];

export const publicServers = [
  { id: 10, name: 'Entropy NA East', secureId: 'VR-842-X', members: 8420, ping: 145, region: 'Virginia, US' },
  { id: 11, name: 'Entropy EU West', secureId: 'FR-121-Z', members: 12100, ping: 210, region: 'Frankfurt, DE' },
  { id: 12, name: 'FPS Latinoamérica', secureId: 'SP-189-Y', members: 1893, ping: 22, region: 'São Paulo, BR' },
  { id: 13, name: 'Overwatch LATAM Hub', secureId: 'ST-756-A', members: 756, ping: 15, region: 'Santiago, CL' },
  { id: 14, name: 'Valorant Chile', secureId: 'ST-534-B', members: 534, ping: 11, region: 'Santiago, CL' },
];

export const textChannels = [
  { id: 't1', name: 'general', unread: 3 },
  { id: 't2', name: 'estrategias', unread: 0 },
  { id: 't3', name: 'scrims', unread: 1 },
  { id: 't4', name: 'off-topic', unread: 0 },
];

export const voiceChannels = [
  { id: 'v1', name: 'Competitivo', users: [{ name: 'Durán', ping: '2ms' }, { name: 'Celis', ping: '4ms' }, { name: 'NightOwl', ping: '1ms' }] },
  { id: 'v2', name: 'Práctica', users: [] },
  { id: 'v3', name: 'Reunión Staff', users: [] },
];

export const messages = [
  { id: 1, user: 'NightOwl', time: '14:32', text: '¿Alguien para revisar los VODs del scrimmage de anoche? Creo que fallamos en la rotación de B en la ronda 12.', initials: 'NO', highlight: false, userColor: 'primary' },
  { id: 2, user: 'Phantom', time: '14:35', text: 'Esa rotación fue tardía porque Durán se quedó sin utilidad temprano. Tenemos que coordinar mejor los humos para el re-take.', initials: 'PH', highlight: false, userColor: 'on-surface' },
  { id: 3, user: 'Durán', time: '14:36', text: 'Mala mía. El bait de ellos en A me hizo gastar todo. Para la próxima espero el call de Celis antes de soltar el kit completo.', initials: 'DU', highlight: false, userColor: 'secondary' },
  { id: 4, user: 'Celis', time: '14:38', text: 'Entendido. Usemos el server de práctica en 15 min para ajustar eso. NightOwl, trae los clips marcados.', initials: 'CE', highlight: false, userColor: 'on-surface' },
  { id: 5, user: 'NightOwl', time: '14:40', text: 'Listo. Ya los subí a #estrategias. Nos vemos en el canal de voz en 10.', initials: 'NO', highlight: true, userColor: 'primary' },
];
