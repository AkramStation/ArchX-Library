-- UI Manager Module for Roblox Plugin
-- Creates and manages the DockWidget GUI

local UIManager = {}
UIManager.__index = UIManager

function UIManager.new(plugin)
    local self = setmetatable({}, UIManager)
    self.plugin = plugin
    self.widget = nil
    self.isConnected = false
    self.statusLabel = nil
    self.portInput = nil
    
    return self
end

function UIManager:createWidget()
    -- Create DockWidget
    local widgetInfo = DockWidgetPluginGuiInfo.new(
        Enum.InitialDockState.Right,
        false,   -- Initially enabled
        false,   -- Override previous enabled state
        300,     -- Float size X
        400,     -- Float size Y
        250,     -- Min width
        300      -- Min height
    )
    
    self.widget = self.plugin:CreateDockWidgetPluginGui("RobloxManagerWidget", widgetInfo)
    self.widget.Title = "Roblox Manager"
    self.widget.ZIndexBehavior = Enum.ZIndexBehavior.Sibling
    
    -- Create UI Elements
    self:createUI()
    
    return self.widget
end

function UIManager:createUI()
    -- Main Frame
    local mainFrame = Instance.new("Frame")
    mainFrame.Name = "MainFrame"
    mainFrame.Size = UDim2.new(1, 0, 1, 0)
    mainFrame.BackgroundColor3 = Color3.fromRGB(30, 39, 46) -- Dark Void
    mainFrame.BorderSizePixel = 0
    mainFrame.Parent = self.widget
    
    -- Header
    local header = Instance.new("Frame")
    header.Name = "Header"
    header.Size = UDim2.new(1, 0, 0, 60)
    header.BackgroundColor3 = Color3.fromRGB(47, 54, 64) -- Slate
    header.BorderSizePixel = 0
    header.Parent = mainFrame
    
    -- Title
    local title = Instance.new("TextLabel")
    title.Name = "Title"
    title.Size = UDim2.new(1, -20, 0, 30)
    title.Position = UDim2.new(0, 10, 0, 5)
    title.BackgroundTransparency = 1
    title.Text = "RoWinget"
    title.TextColor3 = Color3.fromRGB(0, 168, 255) -- Electric Blue

    title.TextSize = 18
    title.Font = Enum.Font.SourceSansBold
    title.TextXAlignment = Enum.TextXAlignment.Left
    title.Parent = header
    
    -- Status Indicator
    local statusFrame = Instance.new("Frame")
    statusFrame.Name = "StatusFrame"
    statusFrame.Size = UDim2.new(1, -20, 0, 20)
    statusFrame.Position = UDim2.new(0, 10, 0, 35)
    statusFrame.BackgroundColor3 = Color3.fromRGB(189, 147, 249, 60)
    statusFrame.BorderColor3 = Color3.fromRGB(189, 147, 249)
    statusFrame.BorderSizePixel = 1
    statusFrame.Parent = header
    
    -- Corner for status frame
    local corner1 = Instance.new("UICorner")
    corner1.CornerRadius = UDim.new(0, 4)
    corner1.Parent = statusFrame
    
    -- Status Dot
    local statusDot = Instance.new("Frame")
    statusDot.Name = "StatusDot"
    statusDot.Size = UDim2.new(0, 8, 0, 8)
    statusDot.Position = UDim2.new(0, 8, 0.5, -4)
    statusDot.BackgroundColor3 = Color3.fromRGB(150, 150, 150)
    statusDot.BorderSizePixel = 0
    statusDot.Parent = statusFrame
    
    local dotCorner = Instance.new("UICorner")
    dotCorner.CornerRadius = UDim.new(1, 0)
    dotCorner.Parent = statusDot
    
    self.statusDot = statusDot
    
    -- Status Label
    self.statusLabel = Instance.new("TextLabel")
    self.statusLabel.Name = "StatusLabel"
    self.statusLabel.Size = UDim2.new(1, -24, 1, 0)
    self.statusLabel.Position = UDim2.new(0, 24, 0, 0)
    self.statusLabel.BackgroundTransparency = 1
    self.statusLabel.Text = "Not Connected"
    self.statusLabel.TextColor3 = Color3.fromRGB(200, 200, 200)
    self.statusLabel.TextSize = 12
    self.statusLabel.Font = Enum.Font.SourceSans
    self.statusLabel.TextXAlignment = Enum.TextXAlignment.Left
    self.statusLabel.Parent = statusFrame
    
    -- Content Frame
    local contentFrame = Instance.new("ScrollingFrame")
    contentFrame.Name = "ContentFrame"
    contentFrame.Size = UDim2.new(1, 0, 1, -60)
    contentFrame.Position = UDim2.new(0, 0, 0, 60)
    contentFrame.BackgroundTransparency = 1
    contentFrame.BorderSizePixel = 0
    contentFrame.ScrollBarThickness = 6
    contentFrame.CanvasSize = UDim2.new(0, 0, 0, 500)
    contentFrame.Parent = mainFrame
    
    local padding = Instance.new("UIPadding")
    padding.PaddingLeft = UDim.new(0, 10)
    padding.PaddingRight = UDim.new(0, 10)
    padding.PaddingTop = UDim.new(0, 10)
    padding.Parent = contentFrame
    
    local yOffset = 0
    
    -- Connection Section
    yOffset = self:createSection(contentFrame, "Connection", yOffset)
    
    -- Port Input
    local portLabel = Instance.new("TextLabel")
    portLabel.Size = UDim2.new(1, 0, 0, 20)
    portLabel.Position = UDim2.new(0, 0, 0, yOffset)
    portLabel.BackgroundTransparency = 1
    portLabel.Text = "VS Code Port:"
    portLabel.TextColor3 = Color3.fromRGB(200, 200, 200)
    portLabel.TextSize = 14
    portLabel.Font = Enum.Font.SourceSans
    portLabel.TextXAlignment = Enum.TextXAlignment.Left
    portLabel.Parent = contentFrame
    yOffset = yOffset + 25
    
    self.portInput = Instance.new("TextBox")
    self.portInput.Name = "PortInput"
    self.portInput.Size = UDim2.new(1, 0, 0, 30)
    self.portInput.Position = UDim2.new(0, 0, 0, yOffset)
    self.portInput.BackgroundColor3 = Color3.fromRGB(50, 50, 50)
    self.portInput.BorderColor3 = Color3.fromRGB(70, 70, 70)
    self.portInput.Text = "8080"
    self.portInput.TextColor3 = Color3.fromRGB(255, 255, 255)
    self.portInput.TextSize = 14
    self.portInput.Font = Enum.Font.SourceSans
    self.portInput.PlaceholderText = "Enter port number"
    self.portInput.Parent = contentFrame
    
    local portCorner = Instance.new("UICorner")
    portCorner.CornerRadius = UDim.new(0, 4)
    portCorner.Parent = self.portInput
    
    yOffset = yOffset + 40
    
    -- Connect Button
    self.connectButton = self:createButton(contentFrame, "Connect to VS Code", yOffset, Color3.fromRGB(0, 168, 255)) -- Electric Blue
    yOffset = yOffset + 45
    
    -- Disconnect Button
    self.disconnectButton = self:createButton(contentFrame, "Disconnect", yOffset, Color3.fromRGB(232, 65, 24)) -- Alert Red
    self.disconnectButton.Visible = false
    yOffset = yOffset + 45
    
    -- Actions Section
    yOffset = self:createSection(contentFrame, "Actions", yOffset + 10)
    
    -- Sync Button
    self.syncButton = self:createButton(contentFrame, "🔄 Sync Files", yOffset, Color3.fromRGB(156, 136, 255)) -- Cyber Violet
    self.syncButton.Active = false
    self.syncButton.BackgroundColor3 = Color3.fromRGB(70, 70, 70)
    yOffset = yOffset + 45
    
    -- Refresh Button
    self.refreshButton = self:createButton(contentFrame, "🔃 Refresh Status", yOffset, Color3.fromRGB(70, 70, 70)) -- Secondary Grey
    yOffset = yOffset + 45
    
    -- Stats Section
    yOffset = self:createSection(contentFrame, "Statistics", yOffset + 10)
    
    -- Stats Container
    local statsFrame = Instance.new("Frame")
    statsFrame.Size = UDim2.new(1, 0, 0, 120)
    statsFrame.Position = UDim2.new(0, 0, 0, yOffset)
    statsFrame.BackgroundColor3 = Color3.fromRGB(40, 40, 40)
    statsFrame.BorderColor3 = Color3.fromRGB(60, 60, 60)
    statsFrame.BorderSizePixel = 1
    statsFrame.Parent = contentFrame
    
    local statsCorner = Instance.new("UICorner")
    statsCorner.CornerRadius = UDim.new(0, 6)
    statsCorner.Parent = statsFrame
    
    local statsPadding = Instance.new("UIPadding")
    statsPadding.PaddingLeft = UDim.new(0, 12)
    statsPadding.PaddingRight = UDim.new(0, 12)
    statsPadding.PaddingTop = UDim.new(0, 10)
    statsPadding.Parent = statsFrame
    
    -- Stat Items
    self.fileCountLabel = self:createStatItem(statsFrame, "Synced Files:", "0", 0)
    self.lastSyncLabel = self:createStatItem(statsFrame, "Last Sync:", "Never", 30)
    self.statusTextLabel = self:createStatItem(statsFrame, "Status:", "Ready", 60)
