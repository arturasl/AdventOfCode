local mk = require("multikey")

local function read()
    local data = { map = {}, robot = { y = nil, x = nil }, moves = {}, step = 0 }
    for line in io.lines() do
        line = line:gsub("%s+", "")
        if line ~= "" then
            if string.find("<>v^", line:sub(1, 1), 1, true) ~= nil then
                for i = 1, #line do
                    data.moves[#data.moves + 1] = line:sub(i, i)
                    assert(string.find("<>v^", data.moves[#data.moves], 1, true))
                end
            else
                data.map[#data.map + 1] = {}
                for i = 1, #line do
                    local char = line:sub(i, i)
                    if char == "@" then
                        assert(data.robot.y == nil and data.robot.x == nil)
                        data.robot.y = #data.map
                        data.robot.x = i * 2 - 1
                        char = "."
                    end

                    assert(string.find("#.O", char, 1, true) ~= nil, char)
                    if string.find("#.", char, 1, true) ~= nil then
                        table.insert(data.map[#data.map], char)
                        table.insert(data.map[#data.map], char)
                    else
                        table.insert(data.map[#data.map], "[")
                        table.insert(data.map[#data.map], "]")
                    end
                end
            end
        end
    end
    return data
end

local function visit(cur, dir, data, visited)
    if data.map[cur.y][cur.x] == "#" then
        return false
    end
    if data.map[cur.y][cur.x] == "." then
        return true
    end

    if visited:get(cur.y, cur.x) ~= nil then
        return true
    end
    visited:put(cur.y, cur.x, data.map[cur.y][cur.x])

    if data.map[cur.y][cur.x] == "[" and not visit({ y = cur.y, x = cur.x + 1 }, dir, data, visited) then
        return false
    end
    if data.map[cur.y][cur.x] == "]" and not visit({ y = cur.y, x = cur.x - 1 }, dir, data, visited) then
        return false
    end

    return visit({ y = cur.y + dir.y, x = cur.x + dir.x }, dir, data, visited)
end

local function cal_result(data)
    local result = 0
    for y = 1, #data.map do
        for x = 1, #data.map[y] do
            if data.map[y][x] == "[" then
                result = result + (y - 1) * 100 + (x - 1)
            end
        end
    end
    return result
end

local function main()
    local data = read()

    local move_to_d = {
        ["v"] = { y = 1, x = 0 },
        ["^"] = { y = -1, x = 0 },
        ["<"] = { y = 0, x = -1 },
        [">"] = { y = 0, x = 1 },
    }

    while true do
        data.step = data.step + 1
        if data.step == #data.moves + 1 then
            break
        end
        local d = move_to_d[data.moves[data.step]]

        local visited = mk:new()
        if visit({ y = data.robot.y + d.y, x = data.robot.x + d.x }, d, data, visited) then
            data.robot.y, data.robot.x = data.robot.y + d.y, data.robot.x + d.x
            for _, y, x, _ in visited:tuples() do
                data.map[y][x] = "."
            end
            for _, y, x, v in visited:tuples() do
                data.map[y + d.y][x + d.x] = v
            end
        end
    end

    print(cal_result(data))
end

main()
