local inspect = require("inspect")

local function read()
    local data = { available = {}, test = {} }
    for line in io.lines() do
        line = line:gsub("%s+", "")
        if line ~= "" then
            if #data.available == 0 then
                for pattern in line:gmatch("[^,]+") do
                    table.insert(data.available, pattern)
                end
            else
                table.insert(data.test, line)
            end
        end
    end
    return data
end

local available = nil
local function can_make(pattern)
    if pattern == "" then
        return true
    end

    for _, use in ipairs(available) do
        if pattern:match("^" .. use) and can_make(pattern:gsub("^" .. use, "")) then
            return true
        end
    end

    return false
end
can_make = require("multikey.memoize")(can_make)

local function main()
    local data = read()
    available = data.available

    local result = 0
    for _, pattern in ipairs(data.test) do
        local ok = can_make(pattern)
        print(pattern, ok)
        result = result + (ok and 1 or 0)
    end

    print(result)
end

main()