end

function UIManager:createSection(parent, title, yPos)
    local section = Instance.new("TextLabel")
    section.Size = UDim2.new(1, 0, 0, 25)
    section.Position = UDim2.new(0, 0, 0, yPos)
    section.BackgroundTransparency = 1
    section.Text = title
    section.TextColor3 = Color3.fromRGB(255, 255, 255)
    section.TextSize = 16
    section.Font = Enum.Font.SourceSansBold
    section.TextXAlignment = Enum.TextXAlignment.Left
    section.Parent = parent
    
    return yPos + 30
end

function UIManager:createButton(parent, text, yPos, color)
    local button = Instance.new("TextButton")
    button.Size = UDim2.new(1, 0, 0, 35)
    button.Position = UDim2.new(0, 0, 0, yPos)
    button.BackgroundColor3 = color
    button.Text = text
    button.TextColor3 = Color3.fromRGB(20, 20, 20)
    button.TextSize = 14
    button.Font = Enum.Font.SourceSansBold
    button.BorderSizePixel = 0
    button.Parent = parent
    
    local corner = Instance.new("UICorner")
    corner.CornerRadius = UDim.new(0, 4)
    corner.Parent = button
    
    -- Hover effect
    button.MouseEnter:Connect(function()
        button.BackgroundColor3 = Color3.fromRGB(
            math.min(color.R * 255 * 1.1, 255),
            math.min(color.G * 255 * 1.1, 255),
            math.min(color.B * 255 * 1.1, 255)
        )
    end)
    
    button.MouseLeave:Connect(function()
        button.BackgroundColor3 = color
    end)
    
    return button
