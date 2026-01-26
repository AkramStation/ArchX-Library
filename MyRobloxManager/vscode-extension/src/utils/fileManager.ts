import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';

interface FileState {
    path: string;
    content: string;
    timestamp: number;
}

export class FileManager {
    private undoStack: FileState[] = [];
    private redoStack: FileState[] = [];
    private maxStackSize = 50;

    constructor(private context: vscode.ExtensionContext) {}

    getAllFiles(): Array<{ path: string; content: string }> {
        const workspaceRoot = vscode.workspace.rootPath;
        if (!workspaceRoot) {
            return [];
        }

        const files: Array<{ path: string; content: string }> = [];
        this.collectLuaFiles(workspaceRoot, files);
        return files;
    }

    private collectLuaFiles(dir: string, files: Array<{ path: string; content: string }>): void {
        if (!fs.existsSync(dir)) {
            return;
        }

        const items = fs.readdirSync(dir);
        
        items.forEach(item => {
            const fullPath = path.join(dir, item);
            const stat = fs.statSync(fullPath);

            if (stat.isDirectory()) {
                this.collectLuaFiles(fullPath, files);
            } else if (item.endsWith('.lua')) {
                const content = fs.readFileSync(fullPath, 'utf-8');
                files.push({ path: fullPath, content });
            }
        });
    }

    async createScript(scriptName: string): Promise<void> {
        const workspaceRoot = vscode.workspace.rootPath;
        if (!workspaceRoot) {
            vscode.window.showErrorMessage('No workspace opened');
            return;
        }

        const scriptPath = path.join(workspaceRoot, 'src', `${scriptName}.lua`);
        const scriptDir = path.dirname(scriptPath);

        // Create directory if it doesn't exist
        if (!fs.existsSync(scriptDir)) {
            fs.mkdirSync(scriptDir, { recursive: true });
        }

        // Create script with template
        const template = `-- ${scriptName}.lua
-- Created: ${new Date().toISOString()}

local ${scriptName} = {}

function ${scriptName}.new()
    local self = setmetatable({}, { __index = ${scriptName} })
    return self
end

function ${scriptName}:init()
    print("${scriptName} initialized")
end

return ${scriptName}
`;

        fs.writeFileSync(scriptPath, template, 'utf-8');
        
        // Open the new file
        const document = await vscode.workspace.openTextDocument(scriptPath);
        await vscode.window.showTextDocument(document);

        // Save state for undo
        this.saveState(scriptPath, template);
    }

    private saveState(filePath: string, content: string): void {
        const state: FileState = {
            path: filePath,
            content: content,
            timestamp: Date.now()
        };

        this.undoStack.push(state);
        
        // Limit stack size
        if (this.undoStack.length > this.maxStackSize) {
            this.undoStack.shift();
        }

        // Clear redo stack on new action
        this.redoStack = [];
    }

    undo(): void {
        const state = this.undoStack.pop();
        if (!state) {
            vscode.window.showInformationMessage('Nothing to undo');
            return;
        }

        // Save current state to redo stack
        if (fs.existsSync(state.path)) {
            const currentContent = fs.readFileSync(state.path, 'utf-8');
            this.redoStack.push({
                path: state.path,
                content: currentContent,
                timestamp: Date.now()
            });
        }

        // Restore previous state
        fs.writeFileSync(state.path, state.content, 'utf-8');
    }

    redo(): void {
        const state = this.redoStack.pop();
        if (!state) {
            vscode.window.showInformationMessage('Nothing to redo');
            return;
        }

        // Save current state to undo stack
        if (fs.existsSync(state.path)) {
            const currentContent = fs.readFileSync(state.path, 'utf-8');
            this.undoStack.push({
                path: state.path,
                content: currentContent,
                timestamp: Date.now()
            });
        }

        // Restore redo state
        fs.writeFileSync(state.path, state.content, 'utf-8');
    }

    getFileContent(filePath: string): string | null {
        if (!fs.existsSync(filePath)) {
            return null;
        }
        return fs.readFileSync(filePath, 'utf-8');
    }

    updateFileContent(filePath: string, content: string): void {
        // Save current state before updating
        const currentContent = this.getFileContent(filePath);
        if (currentContent !== null) {
            this.saveState(filePath, currentContent);
        }

        fs.writeFileSync(filePath, content, 'utf-8');
    }
    async createFile(relativePath: string, content: string): Promise<void> {
        const workspaceRoot = vscode.workspace.rootPath;
        if (!workspaceRoot) return;
        
        // Sanitize and construct full path
        const fullPath = path.join(workspaceRoot, relativePath);
        const dir = path.dirname(fullPath);
        
        // Ensure directory exists
        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true });
        }
        
        // Write file
        fs.writeFileSync(fullPath, content, 'utf8');
        
        // Open the document if it's new? Optional.
        // const document = await vscode.workspace.openTextDocument(fullPath);
        // await vscode.window.showTextDocument(document, { preview: false });
    }

    async moveFile(sourcePath: string, targetPath: string): Promise<void> {
        if (!fs.existsSync(sourcePath)) {
            throw new Error(`Source file not found: ${sourcePath}`);
        }
        
        // If target is a directory, append filename
        if (fs.existsSync(targetPath) && fs.statSync(targetPath).isDirectory()) {
            targetPath = path.join(targetPath, path.basename(sourcePath));
        }
        
        const targetDir = path.dirname(targetPath);
        if (!fs.existsSync(targetDir)) {
            fs.mkdirSync(targetDir, { recursive: true });
        }
        
        fs.renameSync(sourcePath, targetPath);
    }
}
