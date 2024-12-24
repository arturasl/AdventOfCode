local inspect = require("inspect")

local function add_edge(graph, from, to)
    graph[from] = graph[from] or {}
    graph[from][to] = true
end

local function set_intersect(lhs, rhs)
    local result = {}
    for val, _ in pairs(lhs) do
        if rhs[val] then
            result[val] = true
        end
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

    local result = 0
    for from, set_to in pairs(graph) do
        for to, _ in pairs(set_to) do
            if from < to then
                for o, _ in pairs(set_intersect(set_to, graph[to])) do
                    if to < o and (from:sub(1, 1) == "t" or to:sub(1, 1) == "t" or o:sub(1, 1) == "t") then
                        result = result + 1
                    end
                end
            end
        end
    end
    print(result)
end

main()
