-- GameManager.lua
-- Example game manager script

local GameManager = {}
GameManager.__index = GameManager

-- Services
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

function GameManager.new()
    local self = setmetatable({}, GameManager)
    self.players = {}
    self.gameState = "Waiting"
    self.roundTime = 180
    
    return self
end

function GameManager:init()
    print("[GameManager] Initializing game manager...")
    
    -- Set up player events
    Players.PlayerAdded:Connect(function(player)
        self:onPlayerAdded(player)
    end)
    
    Players.PlayerRemoving:Connect(function(player)
        self:onPlayerRemoving(player)
    end)
    
    -- Initialize existing players
    for _, player in ipairs(Players:GetPlayers()) do
        self:onPlayerAdded(player)
    end
    
    print("[GameManager] Game manager ready!")
end

function GameManager:onPlayerAdded(player)
    print("[GameManager] Player joined:", player.Name)
    
    self.players[player.UserId] = {
        player = player,
        score = 0,
        joinTime = os.time()
    }
end

function GameManager:onPlayerRemoving(player)
    print("[GameManager] Player left:", player.Name)
    self.players[player.UserId] = nil
end

function GameManager:startRound()
    if self.gameState ~= "Waiting" then
        warn("[GameManager] Cannot start - game already in progress")
        return
    end
    
    print("[GameManager] Starting new round...")
    self.gameState = "InProgress"
    
    -- Round timer
    task.wait(self.roundTime)
    self:endRound()
end

function GameManager:endRound()
    print("[GameManager] Round ended")
    self.gameState = "Ended"
    
    -- Reset after delay
    task.wait(10)
    self:reset()
end

function GameManager:reset()
    print("[GameManager] Resetting game...")
    self.gameState = "Waiting"
end

function GameManager:getPlayerCount()
    local count = 0
    for _ in pairs(self.players) do
        count = count + 1
    end
    return count
end

return GameManager
