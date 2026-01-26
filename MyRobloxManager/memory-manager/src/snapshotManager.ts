import * as fs from 'fs';
import * as path from 'path';
import { FileCache } from './fileCache';

interface Snapshot {
    id: string;
    timestamp: number;
    files: any[];
    description: string;
}

export class SnapshotManager {
    private snapshotsDir: string;
    private snapshots: Map<string, Snapshot> = new Map();

    constructor() {
        this.snapshotsDir = path.join(process.cwd(), 'snapshots');
        if (!fs.existsSync(this.snapshotsDir)) {
            fs.mkdirSync(this.snapshotsDir, { recursive: true });
        }
        this.loadSnapshots();
    }

    private loadSnapshots() {
        try {
            const items = fs.readdirSync(this.snapshotsDir);
            items.forEach(item => {
                if (item.endsWith('.json')) {
                    const content = fs.readFileSync(path.join(this.snapshotsDir, item), 'utf8');
                    const snapshot = JSON.parse(content);
                    this.snapshots.set(snapshot.id, snapshot);
                }
            });
            console.log(`[Snapshots] Loaded ${this.snapshots.size} snapshots`);
        } catch (e) {
            console.error('[Snapshots] Failed to load:', e);
        }
    }

    createSnapshot(fileCache: FileCache, description: string = 'Manual Snapshot'): Snapshot {
        const id = new Date().toISOString().replace(/[:.]/g, '-');
        const snapshot: Snapshot = {
            id,
            timestamp: Date.now(),
            files: fileCache.getAll(),
            description
        };

        const filePath = path.join(this.snapshotsDir, `${id}.json`);
        fs.writeFileSync(filePath, JSON.stringify(snapshot, null, 2));
        
        this.snapshots.set(id, snapshot);
        console.log(`[Snapshots] Created: ${id}`);
        return snapshot;
    }

    getSnapshots(): Snapshot[] {
        return Array.from(this.snapshots.values()).sort((a, b) => b.timestamp - a.timestamp);
    }

    restoreSnapshot(id: string, fileCache: FileCache): boolean {
        const snapshot = this.snapshots.get(id);
        if (!snapshot) return false;

        fileCache.clear();
        snapshot.files.forEach(f => {
            fileCache.set(f.path, f.content);
        });
        
        console.log(`[Snapshots] Restored: ${id}`);
        return true;
    }
}
