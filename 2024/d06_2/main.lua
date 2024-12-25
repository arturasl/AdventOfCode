local function find_path(map, cur_y, cur_x, should_calc_visited)
    local dirs = {
        { dy = -1, dx = 0 },
        { dy = 0, dx = 1 },
        { dy = 1, dx = 0 },
        { dy = 0, dx = -1 },
    }
    local cur_dir = 1
    local visited_poses = {}
    local visited = {}

    while true do
        if should_calc_visited then
            visited_poses[string.format("%d %d", cur_y, cur_x)] = true
        end
        local state = string.format("%d %d %d", cur_y, cur_x, cur_dir)
        if visited[state] ~= nil then
            return true, visited_poses
        end
        visited[state] = true

        local next_y = cur_y + dirs[cur_dir].dy
        local next_x = cur_x + dirs[cur_dir].dx

        if map[next_y] == nil or map[next_y][next_x] == nil then
            return false, visited_poses
        elseif map[next_y][next_x] ~= "#" then
            cur_y, cur_x = next_y, next_x
        else
            cur_dir = (cur_dir % #dirs) + 1
        end
    end
end

local function read()
    local data = { map = {}, cur_y = nil, cur_x = nil }
    for line in io.lines() do
        line = line:gsub("^%s+", ""):gsub("%s+$", "")
        if line ~= "" then
            data.map[#data.map + 1] = {}
            for i = 1, #line do
                data.map[#data.map][i] = line:sub(i, i)
                assert(string.find(".#^", data.map[#data.map][i]) ~= nil, data.map[#data.map][i] .. ": " .. line)

                if data.map[#data.map][i] == "^" then
                    assert(data.cur_y == nil and data.cur_x == nil)
                    data.cur_y = #data.map
                    data.cur_x = i
                end
            end
        end
    end
    assert(data.cur_y ~= nil and data.cur_x ~= nil)
    return data
end

local function main()
    local data = read()
    local _, visited = find_path(data.map, data.cur_y, data.cur_x, true)
    local num_cycles = 0
    for visited_pos, _ in pairs(visited) do
        local str_y, str_x = visited_pos:match("(%d+) (%d+)")
        local y, x = tonumber(str_y), tonumber(str_x)

        data.map[y][x] = "#"
        local is_cycled, _ = find_path(data.map, data.cur_y, data.cur_x, false)
        data.map[y][x] = "."

        num_cycles = num_cycles + (is_cycled and 1 or 0)
    end
    print(num_cycles)
end

main()
