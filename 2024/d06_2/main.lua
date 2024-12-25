local function find_path(map, cur_y, cur_x, cur_dir, should_calc_visited)
    local dirs = {
        { dy = -1, dx = 0 },
        { dy = 0, dx = 1 },
        { dy = 1, dx = 0 },
        { dy = 0, dx = -1 },
    }
    local visited_poses = {}
    local visited = {}

    while true do
        if should_calc_visited then
            visited_poses[#visited_poses + 1] = { y = cur_y, x = cur_x, dir = cur_dir }
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
    local _, visited = find_path(data.map, data.cur_y, data.cur_x, 1, true)
    local num_cycles = 0

    local check = {}
    for i, pos in ipairs(visited) do
        local str_pos = string.format("%d %d", pos.y, pos.x)
        if check[str_pos] == nil and i > 1 then
            check[str_pos] = true
            data.map[pos.y][pos.x] = "#"
            local is_cycled, _ = find_path(data.map, visited[i - 1].y, visited[i - 1].x, visited[i - 1].dir, false)
            data.map[pos.y][pos.x] = "."

            num_cycles = num_cycles + (is_cycled and 1 or 0)
        end
    end
    print(num_cycles)
end

main()
