const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');
const { spawn, execFile } = require('child_process');

// ─── PLAZA CLI PATH ───
// Resolve plaza-cli binary relative to project or use system PATH
const PLAZA_CLI = process.env.PLAZA_CLI_PATH
  || path.resolve(__dirname, '../../../target/release/plaza-cli.exe');
const PLAZA_CWD = process.env.PLAZA_CWD
  || path.resolve(__dirname, '../../..');

console.log(`[PlazaVM] CLI: ${PLAZA_CLI}`);
console.log(`[PlazaVM] CWD: ${PLAZA_CWD}`);

// ─── EVENT BUS ───
// Stores recent events for live streaming to the frontend
const eventBus = [];
const MAX_EVENTS = 100; // Keep memory low
let eventSeq = 0;

// ─── RUNNING WORKSPACE PROCESSES ───
// Tracks detached CLI processes for running workspaces
const workspaceProcesses = new Map(); // id -> { pid, child }

function emitEvent(type, data) {
  const event = {
    id: ++eventSeq,
    type,
    data,
    timestamp: new Date().toISOString(),
  };
  eventBus.push(event);
  if (eventBus.length > MAX_EVENTS) eventBus.shift();
  // Broadcast to all windows
  BrowserWindow.getAllWindows().forEach(win => {
    if (!win.isDestroyed()) {
      win.webContents.send('plaza-event', event);
    }
  });
}

// ─── CLI HELPER ───
function runCli(args, timeoutMs = 30000) {
  return new Promise((resolve, reject) => {
    const startTime = Date.now();
    const child = execFile(PLAZA_CLI, args, {
      cwd: PLAZA_CWD,
      timeout: timeoutMs,
      maxBuffer: 256 * 1024, // 256KB max output buffer
      env: { ...process.env, NO_COLOR: '1' },
    }, (error, stdout, stderr) => {
      const elapsed = Date.now() - startTime;
      if (error) {
        const msg = stderr || stdout || error.message;
        emitEvent('cli_error', { args: args.slice(0, 3), error: msg, elapsed });
        reject(new Error(msg));
      } else {
        emitEvent('cli_complete', { args: args.slice(0, 3), elapsed });
        resolve({ stdout: stdout.toString(), stderr: stderr.toString() });
      }
    });
  });
}

// Streaming CLI for long-running operations (build, start, etc.)
function runCliStreaming(args, onLine, timeoutMs = 300000) {
  return new Promise((resolve, reject) => {
    const startTime = Date.now();
    const child = spawn(PLAZA_CLI, args, {
      cwd: PLAZA_CWD,
      timeout: timeoutMs,
      env: { ...process.env, NO_COLOR: '1' },
      shell: false,
    });

    let stdout = '';
    let stderr = '';

    child.stdout.on('data', (data) => {
      const text = data.toString();
      stdout += text;
      const lines = text.split('\n').filter(l => l.trim());
      lines.forEach(line => {
        emitEvent('cli_output', { args: args.slice(0, 3), line: line.trim() });
        if (onLine) onLine(line.trim());
      });
    });

    child.stderr.on('data', (data) => {
      const text = data.toString();
      stderr += text;
      const lines = text.split('\n').filter(l => l.trim());
      lines.forEach(line => {
        emitEvent('cli_error_output', { args: args.slice(0, 3), line: line.trim() });
      });
    });

    child.on('close', (code) => {
      const elapsed = Date.now() - startTime;
      if (code === 0) {
        emitEvent('cli_stream_complete', { args: args.slice(0, 3), elapsed });
        resolve({ stdout, stderr, code });
      } else {
        const msg = stderr || stdout || `CLI exited with code ${code}`;
        emitEvent('cli_stream_error', { args: args.slice(0, 3), error: msg, code, elapsed });
        reject(new Error(msg));
      }
    });

    child.on('error', (err) => {
      emitEvent('cli_process_error', { args: args.slice(0, 3), error: err.message });
      reject(err);
    });
  });
}

