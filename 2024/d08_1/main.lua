local dump = require("dump")

local function sz(tbl)
    local total = 0
    for _, _ in pairs(tbl) do
        total = total + 1
    end
    return total
end

local function read()
    local result = { height = 0, map = {}, antenas = {} }
    for line in io.lines() do
        line = line:gsub("^%s+", ""):gsub("%s+$", "")
        if line ~= "" then
            result.width = result.width or #line
            result.height = result.height + 1
            result.map[#result.map + 1] = {}

            for i = 1, #line do
                local char = line:sub(i, i)
                result.map[#result.map][i] = char
                if char ~= "." then
                    result.antenas[#result.antenas + 1] = {
                        y = result.height,
                        x = i,
                        name = char,
                    }
                end
            end
        end
    end

    return result
end

local function main()
    local data = read()
    local antinodes = {}
    for y = 1, data.height do
        for x = 1, data.width do
            for _, antena in ipairs(data.antenas) do
                if antena.y ~= y or antena.x ~= x then
                    local ny = 2 * antena.y - y
                    local nx = 2 * antena.x - x
                    if data.map[ny] ~= nil and data.map[ny][nx] == antena.name then
                        antinodes[string.format("%d %d", y, x)] = true
                    end
                end
            end
        end
    end

    print(dump(antinodes))
    print(sz(antinodes))
end

main()
