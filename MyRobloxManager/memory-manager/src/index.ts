import express from 'express';
import cors from 'cors';
import { FileCache } from './fileCache';
import { UndoRedoManager } from './undoRedo';
import { createApiRouter } from './api';
import * as dotenv from 'dotenv';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(express.json({ limit: '50mb' }));
app.use(express.urlencoded({ extended: true, limit: '50mb' }));

import { WebhookManager } from './webhookUtils';
import { SnapshotManager } from './snapshotManager';

// Initialize managers
const fileCache = new FileCache();
const undoRedoManager = new UndoRedoManager();
const webhookManager = new WebhookManager();
const snapshotManager = new SnapshotManager();

// Logging middleware
app.use((req, res, next) => {
    console.log(`[${new Date().toISOString()}] ${req.method} ${req.path}`);
    next();
});

// Health check endpoint
app.get('/health', (req, res) => {
    res.json({
        status: 'ok',
        service: 'Roblox Memory Manager',
        version: '0.1.0',
        uptime: process.uptime(),
        memory: process.memoryUsage(),
        cachedFiles: fileCache.getCachedFileCount()
    });
});

// API router
app.use('/api', createApiRouter(fileCache, undoRedoManager, webhookManager, snapshotManager));

// Error handling middleware
app.use((err: any, req: express.Request, res: express.Response, next: express.NextFunction) => {
    console.error('Error:', err);
    webhookManager.notifyError('Global Middleware', err.message);
    
    res.status(500).json({
        error: 'Internal server error',
        message: err.message
    });
});

// Start server
app.listen(PORT, () => {
    console.log(`🚀 Memory Manager running on http://localhost:${PORT}`);
    console.log(`📊 Health check: http://localhost:${PORT}/health`);
    console.log(`📁 File cache initialized`);
    console.log(`↩️  Undo/Redo manager ready`);
});

// Graceful shutdown
process.on('SIGTERM', () => {
    console.log('SIGTERM received, shutting down gracefully...');
    process.exit(0);
});

process.on('SIGINT', () => {
    console.log('\nSIGINT received, shutting down gracefully...');
    process.exit(0);
});

export { app, fileCache, undoRedoManager };
