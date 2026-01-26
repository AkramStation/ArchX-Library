import WebSocket from 'ws';
import axios from 'axios';

export class SyncClient {
    private ws?: WebSocket;
    private studioUrl: string = '';
    private memoryManagerUrl: string = '';
    private isConnected: boolean = false;

    async connect(port: number): Promise<void> {
        this.studioUrl = `ws://localhost:${port}`;
        
        return new Promise((resolve, reject) => {
            try {
                this.ws = new WebSocket(this.studioUrl);

                if (!this.ws) {
                    reject(new Error('Failed to create WebSocket'));
                    return;
                }

                this.ws.on('open', () => {
                    console.log('Connected to Roblox Studio');
                    this.isConnected = true;
                    resolve();
                });

                this.ws.on('message', (data) => {
                    this.handleMessage(data.toString());
                });

                this.ws.on('error', (error) => {
                    console.error('WebSocket error:', error);
                    this.isConnected = false;
                    reject(error);
                });

                this.ws.on('close', () => {
                    console.log('Disconnected from Roblox Studio');
                    this.isConnected = false;
                });

                // Timeout after 5 seconds if can't connect
                setTimeout(() => {
                    if (!this.isConnected) {
                        reject(new Error('Connection timeout'));
                    }
                }, 5000);
            } catch (error) {
                reject(error);
            }
        });
    }

    async connectToMemoryManager(url: string): Promise<void> {
        this.memoryManagerUrl = url;
        try {
            const response = await axios.get(`${url}/health`);
            if (response.status === 200) {
                console.log('Connected to Memory Manager');
            }
        } catch (error) {
            throw new Error(`Failed to connect to Memory Manager: ${error}`);
        }
    }

    async syncFile(filePath: string, content: string): Promise<void> {
        if (!this.isConnected) {
            throw new Error('Not connected to Roblox Studio');
        }

        const message = {
            type: 'FILE_UPDATE',
            data: {
                path: filePath,
                content: content,
                timestamp: Date.now()
            }
        };

        // Send to Roblox Studio
        this.send(message);

        // Send to Memory Manager
        if (this.memoryManagerUrl) {
            try {
                await axios.post(`${this.memoryManagerUrl}/api/files/update`, {
                    path: filePath,
                    content: content
                });
            } catch (error) {
                console.error('Failed to update Memory Manager:', error);
            }
        }
    }

    async syncAllFiles(files: Array<{ path: string; content: string }>): Promise<void> {
        if (!this.isConnected) {
            throw new Error('Not connected to Roblox Studio');
        }

        // Batch update to Memory Manager
        if (this.memoryManagerUrl) {
            try {
                await axios.post(`${this.memoryManagerUrl}/api/files/batch-update`, {
                    files: files
                });
            } catch (error) {
                console.error('Failed to batch update Memory Manager:', error);
            }
        }
    }

    async fetchAllFiles(): Promise<Array<{ path: string, content: string }>> {
        if (!this.memoryManagerUrl) {
            throw new Error('Memory Manager URL not configured');
        }
        try {
            const response = await axios.get(`${this.memoryManagerUrl}/api/files`);
            // The API returns { success: true, files: [...] }
            if (response.data && Array.isArray(response.data.files)) {
                return response.data.files;
            }
            return [];
        } catch (error) {
            console.error('[Sync] Failed to fetch files:', error);
            throw error;
        }
    }

    async fetchLogs(): Promise<Array<{ message: string, type: string, timestamp: number }>> {
        if (!this.memoryManagerUrl) return [];
        try {
            const response = await axios.get(`${this.memoryManagerUrl}/api/logs/poll`);
            return response.data.logs || [];
        } catch (error) {
            return [];
        }
    }

    async createSnapshot(description: string): Promise<void> {
        await axios.post(`${this.memoryManagerUrl}/api/snapshots/create`, { description });
    }

    async getSnapshots(): Promise<any[]> {
        const res = await axios.get(`${this.memoryManagerUrl}/api/snapshots`);
        return res.data.snapshots || [];
    }

    async restoreSnapshot(id: string): Promise<void> {
        await axios.post(`${this.memoryManagerUrl}/api/snapshots/restore`, { id });
    }

    private send(message: any): void {
        if (this.ws && this.isConnected) {
            this.ws.send(JSON.stringify(message));
        }
    }

    private handleMessage(data: string): void {
        try {
            const message = JSON.parse(data);
            console.log('Received from Roblox Studio:', message);

            // Handle different message types
            switch (message.type) {
                case 'ACK':
                    console.log('File sync acknowledged');
                    break;
                case 'ERROR':
                    console.error('Studio error:', message.data);
                    break;
                default:
                    console.log('Unknown message type:', message.type);
            }
        } catch (error) {
            console.error('Failed to parse message:', error);
        }
    }

    disconnect(): void {
        if (this.ws) {
            this.ws.close();
            this.isConnected = false;
        }
    }
}
