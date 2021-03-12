/**
 * Copy text to clipboard
 */
export function toClipboard(text: string) {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(text);
  }
}