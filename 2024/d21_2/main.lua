local heap = require("binaryheap")
local mk = require("multikey")

local num_pad = {
    { "7", "8", "9" },
    { "4", "5", "6" },
    { "1", "2", "3" },
    { "#", "0", "A" },
}
local key_pad = {
    { "#", "^", "A" },
    { "<", "v", ">" },
}

local function read()
    local tests = {}
    for line in io.lines() do
        line = line:gsub("%s+", "")
        if line ~= "" then
            tests[#tests + 1] = {}
            for i = 1, #line do
                tests[#tests][i] = line:sub(i, i)
            end
        end
    end
    return tests
end

local function same_pos(lhs, rhs)
    return lhs.y == rhs.y and lhs.x == rhs.x
end

local function add_pos(lhs, rhs)
    return { y = lhs.y + rhs.y, x = lhs.x + rhs.x }
end

local function cpy_arr(arr)
    local cpy = {}
    for i = 1, #arr do
        cpy[i] = arr[i]
    end
    return cpy
end

local function append_arr(arr, el)
    arr[#arr + 1] = el
    return arr
end

local function join_arr(start, arr, sep)
    local result = ""
    for idx = start, #arr do
        if idx ~= start then
            result = result .. sep
        end
        result = result .. tostring(arr[idx])
    end
    return result
end

local function find_shortest_paths(pad, start_pos, end_pos)
    local move_to_d = {
        { char = ">", y = 0, x = 1 },
        { char = "v", y = 1, x = 0 },
        { char = "<", y = 0, x = -1 },
        { char = "^", y = -1, x = 0 },
    }

    local queue = heap.minHeap(function(lhs, rhs)
        return lhs.cost < rhs.cost
    end)

    queue:insert({ cost = 0, pos = start_pos, path = {} })
    local visited = mk:new()
    visited:put(start_pos.y, start_pos.x, 0)

    local result = {}

    while true do
        local cur_state = queue:pop()
        if cur_state == nil then
            break
        end

        if same_pos(cur_state.pos, end_pos) then
            result[#result + 1] = append_arr(cur_state.path, "A")
        end

        for _, delta in ipairs(move_to_d) do
            local next_state = {
                cost = cur_state.cost + 1,
                pos = add_pos(cur_state.pos, delta),
                path = append_arr(cpy_arr(cur_state.path), delta.char),
            }

            if
                (1 <= next_state.pos.y and next_state.pos.y <= #pad)
                and (1 <= next_state.pos.x and next_state.pos.x <= #pad[next_state.pos.y])
                and pad[next_state.pos.y][next_state.pos.x] ~= "#"
                and (visited:get(next_state.pos.y, next_state.pos.x) or (next_state.cost + 1)) >= next_state.cost
            then
                visited:put(next_state.pos.y, next_state.pos.x, next_state.cost)
                queue:insert(next_state)
            end
        end
    end

    return result
end

local function find_all_paths(pad)
    local result = {}
    for sy = 1, #pad do
        for sx = 1, #pad[sy] do
            if pad[sy][sx] ~= "#" then
                result[pad[sy][sx]] = {}
                for ey = 1, #pad do
                    for ex = 1, #pad[ey] do
                        if pad[ey][ex] ~= "#" then
                            result[pad[sy][sx]][pad[ey][ex]] = find_shortest_paths(
                                pad,
                                { y = sy, x = sx },
                                { y = ey, x = ex }
                            )
                        end
                    end
                end
            end
        end
    end
    return result
end

local num_pad_paths = find_all_paths(num_pad)
local key_pad_paths = find_all_paths(key_pad)
-- num_pads = 6 + 1; lua main.lua < large.in -- 5691960
local num_pads = 25 + 1
local cache = {}

local function execute(cur_keys, pad_idx, key)
    if pad_idx > num_pads then
        return { keys = cpy_arr(cur_keys), len = 1 }
    end

    local cache_key = join_arr(pad_idx, cur_keys, "") .. tostring(pad_idx) .. key
    if cache[cache_key] ~= nil then
        return cache[cache_key]
    end

    local cur_paths = pad_idx == 1 and num_pad_paths or key_pad_paths
    local cur_key = cur_keys[pad_idx]

    local best_len = nil
    local best_keys = nil

    for _, additional_path in ipairs(cur_paths[cur_key][key]) do
        local local_len = 0
        local local_keys = cpy_arr(cur_keys)

        for _, additional_key in ipairs(additional_path) do
            local exec_result = execute(cur_keys, pad_idx + 1, additional_key)
            cur_keys = exec_result.keys
            local_len = local_len + exec_result.len
        end

        if best_len == nil or best_len > local_len then
            best_len = local_len
            best_keys = local_keys
        end
    end

    best_keys[pad_idx] = key
    local result = { keys = best_keys, len = best_len }
    cache[cache_key] = result
    return result
end

local function main()
    local tests = read()

    local result = 0
    for _, test in ipairs(tests) do
        local cur_keys = {}
        for _ = 1, num_pads do
            cur_keys[#cur_keys + 1] = "A"
        end

        local local_len = 0
        for _, key in ipairs(test) do
            local exec_result = execute(cur_keys, 1, key)
            cur_keys = exec_result.keys
            local_len = local_len + exec_result.len
        end

        local str_numeric = ""
        for str_num in join_arr(1, test, ""):gsub("^0+", ""):gmatch("(%d+)") do
            str_numeric = str_numeric .. str_num
        end

        print(("%s = %d * %s"):format(join_arr(1, test, ""), local_len, str_numeric))
        result = result + local_len * tonumber(str_numeric)
    end
    print(result)
end

main()
