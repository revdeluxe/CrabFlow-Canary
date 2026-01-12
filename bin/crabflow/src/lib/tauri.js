// Unified API wrapper that works in both Tauri and regular browser environments

// Detect if we're running inside Tauri
const isTauri = typeof window !== 'undefined' && window.__TAURI_INTERNALS__;

// Dynamic invoke - only works in Tauri context
let invoke = async () => { throw new Error('Tauri not available'); };

if (isTauri) {
    // Dynamically import Tauri API only when in Tauri context
    import('@tauri-apps/api/core').then(module => {
        invoke = module.invoke;
    }).catch(e => {
        console.warn('Failed to load Tauri API:', e);
    });
}

// Determine API URL based on environment
function getApiUrl() {
    if (typeof window === 'undefined') return 'http://localhost:3030/api';
    
    // In browser, use the same origin but route to API port
    // For captive portal access, the API is on port 3030
    const host = window.location.hostname;
    return `http://${host}:3030/api`;
}

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
        const API_URL = getApiUrl();
        const response = await fetch(`${API_URL}${endpoint}`, options);
        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(errorText || `HTTP error! status: ${response.status}`);
        }
        return await response.json();
    } catch (e) {
        console.error(`Fetch failed for ${endpoint}:`, e);
        throw e;
    }
}

// Wrapper that tries invoke first (Tauri), falls back to HTTP
async function invokeOrFetch(command, args = {}, httpFallback = null) {
    if (isTauri) {
        try {
            // Wait a moment for dynamic import to complete
            await new Promise(resolve => setTimeout(resolve, 10));
            return await invoke(command, args);
        } catch (e) {
            if (httpFallback) {
                console.warn(`Invoke '${command}' failed, trying HTTP fallback:`, e);
                return await httpFallback();
            }
            throw e;
        }
    } else if (httpFallback) {
        return await httpFallback();
    } else {
        throw new Error(`Command '${command}' requires Tauri (not available in browser)`);
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
  // Environment check
  isTauri: () => isTauri,

    // Generic invoke wrapper for compatibility
    invokeCommand: async (command, args = {}) => invokeOrFetch(command, args, null),

  // Setup - require Tauri for admin functions, but provide browser fallbacks where sensible
  getWizardStatus: async () => invokeOrFetch('get_wizard_status', {}, () => request('/setup/wizard-status')),
  checkFirstRun: async () => invokeOrFetch('check_first_run', {}, () => Promise.resolve(false)), // Browser: assume not first run
  saveSetup: async (config) => invokeOrFetch('save_setup', { config }),
  loadSetup: async () => invokeOrFetch('load_setup', {}, () => request('/setup/config')),
  resetSetup: async () => invokeOrFetch('reset_setup'),
  
  // Auth - Use HTTP for browser compatibility
  login: async (username, password) => await request('/auth/login', 'POST', { username, password }),
  registerUser: async (username, password) => await request('/auth/register', 'POST', { username, password }),
  logout: async (token) => await request('/auth/logout', 'POST', { token }),
  checkAuth: async (token) => await request('/auth/check', 'POST', { token }),
  
  // User Management
  listUsers: async () => await request('/users'),
  updateUserStatus: async (username, active, approved) => invokeOrFetch('update_user_status', { username, active, approved }),
  updateUserGroups: async (username, groups) => invokeOrFetch('update_user_groups', { username, groups }),
  updateUserProfile: async (username, nickname, email) => invokeOrFetch('update_user_profile', { username, nickname, email }),
  changePassword: async (username, newPassword) => invokeOrFetch('change_password', { username, newPassword }),
  getUserSettings: async () => invokeOrFetch('get_user_settings'),
  setUserSettings: async (settings) => invokeOrFetch('set_user_settings', { settings }),
  
  // Group Management
  listGroups: async () => invokeOrFetch('list_groups', {}, () => request('/groups')),
  addGroup: async (name, description, permissions) => invokeOrFetch('add_group', { name, description, permissions }),
  updateGroup: async (name, description, permissions) => invokeOrFetch('update_group', { name, description, permissions }),
  deleteGroup: async (name) => invokeOrFetch('delete_group', { name }),
  listPermissions: async () => invokeOrFetch('list_permissions'),

  // Network - Use HTTP where available
  listLeases: async () => await request('/dhcp/leases'),
  listRecords: async () => await request('/dns/records'),
  getQueryLogs: async (limit) => invokeOrFetch('get_query_logs', { limit }, () => request(`/dns/logs?limit=${limit}`)),
  getSystemStatus: async () => await request('/system/status'),
  getTrafficSummary: async () => invokeOrFetch('get_traffic_summary', {}, () => request('/traffic/summary')),
  listInterfaces: async () => invokeOrFetch('list_interfaces', {}, () => request('/network/interfaces')),
  listDevices: async () => invokeOrFetch('list_devices', {}, () => request('/devices')),
  updateUpstreamInterface: async (ip) => invokeOrFetch('update_upstream_interface', { ip }),
  
  // Hotspot (Tauri only - requires system access)
  createHotspot: async (ssid, key) => invokeOrFetch('create_hotspot', { ssid, key }),
  stopHotspot: async () => invokeOrFetch('stop_hotspot'),

  // Captive Portal & History
  tagUser: async (username, ip, deviceName) => invokeOrFetch('tag_user', { username, ip, deviceName }, 
    () => request('/portal/tag', 'POST', { username, ip, device_name: deviceName })),
  getUserHistory: async (username) => invokeOrFetch('get_user_history', { username }),
  uploadId: async (username, fileData) => invokeOrFetch('upload_id', { username, fileData }),
  setCaptivePortal: async (enabled) => invokeOrFetch('set_captive_portal', { enabled }),
  setCustomPortal: async (enabled) => invokeOrFetch('set_custom_portal', { enabled }),
  getPortalTemplate: async () => invokeOrFetch('get_portal_template', {}, () => request('/portal/template')),
  savePortalTemplate: async (content) => invokeOrFetch('save_portal_template', { content }),

  // Logs
  getLogs: async (limit) => await request('/logs'), 
  loadLoggingConfig: async () => invokeOrFetch('load_logging_config'),
  saveLoggingConfig: async (config) => invokeOrFetch('save_logging_config', { config }),
  reloadLoggingConfig: async () => invokeOrFetch('reload_logging_config'),
  clearLogs: async () => invokeOrFetch('clear_logs'),

  // Power (Tauri only - requires system access)
  shutdownSystem: async () => invokeOrFetch('shutdown_system'),
  restartSystem: async () => invokeOrFetch('restart_system'),
  restartNetworking: async () => invokeOrFetch('restart_networking'),
  restartApplication: async () => invokeOrFetch('restart_application'),
};
