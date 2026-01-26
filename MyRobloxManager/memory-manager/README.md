# Roblox Memory Manager

A Node.js service that provides memory caching, undo/redo functionality, and API endpoints for the Roblox Project Manager.

## Features

- 💾 **File Caching**: Store files in memory for fast access
- ↩️ **Undo/Redo**: Track changes with full history support
- 🌐 **REST API**: HTTP endpoints for file operations
- 📊 **Statistics**: Monitor cache usage and performance
- 🔍 **Search**: Find files using regex patterns
- 🔄 **Batch Operations**: Update multiple files at once

## Installation

### Prerequisites

- Node.js 18.x or higher
- npm or yarn

### Setup

1. Navigate to the memory-manager directory:
   ```bash
   cd MyRobloxManager/memory-manager
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Create environment file:
   ```bash
   cp .env.example .env
   ```

4. Build the TypeScript code:
   ```bash
   npm run build
   ```

## Usage

### Development Mode

```bash
npm run dev
```

### Production Mode

```bash
npm run build
npm start
```

The server will start on `http://localhost:3000` by default.

## API Documentation

### Health Check

```http
GET /health
```

Returns service status and statistics.

### File Operations

#### Get All Files

```http
GET /api/files
```

Response:
```json
{
  "success": true,
  "count": 10,
  "files": [...]
}
```

#### Get Specific File

```http
GET /api/files/:path
```

#### Update File

```http
POST /api/files/update
Content-Type: application/json

{
  "path": "src/player/Controller.lua",
  "content": "-- Lua code here"
}
```

#### Batch Update Files

```http
POST /api/files/batch-update
Content-Type: application/json

{
  "files": [
    { "path": "...", "content": "..." },
    { "path": "...", "content": "..." }
  ]
}
```

#### Delete File

```http
DELETE /api/files/:path
```

### Undo/Redo Operations

#### Undo Last Action

```http
POST /api/undo
```

#### Redo Last Action

```http
POST /api/redo
```

#### Get History

```http
GET /api/history?limit=10
```

### Statistics & Search

#### Get Statistics

```http
GET /api/stats
```

Returns cache and undo/redo statistics.

#### Search Files

```http
GET /api/search?pattern=Controller
```

Search files using regex pattern.

#### Clear Cache

```http
POST /api/clear
```

Clear all cached files and history.

## Architecture

```
memory-manager/
├── src/
│   ├── index.ts       # Express server entry point
│   ├── fileCache.ts   # File caching implementation
│   ├── undoRedo.ts    # Undo/redo manager
│   └── api.ts         # API route handlers
├── dist/              # Compiled JavaScript (generated)
├── package.json
├── tsconfig.json
└── README.md
```

## Integration

### VS Code Extension

The VS Code extension automatically connects to the Memory Manager on startup:

```typescript
const memoryManagerUrl = 'http://localhost:3000';
await syncClient.connectToMemoryManager(memoryManagerUrl);
```

### Roblox Studio Plugin

The Roblox plugin sends messages via HTTP POST:

```lua
HttpService:PostAsync(
    "http://localhost:3000/api/roblox/message",
    message,
    Enum.HttpContentType.ApplicationJson
)
```

## Configuration

Edit `.env` file to configure:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 3000 | Server port |
| `NODE_ENV` | development | Environment mode |

## Performance

- **Max Cache Size**: 1000 files
- **Max File Size**: 10MB per file
- **Undo Stack Size**: 100 actions
- **Request Body Limit**: 50MB

When cache is full, the oldest file is automatically evicted (LRU policy).

## Examples

### Example 1: Update a Single File

```javascript
const axios = require('axios');

const response = await axios.post('http://localhost:3000/api/files/update', {
  path: 'src/player/Movement.lua',
  content: '-- Movement script\nprint("Moving!")'
});

console.log(response.data);
// { success: true, action: 'create', path: 'src/player/Movement.lua' }
```

### Example 2: Batch Update

```javascript
const files = [
  { path: 'src/ui/MainMenu.lua', content: '-- Menu code' },
  { path: 'src/ui/Inventory.lua', content: '-- Inventory code' }
];

const response = await axios.post('http://localhost:3000/api/files/batch-update', {
  files: files
});

console.log(response.data);
// { success: true, total: 2, succeeded: 2, failed: 0 }
```

### Example 3: Undo Last Change

```javascript
const response = await axios.post('http://localhost:3000/api/undo');

console.log(response.data);
// { success: true, action: 'undo', entry: {...} }
```

### Example 4: Get Statistics

```javascript
const response = await axios.get('http://localhost:3000/api/stats');

console.log(response.data);
/*
{
  success: true,
  cache: {
    fileCount: 50,
    totalSize: 1048576,
    maxCacheSize: 1000,
    averageFileSize: 20971
  },
  undoRedo: {
    undoStackSize: 25,
    redoStackSize: 0,
    canUndo: true,
    canRedo: false
  }
}
*/
```

## Troubleshooting

### Server won't start
- Check if port 3000 is already in use
- Try changing the port in `.env` file
- Ensure all dependencies are installed

### Files not caching
- Check file size (max 10MB)
- Verify API endpoint is correct
- Check server logs for errors

### Memory issues
- Reduce `maxCacheSize` in `fileCache.ts`
- Clear cache periodically using `/api/clear`
- Monitor memory usage via `/health` endpoint

## Development

### Watch Mode

```bash
npm run watch
```

Automatically recompiles TypeScript on file changes.

### Testing

```bash
# Test health endpoint
curl http://localhost:3000/health

# Test file update
curl -X POST http://localhost:3000/api/files/update \
  -H "Content-Type: application/json" \
  -d '{"path":"test.lua","content":"print(\"test\")"}'

# Test undo
curl -X POST http://localhost:3000/api/undo
```

## Future Enhancements

- [ ] WebSocket support for real-time updates
- [ ] Persistent storage with SQLite/Redis
- [ ] File compression for large files
- [ ] Authentication and authorization
- [ ] Rate limiting
- [ ] Logging to file

## License

MIT
