/**
 * Returns a CSS color string based on ping value.
 * @param {number} ping - Ping in milliseconds
 * @returns {string} CSS color string
 */
export function pingColor(ping) {
  if (ping <= 20) return 'var(--primary)';
  if (ping <= 80) return 'var(--tertiary-container)';
  return 'var(--error)';
}

/**
 * Returns a status label based on ping value.
 * @param {number} ping - Ping in milliseconds
 * @returns {'optimal'|'acceptable'|'critical'}
 */
export function pingStatus(ping) {
  if (ping <= 20) return 'optimal';
  if (ping <= 80) return 'acceptable';
  return 'critical';
}