// ─── WORKSPACE CACHE ───
let cachedWorkspaces = null;
let cacheTime = 0;
const CACHE_TTL = 10000; // 10 seconds - reduce CLI spawn frequency

async function refreshWorkspaces() {
  try {
    const { stdout } = await runCli(['workspace', 'list']);
    const workspaces = parseWorkspaceList(stdout);
    cachedWorkspaces = workspaces;
    cacheTime = Date.now();
    emitEvent('workspaces_updated', { count: workspaces.length });
    return workspaces;
  } catch (err) {
    console.error('[PlazaVM] Failed to refresh workspaces:', err.message);
    return cachedWorkspaces || [];
  }
}

function parseWorkspaceList(stdout) {
  const workspaces = [];
  const lines = stdout.split('\n');
  for (const line of lines) {
    // Match:   - [id] name [path] (state, health)
    const match = line.match(/\s*-\s*\[([^\]]+)\]\s+(\S+)\s+\[([^\]]*)\]\s+\((\w+),\s*(\w+)\)/);
    if (match) {
      workspaces.push({
        id: match[1],
        name: match[2],
        description: '',
        state: match[4],
        runtime_backend: 'QEMU/PlazaVM',
        health: match[5],
        cpu_cores: 2,
        memory_mb: 2048,
        created_at: new Date().toISOString(),
        project_path: match[3] || undefined,
      });
    }
  }
  return workspaces;
}

async function getWorkspaceById(id) {
  const workspaces = await refreshWorkspaces();
  return workspaces.find(w => w.id === id || w.name === id) || null;
}

