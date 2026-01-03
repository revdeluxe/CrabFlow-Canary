import { invoke } from '@tauri-apps/api/core';

/**
 * @typedef {Object} SetupConfig
 * @property {string} hostname
 * @property {string} admin_email
 * @property {string} admin_user
 * @property {string} admin_pass
 * @property {boolean} telemetry
 * @property {boolean} first_run
 * @property {number} monitor_interval
 */

/**
 * @typedef {Object} SystemStatus
 * @property {number} cpu_usage
 * @property {number} memory_usage
 * @property {boolean} internet_connected
 * @property {string} active_interface
 * @property {number} timestamp
 */

export const api = {
  // Setup
  checkFirstRun: async () => await invoke('check_first_run'),
  saveSetup: async (config) => await invoke('save_setup', { config }),
  loadSetup: async () => await invoke('load_setup'),
  resetSetup: async () => await invoke('reset_setup'),
  
  // Auth
  login: async (username, password) => await invoke('login', { req: { username, password } }),
  registerUser: async (username, password) => await invoke('register_user', { username, password }),
  logout: async (token) => await invoke('logout', { token }),
  checkAuth: async (token) => await invoke('check_auth', { token }),
  
  // User Management
  listUsers: async () => await invoke('list_users'),
  updateUserStatus: async (username, active, approved) => await invoke('update_user_status', { username, active, approved }),
  updateUserGroups: async (username, groups) => await invoke('update_user_groups', { username, groups }),
  getUserSettings: async () => await invoke('get_user_settings'),
  setUserSettings: async (settings) => await invoke('set_user_settings', { settings }),

  // Network
  listLeases: async () => await invoke('list_leases'),
  listRecords: async () => await invoke('list_records'),
  getSystemStatus: async () => await invoke('get_system_status'),
  
  // Hotspot
  createHotspot: async (ssid, key) => await invoke('create_hotspot', { ssid, key }),
  stopHotspot: async () => await invoke('stop_hotspot'),

  // Captive Portal & History
  tagUser: async (username, ip, mac, deviceName) => await invoke('tag_user', { username, ip, mac, deviceName }),
  getUserHistory: async (username) => await invoke('get_user_history', { username }),
  uploadId: async (username, filePath) => await invoke('upload_id', { username, filePath }),
  setCaptivePortal: async (enabled) => await invoke('set_captive_portal', { enabled }),

  // Logs
  getLogs: async (limit) => await invoke('get_logs', { limit }),

  // Power
  shutdownSystem: async () => await invoke('shutdown_system'),
  restartSystem: async () => await invoke('restart_system'),
  restartNetworking: async () => await invoke('restart_networking'),
  restartApplication: async () => await invoke('restart_application'),
};
