local function add_edge(graph, from, to)
    graph[from] = graph[from] or {}
    graph[from][to] = true
    graph[from][from] = true
end

local function join_arr(arr, sep)
    local result = ""
    for idx = 1, #arr do
        if idx ~= 1 then
            result = result .. sep
        end
        result = result .. tostring(arr[idx])
    end
    return result
end

local function read()
    local graph = {}
    for line in io.lines() do
        line = line:gsub("%s+", "")
        if line ~= "" then
            local lhs, rhs = line:match("([^-]+)-([^-]+)")
            add_edge(graph, lhs, rhs)
            add_edge(graph, rhs, lhs)
        end
    end
    return graph
end

local function main()
    local graph = read()

    local best_cnt = 0
    local best_node_arr = {}

    local tried_to_grow_around = {}

    for from, set_to in pairs(graph) do
        local arr_to = {}
        for to, _ in pairs(set_to) do
            if tried_to_grow_around[to] == nil then
                arr_to[#arr_to + 1] = to
            end
        end
        tried_to_grow_around[from] = true

        if best_cnt < #arr_to then
            for i = 1, (1 << #arr_to) - 1 do
                local arr_subset = {}
                local contains_self = false

                for j = 0, #arr_to do
                    if (i & (1 << j)) ~= 0 then
                        arr_subset[#arr_subset + 1] = arr_to[j + 1]
                        contains_self = contains_self or arr_to[j + 1] == from
                    end
                end

                if contains_self and best_cnt < #arr_subset then
                    local ok = true
                    for lhs_idx = 1, #arr_subset do
                        for rhs_idx = lhs_idx + 1, #arr_subset do
                            ok = ok and graph[arr_subset[lhs_idx]][arr_subset[rhs_idx]] ~= nil
                        end
                    end

                    if ok then
                        best_cnt = #arr_subset
                        best_node_arr = arr_subset
                    end
                end
            end
        end
    end

    print("Best count: ", best_cnt)
    local node_list = {}
    for _, node in ipairs(best_node_arr) do
        node_list[#node_list + 1] = node
    end
    table.sort(node_list)
    print(join_arr(node_list, ","))
end

main()
