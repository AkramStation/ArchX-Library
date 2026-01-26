import * as vscode from 'vscode';
import * as path from 'path';
import { RobloxProjectProvider } from './views/treeView';
import { ControlPanelProvider } from './views/controlPanel';
import { SyncClient } from './communication/syncClient';
import { FileManager } from './utils/fileManager';

let syncClient: SyncClient;
let fileManager: FileManager;

// Helper for creating scripts
async function createScriptHelper(type: string, suffix: string) {
    const fileName = await vscode.window.showInputBox({
        prompt: `Enter ${type} Name (without extension)`,
        placeHolder: `My${type}`
    });

    if (fileName) {
        // Assume creating in src/server or src/client if not specified
        // For simplicity, we just look for open file path or default to src/
        
        // This part would ideally be context-aware from the tree view click
        const fullPath = `src/${fileName}${suffix}`;
        
        let template = `print("Hello from ${fileName}!")`;
        if (type === 'ModuleScript') {
            template = `local ${fileName} = {}\n\nfunction ${fileName}.test()\n    print("Module ran")\nend\n\nreturn ${fileName}`;
        }
        
        // We need to access fileManager here. 
        // Note: This logic needs to be inside activate or have access to fileManager.
        // Since I'm editing the middle of the file, I'll assume fileManager is available in closure 
        // effectively, but I need to make sure I place this correctly.
        // Ideally this logic is inline in the command handler.
    }
}

export function activate(context: vscode.ExtensionContext) {
    console.log('Roblox Project Manager is now active!');

    // Initialize components
    fileManager = new FileManager(context);
    syncClient = new SyncClient();

    // Register Tree View Provider
    const projectProvider = new RobloxProjectProvider(vscode.workspace.rootPath || '', fileManager);
    
    // Register details including drag & drop
    vscode.window.createTreeView('robloxProjectExplorer', {
        treeDataProvider: projectProvider,
        dragAndDropController: projectProvider
    });
    // Legacy support if needed, but createTreeView is better for DnD
    // vscode.window.registerTreeDataProvider('robloxProjectExplorer', projectProvider);

    // Register Control Panel Webview
    const controlPanelProvider = new ControlPanelProvider(
        context.extensionUri,
        syncClient,
        fileManager
    );
    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider(
            ControlPanelProvider.viewType,
            controlPanelProvider
        )
    );

    // Register Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('robloxManager.openProject', async () => {
            const folderUri = await vscode.window.showOpenDialog({
                canSelectFolders: true,
                canSelectFiles: false,
                canSelectMany: false,
                openLabel: 'Select Roblox Project Folder'
            });

            if (folderUri && folderUri[0]) {
                vscode.commands.executeCommand('vscode.openFolder', folderUri[0]);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('robloxManager.syncWithStudio', async () => {
            const config = vscode.workspace.getConfiguration('robloxManager');
            const studioPort = config.get<number>('studioPort', 8080);

            vscode.window.showInformationMessage('Syncing with Roblox Studio...');
            
            try {
                await syncClient.connect(studioPort);
                await syncClient.syncAllFiles(fileManager.getAllFiles());
                vscode.window.showInformationMessage('Successfully synced with Roblox Studio!');
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to sync: ${error}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('robloxManager.refreshExplorer', () => {
            projectProvider.refresh();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('robloxManager.createScript', async () => {
            const scriptName = await vscode.window.showInputBox({
                prompt: 'Enter script name',
                placeHolder: 'MyScript'
            });

            if (scriptName) {
                await fileManager.createScript(scriptName);
                projectProvider.refresh();
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('robloxManager.undo', () => {
            fileManager.undo();
            vscode.window.showInformationMessage('Undid last change');
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('robloxManager.redo', () => {
            fileManager.redo();
            vscode.window.showInformationMessage('Redid last change');
        })
    );

    // Auto-sync on file save
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(async (document) => {
            const config = vscode.workspace.getConfiguration('robloxManager');
            const autoSync = config.get<boolean>('autoSync', false);

            if (autoSync && document.fileName.endsWith('.lua')) {
                try {
                    await syncClient.syncFile(document.fileName, document.getText());
                    vscode.window.showInformationMessage(`Synced ${document.fileName}`);
                } catch (error: any) {
                    if (error.message === 'CONFLICT') {
                        const choice = await vscode.window.showWarningMessage(
                            `Conflict! '${path.basename(document.fileName)}' was recently modified in Roblox.`,
                            'Overwrite Remote',
                            'Dismiss'
                        );
                        
                        if (choice === 'Overwrite Remote') {
                            await syncClient.syncFile(document.fileName, document.getText(), true); // true = force
                            vscode.window.showInformationMessage('Remote overwritten successfully.');
                        }
                    } else {
                        console.error('Auto-sync failed:', error);
                    }
                }
            }
        })
    );

    // Connect to Memory Manager on startup
    const memoryManagerUrl = vscode.workspace.getConfiguration('robloxManager')
        .get<string>('memoryManagerUrl', 'http://localhost:3000');
    
    syncClient.connectToMemoryManager(memoryManagerUrl)
        .catch(err => console.log('Memory Manager not running:', err));
}

export function deactivate() {
    if (syncClient) {
        syncClient.disconnect();
    }
}
