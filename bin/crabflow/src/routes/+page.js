export const ssr = false;

import { redirect } from '@sveltejs/kit'

// Check if running in Tauri
const isTauri = typeof window !== 'undefined' && window.__TAURI_INTERNALS__;

export const load = async () => {
  // Only check first_run in Tauri context
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const firstRun = await invoke('check_first_run');
      if (firstRun) {
        throw redirect(302, '/setup');
      }
    } catch (e) {
      console.warn('First run check failed:', e);
    }
  }
  
  return {};
};
