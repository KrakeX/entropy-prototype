/**
 * Detecta si la app corre dentro de Tauri (desktop) o en el navegador (web/GitHub Pages).
 * Cuando corre en el navegador, `invoke` no está disponible y debemos usar mock data.
 */
export const isTauri = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

/**
 * Wrapper seguro sobre invoke que hace fallback al valor por defecto cuando no está en Tauri.
 * @template T
 * @param {string} command — nombre del comando Tauri
 * @param {object} [args] — argumentos del comando
 * @param {T} [fallback] — valor de retorno cuando no estamos en Tauri
 * @returns {Promise<T>}
 */
export async function invoke(command, args = {}, fallback = null) {
  if (!isTauri()) return fallback;
  const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
  return tauriInvoke(command, args);
}
