local inspect = require("inspect")

local function read()
    local graph = {}
    for line in io.lines() do
        line = line:gsub("%s+", " "):gsub("^%s+", ""):gsub("%s$", "")
        if line ~= "" then
            local node, val = line:match("([^:]+): ([01])")
            if node ~= nil then
                assert(graph[node] == nil)
                graph[node] = {
                    lhs = tonumber(val),
                    rhs = "",
                    op = "const",
                }
            else
                local parts = {}
                for part in line:gmatch("%S+") do
                    parts[#parts + 1] = part
                end
                assert(#parts == 5, #parts)
                assert(parts[4] == "->")
                assert(graph[parts[5]] == nil)
                graph[parts[5]] = {
                    lhs = parts[1],
                    rhs = parts[3],
                    op = parts[2],
                }
            end
        end
    end
    return graph
end

local function eval(graph, node)
    if graph[node].op == "const" then
        return graph[node].lhs
    end

    local lhs = eval(graph, graph[node].lhs)
    local rhs = eval(graph, graph[node].rhs)

    local result = 0
    if graph[node].op == "XOR" then
        result = lhs ~ rhs
    elseif graph[node].op == "OR" then
        result = lhs | rhs
    elseif graph[node].op == "AND" then
        result = lhs & rhs
    else
        assert(false)
    end

    graph[node].op = "const"
    graph[node].lhs = result
    graph[node].rhs = ""
    return result
end

local function main()
    local graph = read()

    local result = ""
    for z = 99, 0, -1 do
        local node = "z"
        if z < 10 then
            node = node .. "0"
        end
        node = node .. tostring(z)
        if graph[node] ~= nil then
            result = result .. tostring(eval(graph, node))
        end
    end
    print(result)
    print(tonumber(result, 2))
end

main()
