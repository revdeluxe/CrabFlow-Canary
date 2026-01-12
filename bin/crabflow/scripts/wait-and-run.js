#!/usr/bin/env node
/**
 * Cross-platform script to wait for Vite dev server and launch Tauri app with elevation
 * Works on Windows, Linux, and macOS
 */

import { spawn } from 'child_process';
import { platform } from 'os';
import { existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const projectRoot = join(__dirname, '..');

const VITE_PORT = 1420;
const MAX_RETRIES = 30;
const RETRY_DELAY = 1000;

async function waitForVite() {
    console.log(`⏳ Waiting for Vite dev server on port ${VITE_PORT}...`);
    
    for (let i = 0; i < MAX_RETRIES; i++) {
        try {
            const response = await fetch(`http://localhost:${VITE_PORT}`);
            if (response.ok || response.status === 200) {
                console.log('✅ Vite dev server is ready!');
                return true;
            }
        } catch (e) {
            // Server not ready yet
        }
        await new Promise(resolve => setTimeout(resolve, RETRY_DELAY));
    }
    
    console.error('❌ Timeout waiting for Vite dev server');
    return false;
}

function getBinaryPath() {
    const os = platform();
    const debugPath = join(projectRoot, 'src-tauri', 'target', 'debug');
    
    if (os === 'win32') {
        return join(debugPath, 'crabflow.exe');
    } else {
        return join(debugPath, 'crabflow');
    }
}

function launchTauriApp() {
    const os = platform();
    const binaryPath = getBinaryPath();
    
    if (!existsSync(binaryPath)) {
        console.error(`❌ Binary not found at: ${binaryPath}`);
        console.error('   Run "npm run cargo:build" first to compile the backend.');
        process.exit(1);
    }
    
    console.log(`🚀 Launching CrabFlow on ${os}...`);
    
    let child;
    
    if (os === 'linux') {
        // Linux: Use sudo for elevation
        child = spawn('sudo', [binaryPath], {
            stdio: 'inherit',
            cwd: projectRoot
        });
    } else if (os === 'darwin') {
        // macOS: Use sudo for elevation
        child = spawn('sudo', [binaryPath], {
            stdio: 'inherit',
            cwd: projectRoot
        });
    } else if (os === 'win32') {
        // Windows: The app handles its own elevation via PowerShell
        child = spawn(binaryPath, [], {
            stdio: 'inherit',
            cwd: projectRoot,
            shell: true
        });
    } else {
        console.error(`❌ Unsupported platform: ${os}`);
        process.exit(1);
    }
    
    child.on('error', (err) => {
        console.error('❌ Failed to launch app:', err.message);
        process.exit(1);
    });
    
    child.on('exit', (code) => {
        process.exit(code || 0);
    });
}

async function main() {
    const ready = await waitForVite();
    if (ready) {
        launchTauriApp();
    } else {
        process.exit(1);
    }
}

main();
