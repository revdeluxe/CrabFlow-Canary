// Tauri doesn't have a Node.js server to do proper SSR
// Disable SSR for all portal pages to avoid errors with Tauri invoke
export const ssr = false;
