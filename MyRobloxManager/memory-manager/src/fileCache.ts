interface CachedFile {
    path: string;
    content: string;
    version: number;
    lastModified: number;
    size: number;
    lastSource: 'vscode' | 'roblox' | 'unknown';
}

interface FileVersion {
    content: string;
    timestamp: number;
    version: number;
    source?: 'vscode' | 'roblox' | 'unknown';
}

interface SyncEvent {
    id: string;
    type: 'update' | 'delete';
    path: string;
    content?: string;
    timestamp: number;
    status: 'pending' | 'synced';
    source: 'vscode' | 'roblox' | 'unknown';
}

export class FileCache {
    private cache: Map<string, CachedFile> = new Map();
    private versions: Map<string, FileVersion[]> = new Map();
    private syncQueue: SyncEvent[] = [];
    private maxVersions: number = 50;
    
    constructor() {
        console.log('[FileCache] Initialized with Versioning & Sync Queue');
    }

    set(path: string, content: string, source: 'vscode' | 'roblox' | 'unknown' = 'unknown'): void {
        const current = this.cache.get(path);
        const newVersion = current ? current.version + 1 : 1;
        
        const cachedFile: CachedFile = {
            path,
            content,
            version: newVersion,
            lastModified: Date.now(),
            size: Buffer.byteLength(content, 'utf8'),
            lastSource: source
        };

        this.cache.set(path, cachedFile);
        this.addVersion(path, content, newVersion, source);
        
        // If change comes from VS Code, queue it for Roblox
        // If change comes from Roblox, we assume it's already there (or we might need to sync to other clients)
        // For now, only queue VS Code changes for Roblox polling
        if (source === 'vscode' || source === 'unknown') {
            this.addToSyncQueue('update', path, content, source);
        }
        
        console.log(`[FileCache] Updated: ${path} (v${newVersion}) via ${source}`);
    }

    delete(path: string): boolean {
        if (this.cache.has(path)) {
            this.cache.delete(path);
            this.addToSyncQueue('delete', path);
            console.log(`[FileCache] Deleted: ${path}`);
            return true;
        }
        return false;
    }

    get(path: string): CachedFile | undefined {
        return this.cache.get(path);
    }

    getAll(): CachedFile[] {
        return Array.from(this.cache.values());
    }

    // Version History
    private addVersion(path: string, content: string, version: number, source?: 'vscode' | 'roblox' | 'unknown') {
        if (!this.versions.has(path)) {
            this.versions.set(path, []);
        }
        const history = this.versions.get(path)!;
        history.push({ content, timestamp: Date.now(), version, source });
        
        if (history.length > this.maxVersions) {
            history.shift();
        }
    }

    getHistory(path: string): FileVersion[] {
        return this.versions.get(path) || [];
    }

    // Sync Queue Management
    private addToSyncQueue(type: 'update' | 'delete', path: string, content?: string, source: 'vscode' | 'roblox' | 'unknown' = 'unknown') {
        // Remove pending events for same file to avoid redundant syncs
        this.syncQueue = this.syncQueue.filter(e => e.path !== path || e.status === 'synced');
        
        this.syncQueue.push({
            id: Math.random().toString(36).substr(2, 9),
            type,
            path,
            content,
            timestamp: Date.now(),
            status: 'pending',
            source
        });
    }

    getPendingSyncs(): SyncEvent[] {
        return this.syncQueue.filter(e => e.status === 'pending');
    }

    confirmSync(eventIds: string[]) {
        this.syncQueue = this.syncQueue.filter(e => !eventIds.includes(e.id));
        console.log(`[Sync] Confirmed ${eventIds.length} events`);
    }
    
    clear() {
        this.cache.clear();
        this.versions.clear();
        this.syncQueue = [];
    }
    
    // Find files matching a regex pattern
    findByPattern(pattern: RegExp): CachedFile[] {
        const results: CachedFile[] = [];
        for (const file of this.cache.values()) {
            if (pattern.test(file.path)) {
                results.push(file);
            }
        }
        return results;
    }

    getStats() {
        return {
            files: this.cache.size,
            pendingSyncs: this.syncQueue.length,
            totalVersions: Array.from(this.versions.values()).reduce((acc, v) => acc + v.length, 0)
        };
    }

    getCachedFileCount(): number {
        return this.cache.size;
    }
}
