--[[
    🎮 Roblox Project Manager - Single File Plugin
    Merged version to prevent module loading errors.
    
    INSTRUCTIONS:
    1. Copy this file to %LOCALAPPDATA%\Roblox\Plugins
    2. Restart Roblox Studio
    3. Make sure HTTP Requests are enabled!
]]

local HttpService = game:GetService("HttpService")
local RunService = game:GetService("RunService")
local CoreGui = game:GetService("CoreGui")

local PLUGIN_NAME = "Roblox Project Manager"
local MEMORY_MANAGER_URL = "http://localhost:3000"

-- ==============================================================================
-- MODULE: FileHandler
-- ==============================================================================
local FileHandler = {}
FileHandler.__index = FileHandler

function FileHandler.new()
    local self = setmetatable({}, FileHandler)
    return self
end

function FileHandler:GetParentDir(path)
    -- "src/server/init.lua" -> "src/server"
    return path:match("(.+)/[^/]+$")
end

function FileHandler:GetFileName(path)
    -- "src/server/script.lua" -> "script.lua"
    return path:match("[^/]+$")
end

function FileHandler:GetScriptName(fileName)
    -- "script.lua" -> "script"
    return fileName:match("(.+)%.lua$") or fileName
end

function FileHandler:GetOrCreateFolder(pathArray, parent)
    local current = parent
    for _, folderName in ipairs(pathArray) do
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

function FileHandler:UpdateFile(path, content)
    -- Determine script type and location based on path
    -- Simplification: All scripts go to ServerScriptService for now unless specified
    local parent = game:GetService("ServerScriptService")
    
    -- Parse path "src/game/manager.lua"
    local parts = {}
    for part in path:gmatch("[^/]+") do
        table.insert(parts, part)
    end
    
    local fileName = table.remove(parts) -- last item
    local scriptName = self:GetScriptName(fileName)
    
    -- Create folder structure
    local container = self:GetOrCreateFolder(parts, parent)
    
    -- Find or create script
    local scriptInstance = container:FindFirstChild(scriptName)
    
    if not scriptInstance then
        -- Detect type
        if fileName:find("%.server%.lua") then
            scriptInstance = Instance.new("Script")
        elseif fileName:find("%.client%.lua") then
            scriptInstance = Instance.new("LocalScript")
        elseif fileName:find("%.module%.lua") or content:find("^%s*return") then
            scriptInstance = Instance.new("ModuleScript")
        else
            scriptInstance = Instance.new("Script") -- Default
        end
        scriptInstance.Name = scriptName
        scriptInstance.Parent = container
        print("created new script: " .. scriptName)
    end
    
    if scriptInstance.Source ~= content then
        scriptInstance.Source = content
        print("Updated: " .. scriptName)
    end
end

function FileHandler:DeleteScript(path)
    -- TODO: Implement delete logic matching UpdateFile traversal
end

function FileHandler:GetAllScripts()
    -- Debugging helper
    return game:GetService("ServerScriptService"):GetDescendants()
end

-- ==============================================================================
-- MODULE: UIManager
-- ==============================================================================
local UIManager = {}
UIManager.__index = UIManager

function UIManager.new(plugin)
    local self = setmetatable({}, UIManager)
    self.plugin = plugin
    self.widget = nil
    
    -- Define widget info
    local widgetInfo = DockWidgetPluginGuiInfo.new(
        Enum.InitialDockState.Right,
        false, false,
        300, 200,
        200, 100
    )
    
    self.widget = plugin:CreateDockWidgetPluginGui("RobloxManagerUltimate", widgetInfo)
    self.widget.Title = "Roblox Manager (Hub Mode)"
    
    self:createUI()
    return self
end

