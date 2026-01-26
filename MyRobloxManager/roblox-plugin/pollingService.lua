-- Polling Service for Roblox Manager
-- Periodically checks Memory Manager for pending updates

local HttpService = game:GetService("HttpService")
local PollingService = {}
PollingService.__index = PollingService

function PollingService.new(fileHandler, uiManager)
    local self = setmetatable({}, PollingService)
    self.fileHandler = fileHandler
    self.uiManager = uiManager
    self.isRunning = false
    self.pollInterval = 1.0 -- Check every 1 second
    self.memoryManagerUrl = "http://localhost:3000"
    return self
end

function PollingService:Start()
    if self.isRunning then return end
    self.isRunning = true
    
    print("[Polling] Started sync service")
    self.uiManager:updateStatus("Sync Service: Active", "connected")
    
    spawn(function()
        while self.isRunning do
            self:CheckForUpdates()
            wait(self.pollInterval)
        end
    end)
end

function PollingService:Stop()
    self.isRunning = false
    self.uiManager:updateStatus("Sync Service: Stopped", "info")
end

function PollingService:CheckForUpdates()
    -- skip if HTTP not enabled
    if not HttpService.HttpEnabled then return end
    
    local success, result = pcall(function()
        return HttpService:GetAsync(self.memoryManagerUrl .. "/api/sync/poll")
    end)
    
    if success then
        local data = HttpService:JSONDecode(result)
        if data.count > 0 then
            self:ProcessUpdates(data.events)
        end
    else
        -- warn("Connection failed: " .. tostring(result))
    end
end

function PollingService:ProcessUpdates(events)
    local processedIds = {}
    local successCount = 0
    
    for _, event in ipairs(events) do
        print("[Sync] Processing: " .. event.type .. " " .. event.path)
        
        if event.type == "update" then
            local success = pcall(function()
                self.fileHandler:UpdateFile(event.path, event.content)
            end)
            if success then successCount = successCount + 1 end
            
        elseif event.type == "delete" then
             local success = pcall(function()
                self.fileHandler:DeleteScript(event.path)
            end)
            if success then successCount = successCount + 1 end
        end
        
        table.insert(processedIds, event.id)
    end
    
    -- Confirm receipt
    if #processedIds > 0 then
        pcall(function()
            HttpService:PostAsync(
                self.memoryManagerUrl .. "/api/sync/confirm",
                HttpService:JSONEncode({ eventIds = processedIds }),
                Enum.HttpContentType.ApplicationJson
            )
        end)
        
        self.uiManager:updateStatus("Synced " .. successCount .. " changes", "connected")
        self.uiManager:updateStats(#self.fileHandler:GetAllScripts(), os.date("%H:%M:%S"))
    end
end

return PollingService
