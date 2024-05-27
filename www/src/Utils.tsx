/**
 * Save text as a text file.
 */
 export function downloadAsHtml(text: string, name: string) {
  const blob = new Blob([text], { type: 'text/html' });
  const link = document.createElement('a');
  link.href = URL.createObjectURL(blob);
  link.download = name;
  link.click();
  link.remove();
}

/**
 * Copy text to clipboard
 */
export function toClipboard(text: string) {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(text);
  }
}