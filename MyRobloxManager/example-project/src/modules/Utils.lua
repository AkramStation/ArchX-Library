-- Utils.lua
-- Example utility module

local Utils = {}

-- Math utilities
function Utils.round(number, decimals)
    local mult = 10 ^ (decimals or 0)
    return math.floor(number * mult + 0.5) / mult
end

function Utils.clamp(value, min, max)
    return math.max(min, math.min(max, value))
end

function Utils.lerp(a, b, t)
    return a + (b - a) * t
end

-- String utilities
function Utils.capitalize(str)
    return str:sub(1, 1):upper() .. str:sub(2):lower()
end

function Utils.split(str, delimiter)
    local result = {}
    local pattern = string.format("([^%s]+)", delimiter)
    
    for match in string.gmatch(str, pattern) do
        table.insert(result, match)
    end
    
    return result
end

-- Table utilities
function Utils.deepCopy(original)
    local copy
    
    if type(original) == "table" then
        copy = {}
        for key, value in next, original, nil do
            copy[Utils.deepCopy(key)] = Utils.deepCopy(value)
        end
        setmetatable(copy, Utils.deepCopy(getmetatable(original)))
    else
        copy = original
    end
    
    return copy
end

function Utils.tableContains(tbl, value)
    for _, v in pairs(tbl) do
        if v == value then
            return true
        end
    end
    return false
end

function Utils.tableLength(tbl)
    local count = 0
    for _ in pairs(tbl) do
        count = count + 1
    end
    return count
end
 --- test
-- Time utilities
function Utils.formatTime(seconds)
    local minutes = math.floor(seconds / 60)
    local secs = seconds % 60
    return string.format("%02d:%02d", minutes, secs)
end

function Utils.getTimestamp()
    return os.time()
end

-- Random utilities
function Utils.randomChance(percentage)
    return math.random(100) <= percentage
end

function Utils.randomElement(tbl)
    return tbl[math.random(#tbl)]
end

-- Debug utilities
function Utils.printTable(tbl, indent)
    indent = indent or 0
    local spacing = string.rep("  ", indent)
    
    for key, value in pairs(tbl) do
        if type(value) == "table" then
            print(spacing .. tostring(key) .. ": {")
            Utils.printTable(value, indent + 1)
            print(spacing .. "}")
        else
            print(spacing .. tostring(key) .. ": " .. tostring(value))
        end
    end
end

return Utils
