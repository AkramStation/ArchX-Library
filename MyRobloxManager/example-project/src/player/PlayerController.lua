-- PlayerController.lua
-- Example player control script for demonstration

local PlayerController = {}
PlayerController.__index = PlayerController

function PlayerController.new(player)
    local self = setmetatable({}, PlayerController)
    self.player = player
    self.isWalking = false
    self.speed = 16
    
    return self
end

function PlayerController:init()
    print("[PlayerController] Initialized for player:", self.player.Name)
    
    -- Set up character added listener
    self.player.CharacterAdded:Connect(function(character)
        self:onCharacterAdded(character)
    end)
    
    -- Handle existing character
    if self.player.Character then
        self:onCharacterAdded(self.player.Character)
    end
end

function PlayerController:onCharacterAdded(character)
    print("[PlayerController] Character added:", character.Name)
    
    local humanoid = character:WaitForChild("Humanoid")
    humanoid.WalkSpeed = self.speed
    
    -- Set up state tracking
    humanoid.Running:Connect(function(speed)
        self.isWalking = speed > 0
    end)
end

function PlayerController:setSpeed(newSpeed)
    self.speed = newSpeed
    
    if self.player.Character then
        local humanoid = self.player.Character:FindFirstChild("Humanoid")
        if humanoid then
            humanoid.WalkSpeed = newSpeed
        end
    end
    
    print("[PlayerController] Speed set to:", newSpeed)
end

function PlayerController:getSpeed()
    return self.speed
end

function PlayerController:isMoving()
    return self.isWalking
end

return PlayerController
