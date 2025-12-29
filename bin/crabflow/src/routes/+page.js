export const ssr = false;

import { redirect } from '@sveltejs/kit'
import { invoke } from '@tauri-apps/api/core'

export const load = async () => {
  const firstRun = await invoke('check_first_run')

  if (firstRun) {
    throw redirect(302, '/setup')
  }

  return {}
}
