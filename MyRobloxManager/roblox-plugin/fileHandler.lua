-- File Handler Module
-- Handles creating, updating, and deleting Roblox scripts

local FileHandler = {}
FileHandler.__index = FileHandler

function FileHandler.new()
    local self = setmetatable({}, FileHandler)
    self.scriptCache = {}
    return self
end

-- Parse file path to determine hierarchy
function FileHandler:ParsePath(filePath)
    -- Convert file path to Roblox hierarchy
    -- Example: "src/player/PlayerController.lua" -> workspace.src.player.PlayerController
    
    local parts = {}
    for part in string.gmatch(filePath, "[^/\\]+") do
        table.insert(parts, part)
    end
    
    -- Remove .lua extension from last part
    if #parts > 0 then
        parts[#parts] = string.gsub(parts[#parts], "%.lua$", "")
    end
    
    return parts
end

-- Get or create parent container
function FileHandler:GetOrCreateParent(pathParts)
    local current = game:GetService("ServerScriptService")
    
    -- Create folder hierarchy
    for i = 1, #pathParts - 1 do
        local folderName = pathParts[i]
        local folder = current:FindFirstChild(folderName)
        
        if not folder then
            folder = Instance.new("Folder")
            folder.Name = folderName
            folder.Parent = current
        end
        
        current = folder
    end
    
    return current
end

-- Determine script type from path or content
function FileHandler:DetermineScriptType(path, content)
    if string.find(path:lower(), "local") or string.find(content, "local ") then
        return "LocalScript"
    elseif string.find(path:lower(), "module") or string.find(content, "return ") then
        return "ModuleScript"
    else
        return "Script"
    end
end

-- Update or create a file
function FileHandler:UpdateFile(filePath, content)
    local pathParts = self:ParsePath(filePath)
    if #pathParts == 0 then
        error("Invalid file path: " .. filePath)
    end
    
    local scriptName = pathParts[#pathParts]
    local parent = self:GetOrCreateParent(pathParts)
    
    -- Find existing script or create new one
    local script = parent:FindFirstChild(scriptName)
    
    if not script then
        local scriptType = self:DetermineScriptType(filePath, content)
        script = Instance.new(scriptType)
        script.Name = scriptName
        script.Parent = parent
    end
    
    -- Update source
    script.Source = content
    
    -- Cache the script
    self.scriptCache[filePath] = script
    
    print("[FileHandler] Updated:", filePath)
    return script
end

-- Create a new script
function FileHandler:CreateScript(scriptName, scriptType, parent)
    scriptType = scriptType or "Script"
    parent = parent or game:GetService("ServerScriptService")
    
    local script = Instance.new(scriptType)
    script.Name = scriptName
    script.Source = "-- " .. scriptName .. "\nprint('Hello from " .. scriptName .. "')"
    script.Parent = parent
    
    print("[FileHandler] Created:", scriptName)
    return script
end

-- Delete a script
function FileHandler:DeleteScript(filePath)
    local pathParts = self:ParsePath(filePath)
    if #pathParts == 0 then
        error("Invalid file path: " .. filePath)
    end
    
    local scriptName = pathParts[#pathParts]
    local parent = self:GetOrCreateParent(pathParts)
    
    local script = parent:FindFirstChild(scriptName)
    if script then
        script:Destroy()
        self.scriptCache[filePath] = nil
        print("[FileHandler] Deleted:", filePath)
    else
        warn("[FileHandler] Script not found:", filePath)
    end
end

-- Get all scripts in the game
function FileHandler:GetAllScripts()
    local scripts = {}
    
    local function collectScripts(parent)
        for _, child in ipairs(parent:GetChildren()) do
            if child:IsA("LuaSourceContainer") then
                table.insert(scripts, child)
            end
            collectScripts(child)
        end
    end
    
    collectScripts(game:GetService("ServerScriptService"))
    collectScripts(game:GetService("StarterPlayer"))
    collectScripts(game:GetService("StarterGui"))
    
    return scripts
end

return FileHandler
