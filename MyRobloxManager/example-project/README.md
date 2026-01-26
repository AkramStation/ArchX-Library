# Example Roblox Project

This is a sample Roblox project to demonstrate the synchronization workflow between VS Code and Roblox Studio using the Roblox Project Manager.

## Project Structure

```
example-project/
├── src/
│   ├── player/
│   │   └── PlayerController.lua    # Player control logic
│   ├── modules/
│   │   └── Utils.lua               # Utility functions
│   ├── ui/
│   │   └── MainMenuUI.lua          # UI components
│   └── GameManager.lua             # Main game logic
├── project.json                     # Project configuration
└── README.md
```

## Scripts Overview

### PlayerController.lua
**Location**: `src/player/PlayerController.lua`  
**Type**: ModuleScript  
**Purpose**: Manages player character controls and states

Key features:
- Player initialization
- Speed control
- Movement state tracking

### GameManager.lua
**Location**: `src/GameManager.lua`  
**Type**: Script  
**Purpose**: Central game state management

Key features:
- Player join/leave handling
- Round management
- Game state tracking

### Utils.lua
**Location**: `src/modules/Utils.lua`  
**Type**: ModuleScript  
**Purpose**: Common utility functions

Includes:
- Math utilities (round, clamp, lerp)
- String utilities (capitalize, split)
- Table utilities (deepCopy, contains)
- Time formatting
- Random helpers

### MainMenuUI.lua
**Location**: `src/ui/MainMenuUI.lua`  
**Type**: LocalScript  
**Purpose**: Client-side UI management

Features:
- Main menu creation
- UI element management

## Testing the Sync Workflow

### Step 1: Open Project in VS Code

1. Open VS Code
2. Use command: `Roblox: Open Project Folder`
3. Select this `example-project` folder

### Step 2: Start Memory Manager

```bash
cd ../memory-manager
npm install
npm run dev
```

### Step 3: Install Roblox Plugin

1. Copy `../roblox-plugin/` files to Roblox Studio Plugins folder
2. Restart Roblox Studio
3. Click "Toggle Sync" button to activate

### Step 4: Sync Files

1. In VS Code, run: `Roblox: Sync with Roblox Studio`
2. Check Roblox Studio - scripts should appear in `ServerScriptService`

### Step 5: Test Live Sync

1. In VS Code settings, enable: `robloxManager.autoSync`
2. Edit any `.lua` file and save
3. Changes should automatically sync to Roblox Studio

## Expected Roblox Hierarchy

After syncing, your Roblox project should look like:

```
ServerScriptService/
├── src/
│   ├── player/
│   │   └── PlayerController (ModuleScript)
│   ├── modules/
│   │   └── Utils (ModuleScript)
│   └── GameManager (Script)

StarterGui/
└── ui/
    └── MainMenuUI (LocalScript)
```

## Modifying Scripts

### Example 1: Change Player Speed

Edit `src/player/PlayerController.lua`:

```lua
function PlayerController.new(player)
    local self = setmetatable({}, PlayerController)
    self.player = player
    self.isWalking = false
    self.speed = 32  -- Changed from 16 to 32
    
    return self
end
```

Save the file and it will automatically sync to Roblox Studio (if auto-sync is enabled).

### Example 2: Add New Utility Function

Edit `src/modules/Utils.lua`:

```lua
-- Add to the Utils module
function Utils.randomRange(min, max)
    return math.random() * (max - min) + min
end
```

### Example 3: Create New Script

1. In VS Code, run: `Roblox: Create New Script`
2. Enter name: `CombatSystem`
3. A new script will be created with template code
4. Edit and save to sync

## Troubleshooting

### Scripts not appearing in Roblox Studio
- Ensure the plugin is activated (button highlighted)
- Check Memory Manager is running
- Verify WebSocket connection (check Output window)

### Auto-sync not working
- Check `robloxManager.autoSync` is set to `true`
- Ensure file has `.lua` extension
- Check console for error messages

### Scripts in wrong location
- Check `project.json` file mapping configuration
- Plugin creates scripts in `ServerScriptService` by default
- UI scripts should manually be moved to `StarterGui`

## Next Steps

1. **Customize Scripts**: Modify the example scripts to suit your game
2. **Add More Scripts**: Create additional game logic files
3. **Test Functionality**: Run the game in Roblox Studio to test
4. **Iterate**: Make changes in VS Code and sync to Roblox Studio

## Tips

- Use meaningful file names for better organization
- Keep ModuleScripts in the `modules/` folder
- LocalScripts for UI should go in `ui/` folder
- Use the Utils module for common functions
- Test frequently in Roblox Studio

## Learning Resources

- [Roblox Lua Documentation](https://create.roblox.com/docs)
- [ModuleScripts Guide](https://create.roblox.com/docs/scripting/scripts#modulescripts)
- [LocalScripts vs Scripts](https://create.roblox.com/docs/scripting/scripts)

## License

MIT - Feel free to use this example project as a starting point for your own games!
