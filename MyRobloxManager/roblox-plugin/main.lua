-- Roblox Project Manager Plugin
-- Main entry point for the plugin

local HttpService = game:GetService("HttpService")
local InsertService = game:GetService("InsertService")

-- Import modules with fallback search
local function requireModule(name)
    -- Try exact name
    local module = script:FindFirstChild(name)
    if not module then module = script.Parent:FindFirstChild(name) end
    
    -- Try with .lua extension
    if not module then module = script:FindFirstChild(name .. ".lua") end
    if not module then module = script.Parent:FindFirstChild(name .. ".lua") end
    
    if not module then
        -- List available files for debugging
        print("[Debug] Looking for: " .. name)
        print("[Debug] Script children:")
        for _, c in ipairs(script:GetChildren()) do print("- " .. c.Name) end
        print("[Debug] Parent children:")
        for _, c in ipairs(script.Parent:GetChildren()) do print("- " .. c.Name) end
        
        error("Could not find module: " .. name)
    end
    return require(module)
end

local FileHandler = requireModule("fileHandler")
local HttpServer = requireModule("httpServer")
local UIManager = requireModule("uiManager")

-- Plugin configuration
local WEBSOCKET_PORT = 8080
local PLUGIN_NAME = "Roblox Project Manager"
local PLUGIN_VERSION = "0.1.0"

-- Create toolbar and button
local toolbar = plugin:CreateToolbar(PLUGIN_NAME)
local toggleButton = toolbar:CreateButton(
    "Toggle Panel",
    "Show/hide the Roblox Manager panel",
    "rbxasset://textures/ui/Settings/MenuBarIcons/ContactsTab.png"
)

local PollingService = requireModule("pollingService")

-- Plugin state
local isActive = false
local uiManager = UIManager.new(plugin)
local fileHandler = FileHandler.new()
local pollingService = PollingService.new(fileHandler, uiManager)

-- Create UI
local widget = uiManager:createWidget()

-- Toggle panel visibility
toggleButton.Click:Connect(function()
    uiManager:toggle()
end)

-- Connect UI button handlers
uiManager.connectButton.MouseButton1Click:Connect(function()
    uiManager:updateStatus("Starting Sync Service...", "loading")
    isActive = true
    pollingService:Start()
    uiManager:setConnected(true)
end)

uiManager.disconnectButton.MouseButton1Click:Connect(function()
    isActive = false
    pollingService:Stop()
    uiManager:setConnected(false)
end)

uiManager.syncButton.MouseButton1Click:Connect(function()
    if not isActive then
        uiManager:updateStatus("Not connected!", "error")
        return
    end
    
    uiManager:updateStatus("Syncing files...", "loading")
    -- Sync will happen via incoming messages
    uiManager:updateStatus("Waiting for sync...", "info")
end)

uiManager.refreshButton.MouseButton1Click:Connect(function()
    local scriptCount = #fileHandler:GetAllScripts()
    local lastSync = os.date("%H:%M:%S")
    uiManager:updateStats(scriptCount, lastSync)
    uiManager:updateStatus("Status refreshed", "info")
end)

-- Start HTTP/WebSocket server
function startServer()
    if httpServer then
        httpServer:Stop()
    end
    
    httpServer = HttpServer.new(WEBSOCKET_PORT)
    
    -- Handle incoming messages
    httpServer.OnMessage:Connect(function(message)
        handleMessage(message)
    end)
    
    httpServer:Start()
    print("[" .. PLUGIN_NAME .. "] Server started on port " .. WEBSOCKET_PORT)
end

-- Stop server
function stopServer()
    if httpServer then
        httpServer:Stop()
        httpServer = nil
        print("[" .. PLUGIN_NAME .. "] Server stopped")
    end
end

