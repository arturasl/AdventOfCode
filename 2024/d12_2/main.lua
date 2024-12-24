local dump = require("dump")

local function visit(y, x, expected, map, visited, perimeter)
    assert(not visited[y][x])
    assert(map[y][x] == expected)

    visited[y][x] = true
    local area = 1

    for _, d in ipairs({ { y = -1, x = 0 }, { y = 1, x = 0 }, { y = 0, x = -1 }, { y = 0, x = 1 } }) do
        local ny, nx = y + d.y, x + d.x
        if not (1 <= ny and ny <= #map) or not (1 <= nx and nx <= #map[1]) or map[ny][nx] ~= expected then
            local d1 = math.abs(d.y)
            local d2 = 2 * y * math.abs(d.y) + d.y + 2 * x * math.abs(d.x) + d.x
            perimeter[d1][d2] = perimeter[d1][d2] or {}
            table.insert(perimeter[d1][d2], math.abs(d.y) == 1 and x or y)
        elseif not visited[ny][nx] then
            area = area + visit(ny, nx, expected, map, visited, perimeter)
        end
    end

    return area
end

local function count_crosses(perimeter)
    local crosses = 0

    for trans_x, data in pairs(perimeter[0]) do
        for i = 2, #data.orig_poses do
            if data.orig_poses[i - 1] + 1 == data.orig_poses[i] then
                local ref = (perimeter[1][data.orig_poses[i - 1] * 2 + 1] or {}).lookup_poses or {}
                if ref[trans_x // 2] ~= nil and ref[trans_x // 2 + 1] ~= nil then
                    crosses = crosses + 1
                end
            end
        end
    end

    return crosses
end

local function count_sides(perimeter)
    local sides = 0

    for _, trans_pos_to_data in pairs(perimeter) do
        for _, data in pairs(trans_pos_to_data) do
            sides = sides + 1
            for i = 2, #data.orig_poses do
                sides = sides + (data.orig_poses[i] - data.orig_poses[i - 1] > 1 and 1 or 0)
            end
        end
    end

    return sides
end

local function simplify_perimeter(perimeter)
    local result = {}
    for dir, trans_pos_to_orig_poses in pairs(perimeter) do
        result[dir] = result[dir] or {}
        for trans_pos, origs_poses in pairs(trans_pos_to_orig_poses) do
            result[dir][trans_pos] = {
                orig_poses = {},
                lookup_poses = {},
            }
            local ref = result[dir][trans_pos]
            for _, orig_pos in ipairs(origs_poses) do
                table.insert(ref.orig_poses, orig_pos)
                ref.lookup_poses[orig_pos] = true
            end
            table.sort(ref.orig_poses)
        end
    end
    return result
end

local function main()
    local map = {}
    for line in io.lines() do
        map[#map + 1] = {}
        for i = 1, #line do
            map[#map][i] = line:sub(i, i)
        end
    end

    local visited = {}
    for i = 1, #map do
        visited[i] = {}
    end

    local result = 0
    for y = 1, #map do
        for x = 1, #map[y] do
            if not visited[y][x] then
                local perimeter = { [0] = {}, [1] = {} }
                local area = visit(y, x, map[y][x], map, visited, perimeter)
                perimeter = simplify_perimeter(perimeter)
                result = result + area * (count_sides(perimeter) + 2 * count_crosses(perimeter))
            end
        end
    end
    print(result)
end

main()
