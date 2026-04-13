# ENTROPY — Secure Communication Platform

Prototipo interactivo de plataforma de comunicación en tiempo real para eSports, con enfoque **privacy-first** y arquitectura federada.

> Proyecto académico — Taller de Innovación y Emprendimiento, Solemne 1
> Universidad San Sebastián, Chile — 2026

---

## Stack

- **Framework:** Svelte 5
- **Build tool:** Vite 8
- **Diseño:** Google Stitch (design system propio)
- **Fuentes:** Space Grotesk · Manrope · JetBrains Mono
- **Iconos:** Material Symbols

## Instalación

```bash
pnpm install
pnpm dev
```

## Pantallas implementadas

| Pantalla | Descripción |
|---|---|
| Welcome | Vista de bienvenida con grid de features y lista de servidores |
| Red Pública | Explorador de nodos geográficos con indicadores de latencia |
| Servidor Privado | Formulario de conexión directa por IP:puerto |
| Canal — Normal | Vista de canales de texto y voz con chat |
| Canal — Competición | Modo de bajo consumo con grid de voz y métricas tácticas |

## Propuesta de valor

1. **Cifrado E2E** — Signal Protocol (Double Ratchet), AES-256-GCM, X25519 ECDH. Sin biometría ni documentos de identidad.
2. **Arquitectura federada** — Servidores públicos + self-hosted + conexión directa por IP.
3. **Modo Competición** — Recursos reducidos, notificaciones silenciadas, métricas de red en tiempo real.

## Autores

Pablo Durán Celis