end

function UIManager:createStatItem(parent, label, value, yPos)
    local container = Instance.new("Frame")
    container.Size = UDim2.new(1, 0, 0, 20)
    container.Position = UDim2.new(0, 0, 0, yPos)
    container.BackgroundTransparency = 1
    container.Parent = parent
    
    local labelText = Instance.new("TextLabel")
    labelText.Size = UDim2.new(0.5, 0, 1, 0)
    labelText.BackgroundTransparency = 1
    labelText.Text = label
    labelText.TextColor3 = Color3.fromRGB(150, 150, 150)
    labelText.TextSize = 12
    labelText.Font = Enum.Font.SourceSans
    labelText.TextXAlignment = Enum.TextXAlignment.Left
    labelText.Parent = container
    
    local valueText = Instance.new("TextLabel")
    valueText.Size = UDim2.new(0.5, 0, 1, 0)
    valueText.Position = UDim2.new(0.5, 0, 0, 0)
    valueText.BackgroundTransparency = 1
    valueText.Text = value
    valueText.TextColor3 = Color3.fromRGB(255, 255, 255)
    valueText.TextSize = 12
    valueText.Font = Enum.Font.SourceSansBold
    valueText.TextXAlignment = Enum.TextXAlignment.Right
    valueText.Parent = container
    
    return valueText
end

function UIManager:updateStatus(message, statusType)
    self.statusLabel.Text = message
    
    if statusType == "connected" then
        self.statusDot.BackgroundColor3 = Color3.fromRGB(76, 209, 55) -- Neo Green
        self.statusLabel.Parent.BackgroundColor3 = Color3.fromRGB(76, 209, 55) -- Neo Green (BackgroundTransparency handles alpha)
        self.statusLabel.Parent.BorderColor3 = Color3.fromRGB(76, 209, 55)
    elseif statusType == "error" then
        self.statusDot.BackgroundColor3 = Color3.fromRGB(232, 65, 24) -- Alert Red
        self.statusLabel.Parent.BackgroundColor3 = Color3.fromRGB(232, 65, 24)
        self.statusLabel.Parent.BorderColor3 = Color3.fromRGB(232, 65, 24)
    elseif statusType == "loading" then
        self.statusDot.BackgroundColor3 = Color3.fromRGB(156, 136, 255) -- Cyber Violet
        self.statusLabel.Parent.BackgroundColor3 = Color3.fromRGB(156, 136, 255)
        self.statusLabel.Parent.BorderColor3 = Color3.fromRGB(156, 136, 255)
    else
        self.statusDot.BackgroundColor3 = Color3.fromRGB(150, 150, 150)
        self.statusLabel.Parent.BackgroundColor3 = Color3.fromRGB(70, 70, 70)
        self.statusLabel.Parent.BorderColor3 = Color3.fromRGB(100, 100, 100)
    end
end

function UIManager:setConnected(connected)
    self.isConnected = connected
    self.connectButton.Visible = not connected
    self.disconnectButton.Visible = connected
    self.syncButton.Active = connected
    
    if connected then
        self.syncButton.BackgroundColor3 = Color3.fromRGB(156, 136, 255) -- Cyber Violet
    else
        self.syncButton.BackgroundColor3 = Color3.fromRGB(70, 70, 70)
    end
end

function UIManager:updateStats(fileCount, lastSync)
    self.fileCountLabel.Text = tostring(fileCount)
    self.lastSyncLabel.Text = lastSync or "Never"
end

function UIManager:show()
    if self.widget then
        self.widget.Enabled = true
    end
end

function UIManager:hide()
    if self.widget then
        self.widget.Enabled = false
    end
end

function UIManager:toggle()
    if self.widget then
        self.widget.Enabled = not self.widget.Enabled
    end
end

return UIManager
