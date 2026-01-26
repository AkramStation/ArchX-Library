-- RoWinget Simple Manager (Standalone)
-- Place this script in ServerScriptService
-- Works without the full Plugin System

local HttpService = game:GetService("HttpService")
local Players = game:GetService("Players")
local RunService = game:GetService("RunService")

print("========================================")
print("  🚀 RoWinget - Simple Manager")
print("========================================")

-- Ensure HTTP is enabled
if not pcall(function() HttpService.HttpEnabled = true end) then
	warn("⚠️  HttpEnabled could not be set automatically.")
	warn("    Please run this in the Command Bar: game:GetService('HttpService').HttpEnabled = true")
end

-- Configuration
local CONFIG = {
	URL = "http://localhost:3000",
	INTERVAL = 2,
	THEME = {
		BG = Color3.fromRGB(30, 39, 46),    -- Dark Void
		ACCENT = Color3.fromRGB(0, 168, 255), -- Electric Blue
		TEXT = Color3.fromRGB(245, 246, 250), -- Text White
		SUCCESS = Color3.fromRGB(76, 209, 55),
		FAIL = Color3.fromRGB(232, 65, 24)
	}
}

-- Gloabal State
local statusUI = nil
local statusLabel = nil
local lastSyncTime = 0

-- ============================================================================
-- UI CREATION
-- ============================================================================
local function createStatusUI()
	local screenGui = Instance.new("ScreenGui")
	screenGui.Name = "RoWingetStatus"
	screenGui.ResetOnSpawn = false
	
	-- Card Container
	local frame = Instance.new("Frame")
	frame.Name = "StatusCard"
	frame.Size = UDim2.new(0, 200, 0, 80)
	frame.Position = UDim2.new(1, -220, 1, -100) -- Bottom Right
	frame.BackgroundColor3 = CONFIG.THEME.BG
	frame.BorderSizePixel = 0
	frame.Parent = screenGui
	
	-- Rounded Corners
	local corner = Instance.new("UICorner")
	corner.CornerRadius = UDim.new(0, 8)
	corner.Parent = frame
	
	-- Title
	local title = Instance.new("TextLabel")
	title.Size = UDim2.new(1, -20, 0, 30)
	title.Position = UDim2.new(0, 10, 0, 5)
	title.BackgroundTransparency = 1
	title.Text = "RoWinget Status"
	title.TextColor3 = CONFIG.THEME.ACCENT
	title.Font = Enum.Font.GothamBold
	title.TextSize = 14
	title.TextXAlignment = Enum.TextXAlignment.Left
	title.Parent = frame
	
	-- Status Text
	statusLabel = Instance.new("TextLabel")
	statusLabel.Name = "Status"
	statusLabel.Size = UDim2.new(1, -20, 0, 20)
	statusLabel.Position = UDim2.new(0, 10, 0, 35)
	statusLabel.BackgroundTransparency = 1
	statusLabel.Text = "Connecting..."
	statusLabel.TextColor3 = CONFIG.THEME.TEXT
	statusLabel.Font = Enum.Font.Gotham
	statusLabel.TextSize = 12
	statusLabel.TextXAlignment = Enum.TextXAlignment.Left
	statusLabel.Parent = frame
	
	-- Ping/Time
	local timeLabel = Instance.new("TextLabel")
	timeLabel.Name = "Time"
	timeLabel.Size = UDim2.new(1, -20, 0, 15)
	timeLabel.Position = UDim2.new(0, 10, 0, 55)
	timeLabel.BackgroundTransparency = 1
	timeLabel.Text = "Waiting..."
	timeLabel.TextColor3 = Color3.fromRGB(150, 150, 150)
	timeLabel.Font = Enum.Font.Code
	timeLabel.TextSize = 10
	timeLabel.TextXAlignment = Enum.TextXAlignment.Left
	timeLabel.Parent = frame
	
	-- Distribute to players
	local function giveToPlayer(player)
		if not player:FindFirstChild("PlayerGui") then return end
		if player.PlayerGui:FindFirstChild("RoWingetStatus") then return end
		screenGui:Clone().Parent = player.PlayerGui
	end
	
	Players.PlayerAdded:Connect(giveToPlayer)
	for _, p in ipairs(Players:GetPlayers()) do giveToPlayer(p) end
	
	-- Also put in StarterGui for auto-respawn
	screenGui.Parent = game:GetService("StarterGui")
	
	statusUI = screenGui
end

local function updateUI(connected, message, details)
	if not statusUI then return end
	
	-- Update local copy (StarterGui)
	-- Note: In a real server script, we need to update all clients' PlayerGui copies
	-- For simplicity in Studio "Run" mode, updating instances works if they are replicated
	
	local function updateFrame(gui)
		local frame = gui:FindFirstChild("StatusCard")
		if not frame then return end
		
		local lbl = frame:FindFirstChild("Status")
		local timeLbl = frame:FindFirstChild("Time")
		
		if connected then
			lbl.Text = "✅ Connected"
			lbl.TextColor3 = CONFIG.THEME.SUCCESS
		else
			lbl.Text = "❌ Disconnected"
			lbl.TextColor3 = CONFIG.THEME.FAIL
		end
		
		if details then
			timeLbl.Text = details
		end
	end
	
	-- Update StarterGui (for new spawns)
	updateFrame(statusUI)
	
	-- Update existing players
	for _, player in ipairs(Players:GetPlayers()) do
		if player:FindFirstChild("PlayerGui") then
			local gui = player.PlayerGui:FindFirstChild("RoWingetStatus")
			if gui then updateFrame(gui) end
		end
	end
end

-- ============================================================================
-- LOGIC
-- ============================================================================

-- Test connection to Memory Manager
local function checkConnection()
	local start = tick()
	local success, result = pcall(function()
		return HttpService:GetAsync(CONFIG.URL .. "/health")
	end)
	
	local latency = math.floor((tick() - start) * 1000)

	if success then
		local data = HttpService:JSONDecode(result)
		-- print("✅ [RoWinget] Connected (" .. latency .. "ms)")
		updateUI(true, nil, "Ping: " .. latency .. "ms | Files: " .. (data.cachedFiles or 0))
		return true
	else
		warn("❌ [RoWinget] Connection Failed: " .. tostring(result))
		updateUI(false, nil, "Retrying in " .. CONFIG.INTERVAL .. "s...")
		return false
	end
end

-- ============================================================================
-- MAIN LOOP
-- ============================================================================

-- Initialize UI
pcall(createStatusUI)

-------------------------------------------------------------------------------
-- 🛑 CRITICAL FIX: PREVENT CLIENT ERRORS
-- Roblox Clients cannot use HttpService. This check prevents red error spam.
if RunService:IsClient() then
	print("ℹ️ RoWinget: Client Mode. UI will update via Server replication.")
	return
end
-------------------------------------------------------------------------------

print("🔄 Monitor Active. Check the bottom-right corner of the screen when playing.")

spawn(function()
	while true do
		checkConnection()
		wait(CONFIG.INTERVAL)
	end
end)
