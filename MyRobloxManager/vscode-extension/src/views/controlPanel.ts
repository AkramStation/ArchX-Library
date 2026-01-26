import * as vscode from 'vscode';
import { SyncClient } from '../communication/syncClient';
import { FileManager } from '../utils/fileManager';

export class ControlPanelProvider implements vscode.WebviewViewProvider {
    public static readonly viewType = 'robloxManager.controlPanel';
    private _view?: vscode.WebviewView;
    private syncClient: SyncClient;
    private fileManager: FileManager;
    private isConnected: boolean = false;

    constructor(
        private readonly _extensionUri: vscode.Uri,
        syncClient: SyncClient,
        fileManager: FileManager
    ) {
        this.syncClient = syncClient;
        this.fileManager = fileManager;
    }

    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        context: vscode.WebviewViewResolveContext,
        _token: vscode.CancellationToken,
    ) {
        this._view = webviewView;

        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [this._extensionUri]
        };

        webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

        // Handle messages from webview
        webviewView.webview.onDidReceiveMessage(async (data) => {
            switch (data.type) {
                case 'connect':
                    await this.handleConnect(data.port);
                    break;
                case 'disconnect':
                    await this.handleDisconnect();
                    break;
                case 'sync':
                    await this.handleSync();
                    break;
                case 'import':
                    await this.handleImport();
                    break;
                case 'undo':
                    this.fileManager.undo();
                    this.updateStatus('Undid last change');
                    break;
                case 'redo':
                    this.fileManager.redo();
                    this.updateStatus('Redid last change');
                    break;
                case 'refresh':
                    await this.updateConnectionStatus();
                    break;
            }
        });

        // Update status on load
        this.updateConnectionStatus();
    }

    private async handleConnect(port: number) {
        try {
            this.updateStatus('Connecting to Memory Manager...', 'loading');
            
            // In the new architecture, we connect to the Memory Manager (HTTP), not directly to Studio via WS
            // The port argument is legacy from the WS architecture, strictly speaking we rely on the config URL
            // but we can preserve the interface for now.
            
            await this.syncClient.connectToMemoryManager('http://localhost:3000');
            
            this.isConnected = true;
            this.updateStatus('Connected to Memory Hub', 'success');
            
            // Get stats immediately
            await this.updateConnectionStatus();
            
            this._view?.webview.postMessage({ type: 'connected', port });
        } catch (error: any) {
            this.isConnected = false;
            this.updateStatus(`Connection failed: Is Memory Manager running?`, 'error');
            this._view?.webview.postMessage({ type: 'disconnected' });
        }
    }

    private async handleDisconnect() {
        // Just reset local state since there is no persistent connection to close
        this.isConnected = false;
        this.updateStatus('Disconnected', 'info');
        this._view?.webview.postMessage({ type: 'disconnected' });
    }

    private async handleSync() {
        if (!this.isConnected) {
            this.updateStatus('Not connected to Roblox Studio', 'error');
            return;
        }

        try {
            this.updateStatus('Syncing files...', 'loading');
            const files = this.fileManager.getAllFiles();
            await this.syncClient.syncAllFiles(files);
            this.updateStatus(`Synced ${files.length} files successfully!`, 'success');
            this._view?.webview.postMessage({ 
                type: 'syncComplete', 
                fileCount: files.length 
            });
        } catch (error: any) {
            this.updateStatus(`Sync failed: ${error.message}`, 'error');
        }
    }

    private async handleImport() {
        if (!this.isConnected) return;
        
        try {
            this.updateStatus('Importing from Hub...', 'loading');
            const files = await this.syncClient.fetchAllFiles();
            
            if (files.length === 0) {
                this.updateStatus('No files found in Hub to import.', 'info');
                return;
            }
            
            let count = 0;
            for (const file of files) {
                try {
                    // path from Hub (e.g., "src/server/script.lua")
                    await this.fileManager.createFile(file.path, file.content);
                    count++;
                } catch (e) {
                    console.error(`Failed to write ${file.path}`, e);
                }
            }
            
            this.updateStatus(`Imported ${count} files to disk!`, 'success');
             // Refresh explorer if needed
             vscode.commands.executeCommand('robloxManager.refreshExplorer');
        } catch (error: any) {
            this.updateStatus(`Import failed: ${error.message}`, 'error');
        }
    }

    private updateStatus(message: string, type: string = 'info') {
        this._view?.webview.postMessage({ 
            type: 'status', 
            message, 
            statusType: type 
        });
        
        // Also show in VS Code status bar
        if (type === 'error') {
            vscode.window.showErrorMessage(message);
        } else if (type === 'success') {
            vscode.window.showInformationMessage(message);
        }
    }

    private async updateConnectionStatus() {
        const config = vscode.workspace.getConfiguration('robloxManager');
        const memoryManagerUrl = config.get<string>('memoryManagerUrl', 'http://localhost:3000');
        const fileCount = this.fileManager.getAllFiles().length;

        this._view?.webview.postMessage({
            type: 'stats',
            connected: this.isConnected,
            fileCount: fileCount,
            memoryManagerUrl: memoryManagerUrl
        });
    }

    private _getHtmlForWebview(webview: vscode.Webview) {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RoWinget Control</title>
    <style>
        :root {
            --primary: #00A8FF;
            --secondary: #9C88FF;
            --bg-dark: #1E272E;
            --bg-panel: #2F3640;
            --text-main: #F5F6FA;
            --text-dim: #DCDDE1;
            --success: #4CD137;
            --error: #E84118;
        }

        body {
            padding: 20px;
            color: var(--text-main);
            font-family: 'Segoe UI', user-select, sans-serif;
            font-size: 13px;
            background: linear-gradient(135deg, var(--bg-dark) 0%, #121518 100%);
            height: 100vh;
            box-sizing: border-box;
        }
        
        .header {
            margin-bottom: 24px;
            padding-bottom: 16px;
            border-bottom: 1px solid rgba(255,255,255,0.1);
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        
        .header h2 {
            margin: 0;
            font-weight: 700;
            letter-spacing: 0.5px;
            background: linear-gradient(90deg, var(--primary), var(--secondary));
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        
        .status {
            padding: 6px 12px;
            border-radius: 12px;
            font-size: 11px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            backdrop-filter: blur(4px);
        }
        
        .status.info {
            background: rgba(47, 54, 64, 0.6);
            border: 1px solid rgba(255,255,255,0.1);
            color: var(--text-dim);
        }
        
        .status.success {
            background: rgba(76, 209, 55, 0.2);
            border: 1px solid rgba(76, 209, 55, 0.4);
            color: var(--success);
            box-shadow: 0 0 10px rgba(76, 209, 55, 0.1);
        }
        
        .status.error {
            background: rgba(232, 65, 24, 0.2);
            border: 1px solid rgba(232, 65, 24, 0.4);
            color: var(--error);
        }
        
        .status.loading {
            background: rgba(156, 136, 255, 0.2);
            border: 1px solid rgba(156, 136, 255, 0.4);
            color: var(--secondary);
        }
        
        h3 {
            font-size: 11px;
            text-transform: uppercase;
            color: var(--text-dim);
            margin-bottom: 12px;
            opacity: 0.8;
        }

        .input-group input {
            width: 100%;
            padding: 10px;
            background: rgba(0,0,0,0.2);
            border: 1px solid rgba(255,255,255,0.1);
            color: var(--text-main);
            border-radius: 6px;
            margin-bottom: 12px;
            transition: all 0.3s ease;
        }
        
        .input-group input:focus {
            border-color: var(--primary);
            box-shadow: 0 0 0 2px rgba(0, 168, 255, 0.2);
            outline: none;
        }
        
        button {
            background: linear-gradient(90deg, var(--primary), #0097e6);
            color: white;
            border: none;
            padding: 10px 16px;
            border-radius: 6px;
            cursor: pointer;
            font-weight: 600;
            width: 100%;
            margin-bottom: 10px;
            transition: transform 0.1s, box-shadow 0.2s;
            text-shadow: 0 1px 2px rgba(0,0,0,0.2);
        }
        
        button:hover {
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(0, 168, 255, 0.3);
        }
        
        button:active {
            transform: translateY(0);
        }
        
        button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            transform: none;
            box-shadow: none;
            background: var(--bg-panel);
        }
        
        button.secondary {
            background: transparent;
            border: 1px solid rgba(255,255,255,0.2);
            color: var(--text-dim);
        }
        
        button.secondary:hover {
            border-color: var(--text-main);
            color: var(--text-main);
            background: rgba(255,255,255,0.05);
            box-shadow: none;
        }
        
        .stats {
            background: rgba(47, 54, 64, 0.4);
            border-radius: 8px;
            padding: 15px;
            border: 1px solid rgba(255,255,255,0.05);
            backdrop-filter: blur(10px);
        }
        
        .stat-item {
            display: flex;
            justify-content: space-between;
            margin-bottom: 6px;
            font-size: 12px;
        }
        
        .stat-value {
            color: var(--primary);
            font-family: 'Consolas', monospace;
        }

        /* Animations */
        @keyframes pulse {
            0% { box-shadow: 0 0 0 0 rgba(76, 209, 55, 0.4); }
            70% { box-shadow: 0 0 0 6px rgba(76, 209, 55, 0); }
            100% { box-shadow: 0 0 0 0 rgba(76, 209, 55, 0); }
        }
        
        .connection-indicator.connected {
            animation: pulse 2s infinite;
        }
        
        .actions-section { margin-top: 30px; }
        
        .spinner {
            border: 2px solid rgba(255,255,255,0.1);
            border-top-color: var(--primary);
        }
    </style>
</head>
<body>
    <div class="header">
        <h2>RoWinget</h2>
        <div id="connectionStatus" class="status info">
            <span class="connection-indicator disconnected"></span>
            Offline
        </div>
    </div>
    
    <div class="connection-section">
        <h3>Connection</h3>
        <div class="input-group">
            <label for="portInput">Roblox Studio Port:</label>
            <input type="number" id="portInput" value="8080" min="1" max="65535">
        </div>
        <button id="connectBtn" onclick="connect()">Connect to Roblox Studio</button>
        <button id="disconnectBtn" onclick="disconnect()" class="secondary" style="display: none;">Disconnect</button>
    </div>
    
    <div class="actions-section">
        <h3>Actions</h3>
        <button id="syncBtn" onclick="syncFiles()" disabled>🔄 Sync All Files (Push)</button>
        <button id="importBtn" onclick="importFiles()" class="secondary" disabled>⬇️ Import from Roblox</button>
        <button onclick="undo()">↩️ Undo</button>
        <button onclick="redo()">↪️ Redo</button>
        <button onclick="refresh()" class="secondary">🔃 Refresh Status</button>
    </div>
    
    <div class="stats">
        <h3>Statistics</h3>
        <div class="stat-item">
            <span class="stat-label">Cached Files:</span>
            <span class="stat-value" id="fileCount">0</span>
        </div>
        <div class="stat-item">
            <span class="stat-label">Memory Manager:</span>
            <span class="stat-value" id="memoryManager">Not Connected</span>
        </div>
        <div class="stat-item">
            <span class="stat-label">Status:</span>
            <span class="stat-value" id="statusText">Ready</span>
        </div>
    </div>
    
    <script>
        const vscode = acquireVsCodeApi();
        let isConnected = false;
        
        function connect() {
            const port = parseInt(document.getElementById('portInput').value);
            vscode.postMessage({ type: 'connect', port: port });
        }
        
        function disconnect() {
            vscode.postMessage({ type: 'disconnect' });
        }
        
        function syncFiles() {
            vscode.postMessage({ type: 'sync' });
        }
        
        function importFiles() {
            vscode.postMessage({ type: 'import' });
        }
        
        function undo() {
            vscode.postMessage({ type: 'undo' });
        }
        
        function redo() {
            vscode.postMessage({ type: 'redo' });
        }
        
        function refresh() {
            vscode.postMessage({ type: 'refresh' });
        }
        
        window.addEventListener('message', event => {
            const message = event.data;
            // Buffer logs for debugging
            console.log('[WebView] Received:', message.type, message);
            
            switch (message.type) {
                case 'connected':
                    isConnected = true;
                    // Force UI update
                    document.getElementById('connectBtn').style.display = 'none';
                    document.getElementById('disconnectBtn').style.display = 'block';
                    document.getElementById('syncBtn').disabled = false;
                    document.getElementById('importBtn').disabled = false;
                    
                    updateConnectionStatus(true, 'Connected to Port ' + (message.port || 3000));
                    updateStatus('Connected to Memory Hub', 'success');
                    break;
                    
                case 'disconnected':
                    isConnected = false;
                    document.getElementById('connectBtn').style.display = 'block';
                    document.getElementById('disconnectBtn').style.display = 'none';
                    document.getElementById('syncBtn').disabled = true;
                    document.getElementById('importBtn').disabled = true;
                    
                    updateConnectionStatus(false, 'Not Connected');
                    break;
                    
                case 'status':
                    updateStatus(message.message, message.statusType);
                    break;
                    
                case 'stats':
                    document.getElementById('fileCount').textContent = message.fileCount;
                    document.getElementById('memoryManager').textContent = message.memoryManagerUrl;
                    
                    // Check if we need to sync connected state from backend
                    if (message.connected && !isConnected) {
                        // Backend thinks we are connected, but UI doesn't. Fixing...
                        isConnected = true;
                        document.getElementById('connectBtn').style.display = 'none';
                        document.getElementById('disconnectBtn').style.display = 'block';
                        document.getElementById('syncBtn').disabled = false;
                        document.getElementById('importBtn').disabled = false;
                        updateConnectionStatus(true, 'Connected');
                    }
                    break;
                    
                case 'syncComplete':
                    updateStatus('Synced ' + message.fileCount + ' files!', 'success');
                    break;
            }
        });
        
        function updateConnectionStatus(connected, text) {
            const statusDiv = document.getElementById('connectionStatus');
            const indicator = statusDiv.querySelector('.connection-indicator');
            
            if (connected) {
                indicator.className = 'connection-indicator connected';
                statusDiv.className = 'status success';
            } else {
                indicator.className = 'connection-indicator disconnected';
                statusDiv.className = 'status info';
            }
            
            statusDiv.innerHTML = '<span class="' + indicator.className + '"></span>' + text;
        }
        
        function updateStatus(message, type) {
            document.getElementById('statusText').textContent = message;
            const statusDiv = document.getElementById('connectionStatus');
            
            if (type === 'loading') {
                statusDiv.innerHTML = '<span class="spinner"></span>' + message;
                statusDiv.className = 'status loading';
            }
        }
    </script>
</body>
</html>`;
    }
}
