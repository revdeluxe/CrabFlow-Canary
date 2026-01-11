import { invoke } from '@tauri-apps/api/core';

const API_URL = "http://localhost:3030/api";

async function request(endpoint, method = "GET", body = null) {
    try {
        const options = {
            method,
            headers: {
                'Content-Type': 'application/json',
            },
        };
        if (body) {
            options.body = JSON.stringify(body);
        }
        const response = await fetch(`${API_URL}${endpoint}`, options);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return await response.json();
    } catch (e) {
        console.error(`Fetch failed for ${endpoint}, falling back to invoke if possible`, e);
        throw e;
    }
}

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
  getWizardStatus: async () => await invoke('get_wizard_status'),
  checkFirstRun: async () => await invoke('check_first_run'),
  saveSetup: async (config) => await invoke('save_setup', { config }),
  loadSetup: async () => await invoke('load_setup'),
  resetSetup: async () => await invoke('reset_setup'),
  
  // Auth
  login: async (username, password) => await request('/auth/login', 'POST', { username, password }),
  registerUser: async (username, password) => await request('/auth/register', 'POST', { username, password }),
  logout: async (token) => await invoke('logout', { token }), // Not yet migrated
  checkAuth: async (token) => await request('/auth/check', 'POST', { token }),
  
  // User Management
  listUsers: async () => await request('/users'),
  updateUserStatus: async (username, active, approved) => await invoke('update_user_status', { username, active, approved }),
  updateUserGroups: async (username, groups) => await invoke('update_user_groups', { username, groups }),
  updateUserProfile: async (username, nickname, email) => await invoke('update_user_profile', { username, nickname, email }),
  changePassword: async (username, newPassword) => await invoke('change_password', { username, newPassword }),
  getUserSettings: async () => await invoke('get_user_settings'),
  setUserSettings: async (settings) => await invoke('set_user_settings', { settings }),
  
  // Group Management
  listGroups: async () => await invoke('list_groups'),
  addGroup: async (name, description, permissions) => await invoke('add_group', { name, description, permissions }),
  updateGroup: async (name, description, permissions) => await invoke('update_group', { name, description, permissions }),
  deleteGroup: async (name) => await invoke('delete_group', { name }),
  listPermissions: async () => await invoke('list_permissions'),

  // Network
  listLeases: async () => await request('/dhcp/leases'),
  listRecords: async () => await request('/dns/records'),
  getQueryLogs: async (limit) => await invoke('get_query_logs', { limit }),
  getSystemStatus: async () => await request('/system/status'),
  getTrafficSummary: async () => await invoke('get_traffic_summary'),
  listInterfaces: async () => await invoke('list_interfaces'),
  listDevices: async () => await invoke('list_devices'),
  updateUpstreamInterface: async (ip) => await invoke('update_upstream_interface', { ip }),
  
  // Hotspot
  createHotspot: async (ssid, key) => await invoke('create_hotspot', { ssid, key }),
  stopHotspot: async () => await invoke('stop_hotspot'),

  // Captive Portal & History
  tagUser: async (username, ip, deviceName) => await invoke('tag_user', { username, ip, deviceName }),
  getUserHistory: async (username) => await invoke('get_user_history', { username }),
  uploadId: async (username, fileData) => await invoke('upload_id', { username, fileData }),
  setCaptivePortal: async (enabled) => await invoke('set_captive_portal', { enabled }),
  setCustomPortal: async (enabled) => await invoke('set_custom_portal', { enabled }),
  getPortalTemplate: async () => await invoke('get_portal_template'),
  savePortalTemplate: async (content) => await invoke('save_portal_template', { content }),

  // Logs
  getLogs: async (limit) => await request('/logs'), 
  loadLoggingConfig: async () => await invoke('load_logging_config'),
  saveLoggingConfig: async (config) => await invoke('save_logging_config', { config }),
  reloadLoggingConfig: async () => await invoke('reload_logging_config'),
  clearLogs: async () => await invoke('clear_logs'),

  // Power
  shutdownSystem: async () => await invoke('shutdown_system'),
  restartSystem: async () => await invoke('restart_system'),
  restartNetworking: async () => await invoke('restart_networking'),
  restartApplication: async () => await invoke('restart_application'),
};