-- Handle messages from VS Code
function handleMessage(messageStr)
    local success, message = pcall(function()
        return HttpService:JSONDecode(messageStr)
    end)
    
    if not success then
        warn("[" .. PLUGIN_NAME .. "] Failed to parse message:", messageStr)
        return
    end
    
    local messageType = message.type
    local data = message.data
    
    if messageType == "FILE_UPDATE" then
        handleFileUpdate(data)
    elseif messageType == "BATCH_UPDATE" then
        handleBatchUpdate(data)
    elseif messageType == "CREATE_SCRIPT" then
        handleCreateScript(data)
    elseif messageType == "DELETE_SCRIPT" then
        handleDeleteScript(data)
    else
        warn("[" .. PLUGIN_NAME .. "] Unknown message type:", messageType)
    end
end

-- Handle single file update
function handleFileUpdate(data)
    local path = data.path
    local content = data.content
    
    print("[" .. PLUGIN_NAME .. "] Updating file:", path)
    
    local success, err = pcall(function()
        fileHandler:UpdateFile(path, content)
    end)
    
    if success then
        sendAcknowledgement("FILE_UPDATE", path)
        local scriptCount = #fileHandler:GetAllScripts()
        uiManager:updateStats(scriptCount, os.date("%H:%M:%S"))
        uiManager:updateStatus("File updated: " .. path, "connected")
    else
        sendError("FILE_UPDATE", path, err)
        uiManager:updateStatus("Error updating file", "error")
    end
end

-- Handle batch file updates
function handleBatchUpdate(data)
    local files = data.files
    local successCount = 0
    local failCount = 0
    
    print("[" .. PLUGIN_NAME .. "] Batch updating " .. #files .. " files")
    uiManager:updateStatus("Syncing " .. #files .. " files...", "loading")
    
    for _, file in ipairs(files) do
        local success = pcall(function()
            fileHandler:UpdateFile(file.path, file.content)
        end)
        
        if success then
            successCount = successCount + 1
        else
            failCount = failCount + 1
        end
    end
    
    print(string.format("[%s] Batch update complete: %d succeeded, %d failed", 
        PLUGIN_NAME, successCount, failCount))
    
    sendAcknowledgement("BATCH_UPDATE", tostring(successCount) .. " files")
    
    local scriptCount = #fileHandler:GetAllScripts()
    uiManager:updateStats(scriptCount, os.date("%H:%M:%S"))
    
    if failCount == 0 then
        uiManager:updateStatus(string.format("Synced %d files successfully!", successCount), "connected")
    else
        uiManager:updateStatus(string.format("Synced %d files (%d failed)", successCount, failCount), "error")
    end
end

-- Handle script creation
function handleCreateScript(data)
    local scriptName = data.name
    local scriptType = data.scriptType or "Script"
    local parent = data.parent or workspace
    
    print("[" .. PLUGIN_NAME .. "] Creating script:", scriptName)
    
    local success, script = pcall(function()
        return fileHandler:CreateScript(scriptName, scriptType, parent)
    end)
    
    if success then
        sendAcknowledgement("CREATE_SCRIPT", scriptName)
    else
        sendError("CREATE_SCRIPT", scriptName, script)
    end
end

-- Handle script deletion
function handleDeleteScript(data)
    local path = data.path
    
    print("[" .. PLUGIN_NAME .. "] Deleting script:", path)
    
    local success, err = pcall(function()
        fileHandler:DeleteScript(path)
    end)
    
    if success then
        sendAcknowledgement("DELETE_SCRIPT", path)
    else
        sendError("DELETE_SCRIPT", path, err)
    end
end

-- Send acknowledgement to VS Code
function sendAcknowledgement(action, target)
    if httpServer then
        local message = {
            type = "ACK",
            data = {
                action = action,
                target = target,
                timestamp = os.time()
            }
        }
        httpServer:Send(HttpService:JSONEncode(message))
    end
end

-- Send error to VS Code
function sendError(action, target, error)
    if httpServer then
        local message = {
            type = "ERROR",
            data = {
                action = action,
                target = target,
                error = tostring(error),
                timestamp = os.time()
            }
        }
        httpServer:Send(HttpService:JSONEncode(message))
    end
end

-- Cleanup on plugin unload
plugin.Unloading:Connect(function()
    stopServer()
    print("[" .. PLUGIN_NAME .. "] Plugin unloaded")
end)

print("[" .. PLUGIN_NAME .. "] Loaded successfully - Click toolbar button to activate")
