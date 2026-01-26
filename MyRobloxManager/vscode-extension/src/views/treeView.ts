import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';

import { FileManager } from '../utils/fileManager';

export class RobloxProjectProvider implements vscode.TreeDataProvider<ProjectItem>, vscode.TreeDragAndDropController<ProjectItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<ProjectItem | undefined | null | void> = 
        new vscode.EventEmitter<ProjectItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<ProjectItem | undefined | null | void> = 
        this._onDidChangeTreeData.event;

    // Drag & Drop
    dropMimeTypes = ['application/vnd.code.tree.robloxProjectExplorer'];
    dragMimeTypes = ['text/uri-list'];

    constructor(private workspaceRoot: string, private fileManager: FileManager) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    // Drag
    public handleDrag(source: readonly ProjectItem[], dataTransfer: vscode.DataTransfer, token: vscode.CancellationToken): void | Thenable<void> {
        dataTransfer.set('application/vnd.code.tree.robloxProjectExplorer', new vscode.DataTransferItem(source));
    }

    // Drop
    public async handleDrop(target: ProjectItem | undefined, dataTransfer: vscode.DataTransfer, token: vscode.CancellationToken): Promise<void> {
        const transferItem = dataTransfer.get('application/vnd.code.tree.robloxProjectExplorer');
        if (!transferItem) return;

        const sources: ProjectItem[] = transferItem.value;
        const source = sources[0];
        
        if (!source || !target) return;

        // Prevent dropping on self or file (must drop on folder or root)
        let targetPath = target.resourceUri.fsPath;
        if (target.itemType !== 'folder') {
            targetPath = path.dirname(targetPath);
        }

        try {
            await this.fileManager.moveFile(source.resourceUri.fsPath, targetPath);
            this.refresh();
        } catch (e: any) {
            vscode.window.showErrorMessage(`Move failed: ${e.message}`);
        }
    }

    getTreeItem(element: ProjectItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: ProjectItem): Thenable<ProjectItem[]> {
        if (!this.workspaceRoot) {
            vscode.window.showInformationMessage('No Roblox project opened');
            return Promise.resolve([]);
        }

        if (element) {
            return Promise.resolve(this.getFilesInDirectory(element.resourceUri.fsPath));
        } else {
            return Promise.resolve(this.getFilesInDirectory(this.workspaceRoot));
        }
    }

    private getFilesInDirectory(dirPath: string): ProjectItem[] {
        if (!fs.existsSync(dirPath)) {
            return [];
        }

        const items: ProjectItem[] = [];
        const files = fs.readdirSync(dirPath);

        files.forEach(file => {
            const filePath = path.join(dirPath, file);
            const stat = fs.statSync(filePath);

            if (stat.isDirectory()) {
                items.push(new ProjectItem(
                    file,
                    vscode.Uri.file(filePath),
                    vscode.TreeItemCollapsibleState.Collapsed,
                    'folder'
                ));
            } else if (file.endsWith('.lua')) {
                items.push(new ProjectItem(
                    file,
                    vscode.Uri.file(filePath),
                    vscode.TreeItemCollapsibleState.None,
                    'script'
                ));
            } else {
                items.push(new ProjectItem(
                    file,
                    vscode.Uri.file(filePath),
                    vscode.TreeItemCollapsibleState.None,
                    'file'
                ));
            }
        });

        return items;
    }
}

class ProjectItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly resourceUri: vscode.Uri,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly itemType: 'folder' | 'script' | 'file'
    ) {
        super(label, collapsibleState);

        this.tooltip = this.resourceUri.fsPath;
        this.description = this.getDescription();

        // Set icons based on type
        if (itemType === 'folder') {
            this.iconPath = new vscode.ThemeIcon('folder');
        } else if (itemType === 'script') {
            this.iconPath = new vscode.ThemeIcon('file-code');
            this.command = {
                command: 'vscode.open',
                title: 'Open File',
                arguments: [this.resourceUri]
            };
        } else {
            this.iconPath = new vscode.ThemeIcon('file');
            this.command = {
                command: 'vscode.open',
                title: 'Open File',
                arguments: [this.resourceUri]
            };
        }
    }

    private getDescription(): string {
        // You can add custom descriptions based on file type
        return '';
    }
}
