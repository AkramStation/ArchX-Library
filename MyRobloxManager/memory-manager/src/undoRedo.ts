interface HistoryEntry {
    path: string;
    content: string;
    timestamp: number;
    action: 'create' | 'update' | 'delete';
}

export class UndoRedoManager {
    private undoStack: HistoryEntry[] = [];
    private redoStack: HistoryEntry[] = [];
    private maxStackSize: number = 100;

    constructor() {
        console.log('[UndoRedoManager] Initialized');
    }

    // Record an action
    recordAction(path: string, content: string, action: 'create' | 'update' | 'delete'): void {
        const entry: HistoryEntry = {
            path,
            content,
            timestamp: Date.now(),
            action
        };

        this.undoStack.push(entry);

        // Limit stack size
        if (this.undoStack.length > this.maxStackSize) {
            this.undoStack.shift();
        }

        // Clear redo stack when new action is recorded
        this.redoStack = [];

        console.log(`[UndoRedoManager] Recorded ${action}: ${path}`);
    }

    // Undo last action
    undo(): HistoryEntry | null {
        const entry = this.undoStack.pop();
        if (!entry) {
            console.log('[UndoRedoManager] Nothing to undo');
            return null;
        }

        this.redoStack.push(entry);
        console.log(`[UndoRedoManager] Undo ${entry.action}: ${entry.path}`);
        return entry;
    }

    // Redo last undone action
    redo(): HistoryEntry | null {
        const entry = this.redoStack.pop();
        if (!entry) {
            console.log('[UndoRedoManager] Nothing to redo');
            return null;
        }

        this.undoStack.push(entry);
        console.log(`[UndoRedoManager] Redo ${entry.action}: ${entry.path}`);
        return entry;
    }

    // Get undo history
    getUndoHistory(limit: number = 10): HistoryEntry[] {
        return this.undoStack.slice(-limit).reverse();
    }

    // Get redo history
    getRedoHistory(limit: number = 10): HistoryEntry[] {
        return this.redoStack.slice(-limit).reverse();
    }

    // Check if undo is available
    canUndo(): boolean {
        return this.undoStack.length > 0;
    }

    // Check if redo is available
    canRedo(): boolean {
        return this.redoStack.length > 0;
    }

    // Clear all history
    clearHistory(): void {
        this.undoStack = [];
        this.redoStack = [];
        console.log('[UndoRedoManager] Cleared all history');
    }

    // Get statistics
    getStats() {
        return {
            undoStackSize: this.undoStack.length,
            redoStackSize: this.redoStack.length,
            maxStackSize: this.maxStackSize,
            canUndo: this.canUndo(),
            canRedo: this.canRedo()
        };
    }

    // Get recent actions
    getRecentActions(limit: number = 20): HistoryEntry[] {
        return [...this.undoStack, ...this.redoStack]
            .sort((a, b) => b.timestamp - a.timestamp)
            .slice(0, limit);
    }

    // Get actions for specific file
    getFileHistory(path: string, limit: number = 10): HistoryEntry[] {
        return this.undoStack
            .filter(entry => entry.path === path)
            .slice(-limit)
            .reverse();
    }
}
