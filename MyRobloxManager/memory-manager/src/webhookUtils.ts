import axios from 'axios';
import * as dotenv from 'dotenv';

dotenv.config();

export class WebhookManager {
    private webhookUrl: string | undefined;

    constructor() {
        this.webhookUrl = process.env.WEBHOOK_URL;
        if (this.webhookUrl) {
            console.log('[Webhook] Initialized with URL');
        } else {
            console.log('[Webhook] No URL configured (WEBHOOK_URL is missing)');
        }
    }

    async send(title: string, description: string, color: number = 0x00ff00) {
        if (!this.webhookUrl) return;

        try {
            await axios.post(this.webhookUrl, {
                embeds: [{
                    title: title,
                    description: description,
                    color: color,
                    timestamp: new Date().toISOString(),
                    footer: {
                        text: "Roblox Project Manager Hub"
                    }
                }]
            });
        } catch (error) {
            console.error('[Webhook] Failed to send:', error);
        }
    }

    async notifyChange(path: string, action: 'update' | 'delete' | 'create') {
        let title = '📝 File Updated';
        let color = 0x3498db; // Blue

        if (action === 'create') {
            title = '✨ File Created';
            color = 0x4cd137; // Green
        } else if (action === 'delete') {
            title = '🗑️ File Deleted';
            color = 0xe74c3c; // Red
        }

        await this.send(title, `\`${path}\``, color);
    }

    async notifyError(source: string, error: string) {
        await this.send('🚨 Error Detected', `**Source:** ${source}\n**Error:** ${error}`, 0xff0000);
    }
}
