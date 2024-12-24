local heap = require("binaryheap")
local mk = require("multikey")

local function read()
    local coords = mk:new()
    local time = 1
    for line in io.lines() do
        line = line:gsub("%s+", "")
        if line ~= "" then
            local x, y = line:match("(%d+),(%d+)")
            y, x = tonumber(y), tonumber(x)
            coords:put(y, x, coords:get(y, x) or time)
            time = time + 1
        end
    end
    return coords
end

local function add_pos(lhs, rhs)
    return { y = lhs.y + rhs.y, x = lhs.x + rhs.x }
end

local function same_pos(lhs, rhs)
    return lhs.y == rhs.y and lhs.x == rhs.x
end

local function is_dropped(coords, coord, after_first)
    return (coords:get(coord.y, coord.x) or (after_first + 1)) <= after_first
end

local function is_reachable(coords, height, width, after_first)
    local move_to_d = {
        { y = 0, x = 1 },
        { y = 1, x = 0 },
        { y = 0, x = -1 },
        { y = -1, x = 0 },
    }

    local queue = heap.minHeap(function(lhs, rhs)
        return lhs.cost < rhs.cost
    end)
    local visited = mk:new()

    queue:insert({
        cost = 0,
        pos = { y = 0, x = 0 },
    })
    visited:put(0, 0, true)

    while true do
        local cur_state = queue:pop()
        if cur_state == nil then
            return false
        end

        if same_pos(cur_state.pos, { y = height, x = width }) then
            return true
        end

        for _, delta in ipairs(move_to_d) do
            local next_state = {
                cost = cur_state.cost + 1,
                pos = add_pos(cur_state.pos, delta),
            }

            if
                (0 <= next_state.pos.y and next_state.pos.y <= height)
                and (0 <= next_state.pos.x and next_state.pos.x <= width)
                and visited:get(next_state.pos.y, next_state.pos.x) == nil
                and not is_dropped(coords, next_state.pos, after_first)
            then
                visited:put(next_state.pos.y, next_state.pos.x, true)
                queue:insert(next_state)
            end
        end
    end
end

local function calc_max_time(coords)
    local max_time = 0
    for _, _, _, time in coords:tuples() do
        max_time = math.max(max_time, time)
    end
    return max_time
end

local function main()
    local coords = read()
    -- local width, height = 6, 6
    local height, width = 70, 70

    local low, high = 0, calc_max_time(coords)
    while low ~= high do
        local mid = low + (high - low) // 2
        if is_reachable(coords, height, width, mid) then
            low = mid + 1
        else
            high = mid
        end
    end

    for _, y, x, time in coords:tuples() do
        if time == low then
            print(("%d,%d"):format(x, y))
        end
    end
end

main()
