import { Router, Request, Response } from 'express';
import { FileCache } from './fileCache';
import { UndoRedoManager } from './undoRedo';
import { WebhookManager } from './webhookUtils';
import { SnapshotManager } from './snapshotManager';

export function createApiRouter(
    fileCache: FileCache, 
    undoRedoManager: UndoRedoManager, 
    webhookManager: WebhookManager,
    snapshotManager: SnapshotManager
): Router {
    const router = Router();

    // ==================== File Operations ====================

    // Get all files
    router.get('/files', (req: Request, res: Response) => {
        try {
            const files = fileCache.getAll();
            res.json({
                success: true,
                count: files.length,
                files: files
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // Get specific file
    router.get('/files/:path(*)', (req: Request, res: Response) => {
        try {
            const path = req.params.path;
            const file = fileCache.get(path);

            if (!file) {
                return res.status(404).json({
                    success: false,
                    error: 'File not found'
                });
            }

            res.json({
                success: true,
                file: file
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // Update single file
    router.post('/files/update', (req: Request, res: Response) => {
        try {
            const { path, content, source, force } = req.body;
            const updateSource = source || 'unknown';

            if (!path || content === undefined) {
                return res.status(400).json({
                    success: false,
                    error: 'Missing path or content'
                });
            }

            // Check if file already exists
            const existingFile = fileCache.get(path);
            const action = existingFile ? 'update' : 'create';

            // CONFLICT DETECTION
            // If updating from VS Code, but the file was last modified by Roblox recently
            // Skip checks if 'force' is true
            if (!force && updateSource === 'vscode' && existingFile && existingFile.lastSource === 'roblox') {
                const timeDiff = Date.now() - existingFile.lastModified;
                // If modified in the last 5 minutes and we haven't synced it down
                if (timeDiff < 5 * 60 * 1000) {
                    webhookManager.notifyError('Sync Conflict', `Conflict detected on \`${path}\`. Last modified by Roblox.`);
                    return res.status(409).json({
                        success: false,
                        error: 'Conflict: File was recently modified in Roblox Studio.',
                        remoteContent: existingFile.content
                    });
                }
            }

            // Record action for undo/redo
            if (existingFile) {
                undoRedoManager.recordAction(path, existingFile.content, 'update');
            }

            // Update cache
            fileCache.set(path, content, updateSource);

            // Notify Webhook
            webhookManager.notifyChange(path, action);

            res.json({
                success: true,
                action: action,
                path: path
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // Batch update files
    router.post('/files/batch-update', (req: Request, res: Response) => {
        try {
            const { files } = req.body;

            if (!Array.isArray(files)) {
                return res.status(400).json({
                    success: false,
                    error: 'Files must be an array'
                });
            }

            // Record actions and update cache
            const results = files.map(file => {
                try {
                    const existingFile = fileCache.get(file.path);
                    if (existingFile) {
                        undoRedoManager.recordAction(file.path, existingFile.content, 'update');
                    }
                    fileCache.set(file.path, file.content);
                    return { path: file.path, success: true };
                } catch (error: any) {
                    return { path: file.path, success: false, error: error.message };
                }
            });

            const successCount = results.filter(r => r.success).length;
            const failCount = results.filter(r => !r.success).length;

            if (successCount > 0) {
                webhookManager.send('📦 Batch Update', `Synced ${successCount} files (${failCount} failed)`, 0x9b59b6);
            }

            res.json({
                success: true,
                total: files.length,
                succeeded: successCount,
                failed: failCount,
                results: results
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // Delete file
    router.delete('/files/:path(*)', (req: Request, res: Response) => {
        try {
            const path = req.params.path;
            const file = fileCache.get(path);

            if (!file) {
                return res.status(404).json({
                    success: false,
                    error: 'File not found'
                });
            }

            // Record action for undo
            undoRedoManager.recordAction(path, file.content, 'delete');

            // Delete from cache
            fileCache.delete(path);
            
            webhookManager.notifyChange(path, 'delete');

            res.json({
                success: true,
                message: 'File deleted',
                path: path
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // ==================== Undo/Redo Operations ====================

    // Undo last action
    router.post('/undo', (req: Request, res: Response) => {
        try {
            const entry = undoRedoManager.undo();

            if (!entry) {
                return res.status(404).json({
                    success: false,
                    message: 'Nothing to undo'
                });
            }

            // Restore previous state
            if (entry.action !== 'delete') {
                fileCache.set(entry.path, entry.content);
            } else {
                fileCache.delete(entry.path);
            }

            res.json({
                success: true,
                action: 'undo',
                entry: entry
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // Redo last undone action
    router.post('/redo', (req: Request, res: Response) => {
        try {
            const entry = undoRedoManager.redo();

            if (!entry) {
                return res.status(404).json({
                    success: false,
                    message: 'Nothing to redo'
                });
            }

            // Apply redone action
            if (entry.action !== 'delete') {
                fileCache.set(entry.path, entry.content);
            } else {
                fileCache.delete(entry.path);
            }

            res.json({
                success: true,
                action: 'redo',
                entry: entry
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // Get undo/redo history
    router.get('/history', (req: Request, res: Response) => {
        try {
            const limit = parseInt(req.query.limit as string) || 10;

            res.json({
                success: true,
                undoHistory: undoRedoManager.getUndoHistory(limit),
                redoHistory: undoRedoManager.getRedoHistory(limit),
                stats: undoRedoManager.getStats()
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // ==================== Statistics & Info ====================

    // Get cache statistics
    router.get('/stats', (req: Request, res: Response) => {
        try {
            const cacheStats = fileCache.getStats();
            const undoRedoStats = undoRedoManager.getStats();

            res.json({
                success: true,
                cache: cacheStats,
                undoRedo: undoRedoStats
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // Search files
    router.get('/search', (req: Request, res: Response) => {
        try {
            const pattern = req.query.pattern as string;

            if (!pattern) {
                return res.status(400).json({
                    success: false,
                    error: 'Missing pattern parameter'
                });
            }

            const regex = new RegExp(pattern, 'i');
            const results = fileCache.findByPattern(regex);

            res.json({
                success: true,
                pattern: pattern,
                count: results.length,
                results: results
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // Clear cache
    router.post('/clear', (req: Request, res: Response) => {
        try {
            fileCache.clear();
            undoRedoManager.clearHistory();

            res.json({
                success: true,
                message: 'Cache and history cleared'
            });
        } catch (error: any) {
            res.status(500).json({
                success: false,
                error: error.message
            });
        }
    });

    // ==================== Sync Engine Endpoints ====================

    // Roblox polls this endpoint to get pending changes
    router.get('/sync/poll', (req: Request, res: Response) => {
        try {
            const pendingParams = fileCache.getPendingSyncs();
            res.json({
                success: true,
                count: pendingParams.length,
                events: pendingParams
            });
        } catch (error: any) {
            res.status(500).json({ error: error.message });
        }
    });

    // Roblox confirms it received and applied changes
    router.post('/sync/confirm', (req: Request, res: Response) => {
        try {
            const { eventIds } = req.body;
            if (Array.isArray(eventIds)) {
                fileCache.confirmSync(eventIds);
                res.json({ success: true, confirmed: eventIds.length });
            } else {
                res.status(400).json({ error: "Invalid format. 'eventIds' must be an array." });
            }
        } catch (error: any) {
            res.status(500).json({ error: error.message });
        }
    });
    
    // Get version history for a file
    router.get('/files/:path(*)/history', (req: Request, res: Response) => {
        try {
            const path = req.params.path;
            const history = fileCache.getHistory(path);
            res.json({ success: true, history });
        } catch (error: any) {
             res.status(500).json({ error: error.message });
        }
    });

    // ==================== Console / Logging System ====================
    const logBuffer: Array<{ message: string, type: string, timestamp: number }> = [];
    
    // Roblox pushes logs here
    router.post('/logs/push', (req: Request, res: Response) => {
        const { message, type } = req.body;
        if (message) {
            logBuffer.push({ 
                message, 
                type: type || 'Info', 
                timestamp: Date.now() 
            });
            // Keep buffer small
            if (logBuffer.length > 500) logBuffer.shift();
        }
        res.json({ success: true });
    });

    // VS Code polls logs here
    router.get('/logs/poll', (req: Request, res: Response) => {
        const logs = [...logBuffer];
        logBuffer.length = 0; // Clear after reading
        res.json({ logs });
    });

    // ==================== Snapshot System ====================
    
    // Create Snapshot
    router.post('/snapshots/create', (req: Request, res: Response) => {
        try {
            const { description } = req.body;
            const snapshot = snapshotManager.createSnapshot(fileCache, description);
            
            webhookManager.send('📸 Snapshot Created', `ID: \`${snapshot.id}\`\nFiles: ${snapshot.files.length}`, 0x9b59b6);
            
            res.json({ success: true, snapshot });
        } catch (error: any) {
            res.status(500).json({ error: error.message });
        }
    });

    // List Snapshots
    router.get('/snapshots', (req: Request, res: Response) => {
        try {
            const snapshots = snapshotManager.getSnapshots();
            res.json({ success: true, count: snapshots.length, snapshots });
        } catch (error: any) {
            res.status(500).json({ error: error.message });
        }
    });

    // Restore Snapshot
    router.post('/snapshots/restore', (req: Request, res: Response) => {
        try {
            const { id } = req.body;
            const success = snapshotManager.restoreSnapshot(id, fileCache);
            
            if (success) {
                webhookManager.send('⏪ System Rollback', `Restored snapshot: \`${id}\``, 0xe67e22);
                res.json({ success: true, message: 'Restored successfully' });
            } else {
                res.status(404).json({ success: false, error: 'Snapshot not found' });
            }
        } catch (error: any) {
            res.status(500).json({ error: error.message });
        }
    });

    return router;
}
