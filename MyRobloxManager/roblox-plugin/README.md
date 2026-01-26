# Roblox Studio Plugin

This plugin enables synchronization between VS Code and Roblox Studio for the Roblox Project Manager.

## Features

- 🔄 **Real-time Sync**: Receive file updates from VS Code
- 📁 **Auto-hierarchy**: Automatically creates folder structure based on file paths
- 🎯 **Smart Script Detection**: Determines script type (Script, LocalScript, ModuleScript) automatically
- 💬 **WebSocket Communication**: Communicates with VS Code extension via WebSocket

## Installation

### Method 1: Manual Installation

1. Copy all files from this folder to your Roblox Studio plugins directory:
   - Windows: `%LOCALAPPDATA%\Roblox\Plugins\`
   - Mac: `~/Documents/Roblox/Plugins/`

2. Restart Roblox Studio

3. Look for the "Roblox Project Manager" button in the plugins toolbar

### Method 2: Install as Plugin File

1. In Roblox Studio, go to **Plugins** → **Plugins Folder**
2. Copy the entire `roblox-plugin` folder into the Plugins directory
3. Restart Roblox Studio

## Usage

1. **Activate Plugin**:
   - Click the "Toggle Sync" button in the Roblox Studio toolbar
   - The button will highlight when active

2. **Connect from VS Code**:
   - Open your project in VS Code with the Roblox Manager extension
   - Run the command `Roblox: Sync with Roblox Studio`
   - The plugin will receive and apply updates

3. **File Synchronization**:
   - When files are saved in VS Code, they are automatically sent to Roblox Studio
   - Scripts are created in `ServerScriptService` with proper folder hierarchy
   - Existing scripts are updated in-place

## Architecture

```
roblox-plugin/
├── main.lua              # Plugin entry point and message handling
├── fileHandler.lua       # File creation and update logic
├── httpServer.lua        # HTTP/WebSocket communication
├── plugin.manifest.json  # Plugin metadata
└── README.md
```

## Communication Protocol

The plugin listens for these message types from VS Code:

### FILE_UPDATE
Updates a single file:
```json
{
  "type": "FILE_UPDATE",
  "data": {
    "path": "src/player/PlayerController.lua",
    "content": "-- Lua code here",
    "timestamp": 1234567890
  }
}
```

### BATCH_UPDATE
Updates multiple files at once:
```json
{
  "type": "BATCH_UPDATE",
  "data": {
    "files": [
      { "path": "...", "content": "..." }
    ],
    "timestamp": 1234567890
  }
}
```

### CREATE_SCRIPT
Creates a new script:
```json
{
  "type": "CREATE_SCRIPT",
  "data": {
    "name": "NewScript",
    "scriptType": "Script",
    "parent": "workspace"
  }
}
```

### DELETE_SCRIPT
Deletes a script:
```json
{
  "type": "DELETE_SCRIPT",
  "data": {
    "path": "src/player/OldScript.lua"
  }
}
```

## Response Messages

The plugin sends these responses back to VS Code:

### ACK (Acknowledgement)
```json
{
  "type": "ACK",
  "data": {
    "action": "FILE_UPDATE",
    "target": "script_path",
    "timestamp": 1234567890
  }
}
```

### ERROR
```json
{
  "type": "ERROR",
  "data": {
    "action": "FILE_UPDATE",
    "target": "script_path",
    "error": "Error message",
    "timestamp": 1234567890
  }
}
```

## File Path Mapping

The plugin maps file paths to Roblox hierarchy:

| File Path | Roblox Location |
|-----------|----------------|
| `src/player/Controller.lua` | `ServerScriptService.src.player.Controller` |
| `ui/MainMenu.lua` | `ServerScriptService.ui.MainMenu` |
| `modules/Utils.lua` | `ServerScriptService.modules.Utils` |

## Script Type Detection

The plugin automatically determines script type:

- **ModuleScript**: Files containing `return` or with "module" in path
- **LocalScript**: Files with "local" in path or content
- **Script**: Default for all other cases

## Troubleshooting

### Plugin not showing up
- Ensure files are in the correct Plugins directory
- Restart Roblox Studio completely
- Check Output window for error messages

### Connection fails
- Verify the port number matches in both VS Code settings and plugin
- Check Windows Firewall allows local connections on port 8080
- Ensure no other application is using the same port

### Files not updating
- Click the toolbar button to ensure plugin is active (highlighted)
- Check Output window for error messages
- Verify file paths are valid and don't contain special characters

### Scripts appear in wrong location
- The plugin creates scripts in `ServerScriptService` by default
- Folder structure mirrors your file path
- To change location, modify the `GetOrCreateParent` function in `fileHandler.lua`

## Configuration

Edit `main.lua` to change settings:

```lua
-- Change WebSocket port
local WEBSOCKET_PORT = 8080

-- Change default script location
-- In fileHandler.lua, modify GetOrCreateParent function
```

## Limitations

- Roblox Studio doesn't support native WebSocket servers
- Communication requires a proxy server (Memory Manager) for live sync
- HTTP polling is used as fallback with 100ms interval
- Large file batches may take time to process

## Development

To modify the plugin:

1. Edit Lua files in this directory
2. Save changes
3. In Roblox Studio, reload the plugin or restart Studio
4. Test changes with the VS Code extension

## Future Enhancements

- [ ] Live file watching in Roblox Studio
- [ ] Reverse sync (Roblox → VS Code)
- [ ] Custom script placement rules
- [ ] UI for configuration
- [ ] Sync status indicators

## License

MIT
