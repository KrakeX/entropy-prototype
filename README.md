# ENTROPY — Secure Communication Platform

Cliente de escritorio para una plataforma de comunicación en tiempo real para eSports, con enfoque **privacy-first** y arquitectura federada.

> Proyecto académico — Taller de Innovación y Emprendimiento, Solemne 1
> Universidad San Sebastián, Chile — 2026

---

## Stack

### Frontend
- **Framework:** Svelte 5
- **Build tool:** Vite 8
- **Diseño:** Google Stitch (design system propio)
- **Fuentes:** Space Grotesk · Manrope · JetBrains Mono
- **Iconos:** Material Symbols

### Desktop (Tauri v2)
- **Runtime:** Tauri 2 — envuelve el frontend Svelte en una ventana nativa
- **Backend:** Rust (tokio, async)
- **Audio:** cpal (captura/playback cross-platform) + Opus (codec)
- **Red:** UDP (voz) + QUIC via quinn (señalización/texto)
- **Cifrado:** X25519 ECDH (key exchange) + AES-256-GCM (voz y mensajes)

---

## Estructura del proyecto

```
src/                          # Frontend Svelte
├── features/                 # Screaming architecture por dominio
│   ├── welcome/
│   ├── servers/
│   ├── channels/
│   └── competition/
├── components/               # UI compartida cross-feature
├── stores/                   # Estado global (Svelte 5 $state)
├── data/                     # Mock data
├── utils/                    # Helpers
└── styles/                   # Design tokens CSS

src-tauri/                    # Backend Rust
├── src/
│   ├── commands/             # Comandos invocables desde Svelte
│   ├── voice/                # Captura, playback, codec, UDP, mezcla
│   ├── signaling/            # QUIC client + handler de mensajes
│   └── crypto/               # Key exchange, cifrado, descifrado
└── tauri.conf.json
```

---

## Instalación

### Prerequisitos
- Node.js + pnpm
- Rust + Cargo

```bash
pnpm install
```

### Desarrollo web (solo frontend)
```bash
pnpm dev
```

### Desarrollo desktop (Tauri)
```bash
pnpm tauri dev
```

### Build web (GitHub Pages)
```bash
pnpm build
# Output en docs/ (configurado para GitHub Pages)
```

### Build desktop
```bash
pnpm tauri build
```

---

## Pantallas implementadas

| Pantalla | Descripción |
|---|---|
| Welcome | Vista de bienvenida con grid de features y lista de servidores |
| Red Pública | Explorador de nodos geográficos con indicadores de latencia |
| Servidor Privado | Formulario de conexión directa por IP:puerto |
| Canal — Normal | Vista de canales de texto y voz con chat |
| Canal — Competición | Modo de bajo consumo con grid de voz y métricas tácticas |

---

## Propuesta de valor

1. **Cifrado E2E** — X25519 ECDH + AES-256-GCM. El servidor nunca ve el contenido en claro. Sin biometría ni documentos de identidad.
2. **Arquitectura federada** — Servidores públicos + self-hosted + conexión directa por IP:puerto.
3. **Modo Competición** — Recursos reducidos, notificaciones silenciadas, métricas de red en tiempo real.

---

## Autores

Pablo Durán Celis
