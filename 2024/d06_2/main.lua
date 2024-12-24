local function find_path(map, cur_y, cur_x)
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
        visited_poses[string.format("%d %d", cur_y, cur_x)] = true
        local state = string.format("%d %d %d %d", cur_y, cur_x, dirs[cur_dir].dy, dirs[cur_dir].dx)
        if visited[state] ~= nil then
            return true, visited_poses
        end
        visited[state] = true
        map[cur_y][cur_x] = "x"

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

local map = {}
local cur_y, cur_x = nil, nil
for line in io.lines() do
    line = line:gsub("^%s+", ""):gsub("%s+$", "")
    if line ~= "" then
        map[#map + 1] = {}
        for i = 1, #line do
            map[#map][i] = line:sub(i, i)
            assert(string.find(".#^", map[#map][i]) ~= nil, map[#map][i] .. ": " .. line)

            if map[#map][i] == "^" then
                assert(cur_y == nil)
                cur_y = #map
                cur_x = i
            end
        end
    end
end

assert(cur_y ~= nil)

local _, visited = find_path(map, cur_y, cur_x)
local num_cycles = 0
for visited_pos, _ in pairs(visited) do
    local str_y, str_x = visited_pos:match("(%d+) (%d+)")
    local y, x = tonumber(str_y), tonumber(str_x)

    map[y][x] = "#"
    local is_cycled, _ = find_path(map, cur_y, cur_x)
    map[y][x] = "."

    num_cycles = num_cycles + (is_cycled and 1 or 0)
end

print(num_cycles)
