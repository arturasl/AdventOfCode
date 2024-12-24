local heap = require("binaryheap")
local inspect = require("inspect")
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

local function join_arr(arr, sep)
    local result = ""
    for idx, val in ipairs(arr) do
        if idx ~= 1 then
            result = result .. sep
        end
        result = result .. tostring(val)
    end
    return result
end

local function key_poses(pad)
    local result = {}
    for y = 1, #pad do
        for x = 1, #pad[y] do
            if pad[y][x] ~= "#" then
                result[pad[y][x]] = { y = y, x = x }
            end
        end
    end
    return result
end

local move_to_d = {
    [">"] = { y = 0, x = 1 },
    ["v"] = { y = 1, x = 0 },
    ["<"] = { y = 0, x = -1 },
    ["^"] = { y = -1, x = 0 },
}
local key_pad_key_to_pos = key_poses(key_pad)
local num_pad_key_to_pos = key_poses(num_pad)

local function execute(pad_idx, cur_keys, order)
    local pad = pad_idx == 1 and num_pad or key_pad
    local key_to_pos = pad_idx == 1 and num_pad_key_to_pos or key_pad_key_to_pos
    local cur_key = cur_keys[pad_idx]

    if order == "A" then
        if pad_idx == 1 then
            return cur_key
        else
            return execute(pad_idx - 1, cur_keys, cur_key)
        end
    else
        local pos = add_pos(key_to_pos[cur_key], move_to_d[order])
        if not (1 <= pos.y and pos.y <= #pad and 1 <= pos.x and pos.x <= #pad[pos.y] and pad[pos.y][pos.x] ~= "#") then
            return nil
        end
        cur_keys[pad_idx] = pad[pos.y][pos.x]
        return ""
    end
end

local function solve(test)
    local queue = heap.minHeap(function(lhs, rhs)
        return lhs.cost < rhs.cost
    end)

    local robots = 2 + 1
    local init_keys = {}
    for _ = 1, robots do
        table.insert(init_keys, "A")
    end
    queue:insert({
        cost = 0,
        cur_keys = init_keys,
        print_char_idx = 1,
    })
    local visited = mk:new()
    visited:put(join_arr(init_keys, ""), 1, true)

    while true do
        local cur_state = queue:pop()
        assert(cur_state ~= nil)

        if cur_state.print_char_idx == #test + 1 then
            return cur_state.cost
        end

        for _, key in ipairs({ "A", "<", ">", "v", "^" }) do
            local next_state = {}
            next_state.cur_keys = cpy_arr(cur_state.cur_keys)
            next_state.print_char_idx = cur_state.print_char_idx
            next_state.cost = cur_state.cost + 1

            local exec_result = execute(robots, next_state.cur_keys, key)
            if exec_result ~= nil then
                if exec_result == test[next_state.print_char_idx] then
                    next_state.print_char_idx = next_state.print_char_idx + 1
                    exec_result = ""
                end

                local serialized_keys = join_arr(next_state.cur_keys, "")
                if exec_result == "" and visited:get(serialized_keys, next_state.print_char_idx) == nil then
                    visited:put(serialized_keys, next_state.print_char_idx, true)
                    queue:insert(next_state)
                end
            end
        end
    end
end

local function main()
    local tests = read()
    local result = 0
    for _, test in ipairs(tests) do
        local cost = solve(test)
        local num_test = join_arr(test, ""):gsub("[^%d]", ""):gsub("^0+", "")
        result = result + cost * tonumber(num_test)
    end
    print(result)
end

main()
