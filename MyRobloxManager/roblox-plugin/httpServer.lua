-- HTTP Server Module
-- Handles HTTP/WebSocket communication with VS Code

local HttpService = game:GetService("HttpService")
local RunService = game:GetService("RunService")

local HttpServer = {}
HttpServer.__index = HttpServer

function HttpServer.new(port)
    local self = setmetatable({}, HttpServer)
    self.port = port
    self.isRunning = false
    self.messageQueue = {}
    self.OnMessage = Instance.new("BindableEvent")
    
    return self
end

-- Start the server
function HttpServer:Start()
    if self.isRunning then
        warn("[HttpServer] Already running")
        return
    end
    
    self.isRunning = true
    
    -- Note: Roblox doesn't support direct WebSocket servers
    -- This is a simulation using HttpService for polling
    -- In production, you would use a proxy server or HTTP polling
    
    print("[HttpServer] Started on port", self.port)
    
    -- Start message processing loop
    self:StartMessageLoop()
end

-- Stop the server
function HttpServer:Stop()
    self.isRunning = false
    print("[HttpServer] Stopped")
end

-- Start message processing loop
function HttpServer:StartMessageLoop()
    -- This would be replaced with actual WebSocket implementation
    -- For now, we'll simulate message reception
    
    spawn(function()
        while self.isRunning do
            -- Process queued messages
            if #self.messageQueue > 0 then
                local message = table.remove(self.messageQueue, 1)
                self.OnMessage:Fire(message)
            end
            
            wait(0.1) -- Poll interval
        end
    end)
end

-- Send message to VS Code
function HttpServer:Send(message)
    if not self.isRunning then
        warn("[HttpServer] Cannot send - server not running")
        return
    end
    
    -- In production, this would send via WebSocket
    -- For now, we'll just print it
    print("[HttpServer] Sending:", message)
    
    -- Attempt to send via HTTP POST (requires proxy server)
    local success, result = pcall(function()
        return HttpService:PostAsync(
            "http://localhost:3000/api/roblox/message",
            message,
            Enum.HttpContentType.ApplicationJson
        )
    end)
    
    if not success then
        -- Silently fail if proxy not available
        -- This is expected when running standalone
    end
end

-- Receive message (called externally or via polling)
function HttpServer:ReceiveMessage(message)
    if self.isRunning then
        table.insert(self.messageQueue, message)
    end
end

-- Simulate receiving a message (for testing)
function HttpServer:SimulateMessage(messageType, data)
    local message = {
        type = messageType,
        data = data
    }
    
    local messageStr = HttpService:JSONEncode(message)
    self:ReceiveMessage(messageStr)
end

return HttpServer