function UIManager:createUI()
    -- Simple Hub UI
    local bg = Instance.new("Frame")
    bg.Size = UDim2.fromScale(1, 1)
    bg.BackgroundColor3 = Color3.fromRGB(40, 40, 40)
    bg.Parent = self.widget
    
    local list = Instance.new("UIListLayout")
    list.Padding = UDim.new(0, 10)
    list.HorizontalAlignment = Enum.HorizontalAlignment.Center
    list.SortOrder = Enum.SortOrder.LayoutOrder
    list.Parent = bg
    
    local padding = Instance.new("UIPadding")
    padding.PaddingTop = UDim.new(0, 20)
    padding.Parent = bg
    
    -- Status
    self.statusLabel = Instance.new("TextLabel")
    self.statusLabel.Size = UDim2.new(0.9, 0, 0, 30)
    self.statusLabel.BackgroundColor3 = Color3.fromRGB(60, 60, 60)
    self.statusLabel.TextColor3 = Color3.fromRGB(255, 255, 255)
    self.statusLabel.Text = "Not Connected"
    self.statusLabel.Parent = bg
    
    -- Connect Button
    self.connectBtn = Instance.new("TextButton")
    self.connectBtn.Size = UDim2.new(0.9, 0, 0, 40)
    self.connectBtn.BackgroundColor3 = Color3.fromRGB(0, 170, 0)
    self.connectBtn.Text = "Start Sync"
    self.connectBtn.TextColor3 = Color3.WHITE
    self.connectBtn.Font = Enum.Font.SourceSansBold
    self.connectBtn.TextSize = 18
    self.connectBtn.Parent = bg
    
    local corner = Instance.new("UICorner")
    corner.Parent = self.connectBtn
end

function UIManager:updateStatus(text, color)
    self.statusLabel.Text = text
    if color == "green" then 
        self.statusLabel.TextColor3 = Color3.fromRGB(100, 255, 100) 
    else
        self.statusLabel.TextColor3 = Color3.fromRGB(255, 255, 255)
    end
end

function UIManager:toggle()
    self.widget.Enabled = not self.widget.Enabled
end

-- ==============================================================================
-- MAIN LOGIC
-- ==============================================================================

local ui = UIManager.new(plugin)
local files = FileHandler.new()
local isRunning = false

-- Helper: Export scripts finding
local function getAllScripts()
    local scripts = {}
    local services = {
        game:GetService("ServerScriptService"),
        game:GetService("ReplicatedStorage"),
        game:GetService("StarterPlayer"),
        game:GetService("Workspace")
    }
    
    for _, service in ipairs(services) do
        for _, obj in ipairs(service:GetDescendants()) do
            if obj:IsA("LuaSourceContainer") and not obj:IsDescendantOf(plugin) then
                -- Construct path relative to service
                local path = obj:GetFullName()
                -- Basic cleaning: "ServerScriptService.MyScript" -> "src/ServerScriptService/MyScript.lua" or similar
                -- For simple mapping, we'll prefix with 'game/' for now or mapping logic
                
                -- Custom Simple Mapping for VS Code structure
                -- Map services to folder names
                local root = ""
                if obj:IsDescendantOf(game:GetService("ServerScriptService")) then root = "src/server" 
                elseif obj:IsDescendantOf(game:GetService("ReplicatedStorage")) then root = "src/shared"
                elseif obj:IsDescendantOf(game:GetService("StarterPlayer")) then root = "src/client"
                else root = "src/misc" end
                
                local relPath = obj.Name
                local p = obj.Parent
                while p and not (p == service) do
                    relPath = p.Name .. "/" .. relPath
                    p = p.Parent
                end
                
                local ext = ".lua"
                if obj:IsA("ModuleScript") then ext = ".lua" -- standard
                elseif obj:IsA("LocalScript") then ext = ".client.lua"
                elseif obj:IsA("Script") then ext = ".server.lua" end
                
                local fullPath = root .. "/" .. relPath .. ext
                
                table.insert(scripts, {
                    path = fullPath,
                    content = obj.Source
                })
            end
        end
    end
    return scripts
end