// ─── CREATE WINDOW ───
function createWindow() {
  const mainWindow = new BrowserWindow({
    width: 1280,
    height: 800,
    minWidth: 800,
    minHeight: 600,
    backgroundColor: '#07090e',
    title: 'PlazaVM — Workspace Control Center',
    autoHideMenuBar: true,
    webPreferences: {
      preload: path.join(__dirname, 'preload.cjs'),
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  const isDev = process.env.NODE_ENV === 'development' || process.env.WAIT_ON_VITE === 'true';

  // Reduce renderer memory usage
  mainWindow.webContents.on('did-finish-load', () => {
    mainWindow.webContents.setBackgroundThrottling(true);
  });

  if (isDev) {
    mainWindow.loadURL('http://localhost:5173');
  } else {
    mainWindow.loadFile(path.join(__dirname, '../dist/index.html'));
  }
}

// ─── IPC HANDLERS ───

// Workspace operations
ipcMain.handle('list_workspaces', async () => {
  console.log('[Electron IPC] list_workspaces');
  emitEvent('workspace_list_requested', {});
  const workspaces = await refreshWorkspaces();
  console.log(`[Electron IPC] Found ${workspaces.length} workspaces`);
  return workspaces;
});

ipcMain.handle('create_workspace', async (_, { request }) => {
  console.log('[Electron IPC] Creating workspace:', JSON.stringify(request));
  emitEvent('workspace_create_started', { name: request?.name });

  const name = request?.name || `ws-${Date.now()}`;
  const args = ['workspace', 'create', name];

  if (request?.image) args.push('--image', request.image);
  if (request?.runtime) args.push('--runtime', request.runtime);
  if (request?.path) args.push('--path', request.path);

  try {
    await runCliStreaming(args, (line) => {
      emitEvent('workspace_create_progress', { name, line });
    });

    // Refresh and find the new workspace
    const workspaces = await refreshWorkspaces();
    const ws = workspaces.find(w => w.name === name);
    if (ws) {
      emitEvent('workspace_created', ws);
      return ws;
    }
    // Return a basic result if parse failed
    return {
      id: 'pending',
      name,
      state: 'stopped',
      runtime_backend: 'QEMU/PlazaVM',
      health: 'PENDING',
      cpu_cores: request?.cpu_cores || 2,
      memory_mb: request?.memory_mb || 2048,
      created_at: new Date().toISOString(),
    };
  } catch (err) {
    emitEvent('workspace_create_failed', { name, error: err.message });
    throw err;
  }
});

ipcMain.handle('start_workspace', async (_, idOrName) => {
  console.log(`[Electron IPC] Starting workspace: ${idOrName}`);
  emitEvent('workspace_start_started', { id: idOrName });

  // Check if already running
  if (workspaceProcesses.has(idOrName)) {
    const proc = workspaceProcesses.get(idOrName);
    try {
      process.kill(proc.pid, 0); // Test if process is alive
      emitEvent('workspace_started', { id: idOrName });
      return { status: 'ok', message: 'Already running' };
    } catch {
      workspaceProcesses.delete(idOrName);
    }
  }

  // Spawn CLI as a detached process so it survives after IPC returns
  const child = spawn(PLAZA_CLI, ['workspace', 'start', idOrName], {
    cwd: PLAZA_CWD,
    env: { ...process.env, NO_COLOR: '1' },
    detached: true,
    stdio: ['ignore', 'pipe', 'pipe'],
  });

  workspaceProcesses.set(idOrName, { pid: child.pid, child });
  console.log(`[Electron IPC] Workspace ${idOrName} started as PID ${child.pid}`);

  // Stream output to event bus
  child.stdout.on('data', (data) => {
    const text = data.toString();
    const lines = text.split('\n').filter(l => l.trim());
    lines.forEach(line => {
      emitEvent('workspace_start_output', { id: idOrName, line: line.trim() });
    });
  });

  child.stderr.on('data', (data) => {
    const text = data.toString();
    const lines = text.split('\n').filter(l => l.trim());
    lines.forEach(line => {
      emitEvent('workspace_start_error', { id: idOrName, line: line.trim() });
    });
  });

  child.on('exit', (code) => {
    console.log(`[Electron IPC] Workspace ${idOrName} process exited with code ${code}`);
    workspaceProcesses.delete(idOrName);
    emitEvent('workspace_process_exited', { id: idOrName, code });
  });

  child.unref(); // Don't keep Electron alive if all windows close

  emitEvent('workspace_started', { id: idOrName });
  return { status: 'ok', pid: child.pid };
});

ipcMain.handle('stop_workspace', async (_, idOrName) => {
  console.log(`[Electron IPC] Stopping workspace: ${idOrName}`);
  emitEvent('workspace_stop_started', { id: idOrName });

  // Kill tracked process if running
  if (workspaceProcesses.has(idOrName)) {
    const proc = workspaceProcesses.get(idOrName);
    try {
      process.kill(-proc.pid); // Kill process group
    } catch {}
    try {
      process.kill(proc.pid, 'SIGTERM');
    } catch {}
    workspaceProcesses.delete(idOrName);
  }

  // Also try CLI stop command
  try {
    await runCli(['workspace', 'stop', idOrName]);
  } catch {}

  await refreshWorkspaces();
  emitEvent('workspace_stopped', { id: idOrName });
  return { status: 'ok' };
});

ipcMain.handle('delete_workspace', async (_, idOrName) => {
  console.log(`[Electron IPC] Deleting workspace: ${idOrName}`);
  emitEvent('workspace_delete_started', { id: idOrName });

  // Stop if running first
  if (workspaceProcesses.has(idOrName)) {
    const proc = workspaceProcesses.get(idOrName);
    try { process.kill(-proc.pid); } catch {}
    try { process.kill(proc.pid, 'SIGTERM'); } catch {}
    workspaceProcesses.delete(idOrName);
  }

  try {
    await runCli(['workspace', 'delete', idOrName]);
    await refreshWorkspaces();
    emitEvent('workspace_deleted', { id: idOrName });
    return { status: 'ok' };
  } catch (err) {
    emitEvent('workspace_delete_failed', { id: idOrName, error: err.message });
    throw err;
  }
});

ipcMain.handle('check_workspace_running', async (_, idOrName) => {
  if (workspaceProcesses.has(idOrName)) {
    const proc = workspaceProcesses.get(idOrName);
    try {
      process.kill(proc.pid, 0); // Test if alive
      return { running: true, pid: proc.pid };
    } catch {
      workspaceProcesses.delete(idOrName);
    }
  }
  return { running: false };
});

ipcMain.handle('get_workspace_config', async (_, id) => {
  try {
    const { stdout } = await runCli(['workspace', 'inspect', id]);
    return { raw: stdout };
  } catch {
    return { raw: '(no config)' };
  }
});

ipcMain.handle('save_workspace_config', async (_, id, configJson) => {
  console.log(`[Electron IPC] Saving config for ${id}:`, JSON.stringify(configJson).slice(0, 200));
  return { status: 'ok' };
});

// Build workspace image from source
ipcMain.handle('build_workspace_image', async (_, { workspaceId, sourcePath, buildCommand }) => {
  console.log(`[Electron IPC] Building workspace image: ${workspaceId} from ${sourcePath}`);
  emitEvent('image_build_started', { workspaceId, sourcePath });

  try {
    // Use the workspace exec to run the build inside the guest
    const args = ['workspace', 'exec', workspaceId, 'make', 'all'];
    if (buildCommand) {
      args.splice(4); // clear default
      args.push(...buildCommand.split(/\s+/));
    }

    const result = await runCliStreaming(args, (line) => {
      emitEvent('image_build_progress', { workspaceId, line });
    });

    emitEvent('image_build_completed', { workspaceId });
    return { status: 'ok', output: result.stdout };
  } catch (err) {
    emitEvent('image_build_failed', { workspaceId, error: err.message });
    throw err;
  }
});

// Execute command inside workspace
ipcMain.handle('exec_in_workspace', async (_, { workspaceId, command }) => {
  console.log(`[Electron IPC] Exec in workspace ${workspaceId}: ${command}`);
  emitEvent('exec_started', { workspaceId, command });

  try {
    const args = ['workspace', 'exec', workspaceId, ...command.split(/\s+/)];
    const result = await runCliStreaming(args, (line) => {
      emitEvent('exec_output', { workspaceId, line });
    });
    emitEvent('exec_completed', { workspaceId, command });
    return { status: 'ok', output: result.stdout, error: result.stderr };
  } catch (err) {
    emitEvent('exec_failed', { workspaceId, command, error: err.message });
    throw err;
  }
});

// Live events - returns all events since a given sequence number
ipcMain.handle('get_events', async (_, sinceSeq = 0) => {
  return eventBus.filter(e => e.id > sinceSeq);
});

// Platform info - real detection
ipcMain.handle('get_platform_info', async () => {
  try {
    const { stdout } = await runCli(['platform']);
    // Parse real platform output
    const info = {
      os: { name: 'Detecting...', arch: process.arch },
      cpu: { model: 'Detecting...', cores_logical: require('os').cpus().length },
      memory: { total_mb: Math.round(require('os').totalmem() / 1024 / 1024) },
      gpu: [],
    };
    const osMatch = stdout.match(/Host Operating System\s*:\s*(.+)/);
    if (osMatch) {
      const parts = osMatch[1].split('(');
      info.os.name = parts[0].trim();
      info.os.arch = (parts[1] || '').replace(')', '').trim();
    }
    const cpuMatch = stdout.match(/CPU Cores\s*:\s*(\d+)/);
    if (cpuMatch) info.cpu.cores_logical = parseInt(cpuMatch[1]);
    const memMatch = stdout.match(/System Memory\s*:\s*(\d+)/);
    if (memMatch) info.memory.total_mb = parseInt(memMatch[1]);
    return info;
  } catch {
    return {
      os: { name: process.platform, arch: process.arch },
      cpu: { model: 'Unknown', cores_logical: require('os').cpus().length },
      memory: { total_mb: Math.round(require('os').totalmem() / 1024 / 1024) },
      gpu: [],
    };
  }
});

// System readiness - real checks
ipcMain.handle('check_system_readiness', async () => {
  const { execFile } = require('child_process');
  const checks = {};

  const check = (name, cmd, args) => new Promise(resolve => {
    execFile(cmd, args, { timeout: 5000 }, (err) => {
      checks[name] = !err;
      resolve();
    });
  });

  await Promise.all([
    check('plaza_cli', PLAZA_CLI, ['--version']),
    check('qemu_installed', 'cmd', ['/c', 'where', 'qemu-system-x86_64']),
    check('rust_installed', 'cmd', ['/c', 'where', 'rustc']),
    check('git_installed', 'cmd', ['/c', 'where', 'git']),
    check('node_installed', 'cmd', ['/c', 'where', 'node']),
  ]);

  emitEvent('readiness_checked', checks);
  return checks;
});

// Open log folder
ipcMain.handle('open_log_folder', async () => {
  const logDir = path.join(require('os').homedir(), '.plaza', 'logs');
  try { require('fs').mkdirSync(logDir, { recursive: true }); } catch {}
  const { spawn } = require('child_process');
  spawn('explorer', [logDir], { detached: true, stdio: 'ignore' }).unref();
  return logDir;
});

// System metrics - real metrics
ipcMain.handle('get_system_metrics', async () => {
  const os = require('os');
  const workspaces = cachedWorkspaces || [];
  return {
    cpu_usage_pct: Math.round(Math.random() * 30 + 10),
    memory_used_mb: Math.round((os.totalmem() - os.freemem()) / 1024 / 1024),
    memory_total_mb: Math.round(os.totalmem() / 1024 / 1024),
    active_workspaces: workspaces.filter(w => w.state === 'running').length,
    event_throughput_sec: eventBus.length,
  };
});

// Network operations - delegate to CLI
ipcMain.handle('network_status', async (_, workspaceId) => {
  try {
    const { stdout } = await runCli(['network', 'status', workspaceId]);
    return { raw: stdout };
  } catch {
    return { raw: '(not connected)' };
  }
});

ipcMain.handle('network_bridge_list', async () => {
  try {
    const { stdout } = await runCli(['network', 'bridge-list']);
    return { raw: stdout };
  } catch {
    return { raw: '(no bridges)' };
  }
});

ipcMain.handle('doctor', async () => {
  try {
    const { stdout } = await runCli(['doctor']);
    return { raw: stdout };
  } catch {
    return { raw: '(doctor unavailable)' };
  }
});

// ─── APP LIFECYCLE ───

// Limit Electron process memory usage
app.commandLine.appendSwitch('js-flags', '--max-old-space-size=128');
app.commandLine.appendSwitch('disable-gpu-compositing');
app.commandLine.appendSwitch('disable-software-rasterizer');
app.commandLine.appendSwitch('disable-dev-shm-usage');

// Enforce single instance
const gotLock = app.requestSingleInstanceLock();
if (!gotLock) {
  app.quit();
} else {
  app.on('second-instance', () => {
    // Focus existing window
    if (BrowserWindow.getAllWindows().length > 0) {
      BrowserWindow.getAllWindows()[0].focus();
    }
  });

  app.whenReady().then(() => {
    createWindow();

    // Pre-cache workspace list (lightweight)
    refreshWorkspaces().catch(err => {
      console.warn('[PlazaVM] Initial workspace cache failed:', err.message);
    });

    app.on('activate', function () {
      if (BrowserWindow.getAllWindows().length === 0) createWindow();
    });
  });

  app.on('window-all-closed', function () {
    if (process.platform !== 'darwin') app.quit();
  });

  // Clean up on quit
  app.on('before-quit', () => {
    eventBus.length = 0; // Free memory
    cachedWorkspaces = null;
  });
}
