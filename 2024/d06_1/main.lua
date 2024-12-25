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

local directions = {
    { dy = -1, dx = 0 },
    { dy = 0, dx = 1 },
    { dy = 1, dx = 0 },
    { dy = 0, dx = -1 },
}
local cur_dir = 1

for _ = 1, #map * #map[1] do
    map[cur_y][cur_x] = "x"

    local next_y = cur_y + directions[cur_dir].dy
    local next_x = cur_x + directions[cur_dir].dx

    if map[next_y] == nil or map[next_y][next_x] == nil then
        break
    elseif map[next_y][next_x] ~= "#" then
        cur_y, cur_x = next_y, next_x
    else
        cur_dir = (cur_dir + 1) % (#directions + 1)
        cur_dir = cur_dir == 0 and 1 or cur_dir
    end
end

local walked = 0
for i = 1, #map do
    for j = 1, #map[i] do
        walked = walked + (map[i][j] == "x" and 1 or 0)
    end
end

print(walked)