local function exportScriptsToHub()
    ui:updateStatus("Scanning scripts...", "loading")
    local scripts = getAllScripts()
    print("[Export] Found " .. #scripts .. " scripts")
    
    local success, err = pcall(function()
        local body = HttpService:JSONEncode({ files = scripts })
        HttpService:PostAsync(MEMORY_MANAGER_URL .. "/api/files/batch-update", body, Enum.HttpContentType.ApplicationJson)
    end)
    
    if success then
        ui:updateStatus("Exported " .. #scripts .. " scripts!", "green")
        print("[Export] Successfully sent to Memory Hub")
    else
        ui:updateStatus("Export Failed", "red")
        warn("Export error: " .. tostring(err))
    end
end

-- Create Toolbar
local toolbar = plugin:CreateToolbar("Roblox Manager")
local btn = toolbar:CreateButton("Toggle Hub", "Open Sync Hub", "rbxassetid://0")

btn.Click:Connect(function()
    ui:toggle()
end)

-- Add Export Button to UI (injecting into createUI would be cleaner but appending here works for existing widget)
local exportBtn = Instance.new("TextButton")
exportBtn.Size = UDim2.new(0.9, 0, 0, 40)
exportBtn.Position = UDim2.new(0.05, 0, 0, 200) -- Approx position
exportBtn.BackgroundColor3 = Color3.fromRGB(0, 120, 255)
exportBtn.Text = "⬆️ Export Scripts to VS Code"
exportBtn.TextColor3 = Color3.WHITE
exportBtn.Font = Enum.Font.SourceSansBold
exportBtn.TextSize = 16
exportBtn.Parent = ui.widget:FindFirstChildOfClass("Frame")
local c = Instance.new("UICorner") c.Parent = exportBtn

exportBtn.MouseButton1Click:Connect(function()
    exportScriptsToHub()
end)

local LogService = game:GetService("LogService")

-- Log Streaming
local function startLogStream()
    LogService.MessageOut:Connect(function(message, messageType)
        -- Filter out our own logs to prevent loops
        if message:find("%[Sync%]") or message:find("%[Export%]") then return end
        
        -- Simple debounce/batching could happen here, but for now direct push
        pcall(function()
            local typeStr = tostring(messageType):gsub("Enum.MessageType.", "")
            local body = HttpService:JSONEncode({ 
                message = message,
                type = typeStr
            })
            HttpService:PostAsync(MEMORY_MANAGER_URL .. "/api/logs/push", body, Enum.HttpContentType.ApplicationJson)
        end)
    end)
end

-- Start logging immediately
pcall(startLogStream)

-- Sync Loop
local function syncLoop()
    while isRunning do
        if not HttpService.HttpEnabled then
            ui:updateStatus("ERROR: Enable HTTP!", "red")
            wait(2)
            continue
        end
        
        -- Poll
        local success, res = pcall(function()
            return HttpService:GetAsync(MEMORY_MANAGER_URL .. "/api/sync/poll")
        end)
        
        if success then
            local data = HttpService:JSONDecode(res)
            if data.count > 0 then
                print("[Sync] Processing " .. data.count .. " updates")
                local ids = {}
                for _, event in ipairs(data.events) do
                    if event.type == "update" then
                        files:UpdateFile(event.path, event.content)
                    end
                    table.insert(ids, event.id)
                end
                
                -- Confirm
                pcall(function()
                    HttpService:PostAsync(
                        MEMORY_MANAGER_URL .. "/api/sync/confirm",
                        HttpService:JSONEncode({ eventIds = ids }),
                        Enum.HttpContentType.ApplicationJson
                    )
                end)
                ui:updateStatus("Synced " .. #ids .. " files", "green")
            else
                ui:updateStatus("Monitoring... (Idle)", "green")
            end
        else
            ui:updateStatus("Connection Failed", "red")
        end
        
        wait(1)
    end
end

ui.connectBtn.MouseButton1Click:Connect(function()
    if isRunning then
        isRunning = false
        ui.connectBtn.Text = "Start Sync"
        ui.connectBtn.BackgroundColor3 = Color3.fromRGB(0, 170, 0)
        ui:updateStatus("Stopped")
    else
        isRunning = true
        ui.connectBtn.Text = "Stop Sync"
        ui.connectBtn.BackgroundColor3 = Color3.fromRGB(170, 0, 0)
        ui:updateStatus("Starting...", "green")
        
        -- Start loop in new thread
        task.spawn(syncLoop)
    end
end)

print("[Roblox Manager] Single-File Plugin Loaded")
