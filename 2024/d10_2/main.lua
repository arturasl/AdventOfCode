local dump = require("dump")

local function sz(tbl)
    local s = 0
    for _, _ in pairs(tbl) do
        s = s + 1
    end
    return s
end

local function read()
    local map = { width = nil, height = nil, cells = {} }

    for line in io.lines() do
        line = line:gsub("%s+", "")
        if line ~= "" then
            map.cells[#map.cells + 1] = {}
            for i = 1, #line do
                map.cells[#map.cells][i] = tonumber(line:sub(i, i))
            end
        end
    end

    map.height = #map.cells
    map.width = #map.cells[1]

    return map
end

local function traverse(y, x, map, dp)
    if dp[y][x] ~= nil then
        return dp[y][x]
    end

    dp[y][x] = 0
    if map.cells[y][x] == 9 then
        dp[y][x] = 1
    end

    for _, next in ipairs({
        { y = y - 1, x = x },
        { y = y + 1, x = x },
        { y = y, x = x - 1 },
        { y = y, x = x + 1 },
    }) do
        if 1 <= next.y and next.y <= map.height and 1 <= next.x and next.x <= map.width then
            if map.cells[y][x] == map.cells[next.y][next.x] - 1 then
                dp[y][x] = dp[y][x] + traverse(next.y, next.x, map, dp)
            end
        end
    end

    return dp[y][x]
end

local function main()
    local map = read()

    local dp = {}
    for y = 1, map.height do
        dp[y] = {}
    end

    local result = 0
    for y = 1, map.height do
        for x = 1, map.width do
            traverse(y, x, map, dp)

            if map.cells[y][x] == 0 then
                result = result + dp[y][x]
            end
        end
    end

    print(result)
end

main()
