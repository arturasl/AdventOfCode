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

local function eval_internal(graph, cache, node_name)
    local node = graph[node_name]
    if node.op == "const" then
        return node.lhs
    end

    if cache[node_name] ~= nil then
        return cache[node_name]
    end
    cache[node_name] = ""

    local lhs = eval_internal(graph, cache, node.lhs)
    local rhs = eval_internal(graph, cache, node.rhs)
    if lhs == "" or rhs == "" then
        return ""
    end

    local result = 0
    if node.op == "XOR" then
        result = lhs ~= rhs and 1 or 0
    elseif node.op == "OR" then
        result = (lhs == 1 or rhs == 1) and 1 or 0
    elseif node.op == "AND" then
        result = (lhs == 1 and rhs == 1) and 1 or 0
    else
        assert(false)
    end

    cache[node_name] = result
    return result
end

local function arr_join(arr, sep)
    local result = ""
    for i = 1, #arr do
        if i ~= 1 then
            result = result .. sep
        end
        result = result .. tostring(arr[i])
    end
    return result
end

local function eval(graph, num_bits)
    local cache = {}
    local result = ""
    for z = num_bits - 1, 0, -1 do
        local node = ("z%02d"):format(z)
        if graph[node] ~= nil then
            local bit = tostring(eval_internal(graph, cache, node))
            if bit == "" then
                return nil
            end
            result = result .. bit
        end
    end
    return tonumber(result, 2)
end

local function get_num_bits(graph, ch)
    local num_bits = 0
    for z = 0, 99 do
        local node = ("%s%02d"):format(ch, z)
        if graph[node] == nil then
            break
        end
        num_bits = num_bits + 1
    end
    return num_bits
end

local function print_graph(graph, non_input_gates)
    for _, node_name in ipairs(non_input_gates) do
        local node = graph[node_name]
        print(("%s %s %s -> %s"):format(node.lhs, node.op, node.rhs, node_name))
    end
end

local function gen_random_bits(num_bits)
    local bits = {}
    for i = 1, num_bits do
        bits[i] = math.random(0, 1)
    end
    return bits
end

local function bits_to_int(bits)
    local pow = 1
    local result = 0
    for _, bit in ipairs(bits) do
        result = result + bit * pow
        pow = pow * 2
    end
    return result
end

local function mutate_swaps(swaps, non_input_gates)
    local result = {}
    local had = {}
    for i = 1, #swaps do
        if math.random(0, 1) == 1 then
            result[i] = swaps[i]
            had[swaps[i]] = true
        end
    end

    for i = 1, #swaps do
        while result[i] == nil do
            result[i] = non_input_gates[math.random(1, #non_input_gates)]
            if had[result[i]] ~= nil then
                result[i] = nil
            end
        end
        had[result[i]] = true
    end

    return result
end

local function evaluate_canaries(graph, canaries, num_z_bits, arg_less_bits)
    local result = 0
    for _, canary in ipairs(canaries) do
        for b = 1, num_z_bits - arg_less_bits do
            graph[("x%02d"):format(b - 1)].lhs = canary.lhs[b]
            graph[("y%02d"):format(b - 1)].lhs = canary.rhs[b]
        end
        local eval_result = eval(graph, num_z_bits)
        if eval_result == nil then
            return nil
        end
        result = result + math.abs(eval_result - canary.r)
    end
    return result
end

local function apply_swaps(graph, swaps)
    for i = 1, #swaps, 2 do
        graph[swaps[i]], graph[swaps[i + 1]] = graph[swaps[i + 1]], graph[swaps[i]]
    end
end

local function find_non_inpute_gates(graph)
    local non_input_gates = {}
    for node, _ in pairs(graph) do
        if ("xy"):find(node:sub(1, 1)) == nil then
            non_input_gates[#non_input_gates + 1] = node
        end
    end
    table.sort(non_input_gates)
    return non_input_gates
end

local function swaps_to_result(swaps)
    local cpy = {}
    for _, v in ipairs(swaps) do
        cpy[#cpy + 1] = v
    end
    table.sort(cpy)
    return arr_join(cpy, ",")
end

local function main()
    local graph = read()

    local probs = {
        ["small"] = {
            bad_wires = 4,
            fn_to_optimize = function(a, b)
                return bit32.band(a, b)
            end,
            str_op = "&",
            arg_less_bits = 0,
        },
        ["large"] = {
            bad_wires = 8,
            fn_to_optimize = function(a, b)
                return a + b
            end,
            str_op = "+",
            arg_less_bits = 1,
        },
    }
    local prob = probs["large"]

    local num_z_bits = get_num_bits(graph, "z")
    assert(num_z_bits - prob.arg_less_bits == get_num_bits(graph, "x"), "x bit cnt")
    assert(num_z_bits - prob.arg_less_bits == get_num_bits(graph, "y"), "y bit cnt")
    print(("z bits: %s"):format(num_z_bits))

    local non_input_gates = find_non_inpute_gates(graph)

    local canaries = {}
    for _ = 1, 10 do
        local lhs = gen_random_bits(num_z_bits - prob.arg_less_bits)
        local rhs = gen_random_bits(num_z_bits - prob.arg_less_bits)
        local r = prob.fn_to_optimize(bits_to_int(lhs), bits_to_int(rhs))
        canaries[#canaries + 1] = {
            lhs = lhs,
            rhs = rhs,
            r = r,
        }
        print(
            ("Canary: %s (%d) %s %s (%d) = %d"):format(
                arr_join(lhs, ""),
                bits_to_int(lhs),
                prob.str_op,
                arr_join(rhs, ""),
                bits_to_int(rhs),
                r
            )
        )
    end

    local swaps = {}
    for i = 1, prob.bad_wires, 2 do
        swaps[i] = non_input_gates[i]
        swaps[i + 1] = non_input_gates[i]
    end
    mutate_swaps(swaps, non_input_gates)

    local best_score = evaluate_canaries(graph, canaries, num_z_bits, prob.arg_less_bits)
    local best_swaps = swaps

    local local_best_score = best_score

    local its = 0
    while best_score ~= 0 do
        its = its + 1

        local local_swaps = mutate_swaps(swaps, non_input_gates)
        apply_swaps(graph, local_swaps)

        local local_off_by = evaluate_canaries(graph, canaries, num_z_bits, prob.arg_less_bits)
        if its % 10000 == 0 then
            print(
                ("best score: %d, result: %s, local_best_score: %d, local score: %d"):format(
                    best_score,
                    swaps_to_result(best_swaps),
                    local_best_score,
                    local_off_by or -1
                )
            )
        end

        apply_swaps(graph, local_swaps)

        if local_off_by ~= nil then
            if local_off_by < local_best_score * 2 then
                swaps = local_swaps
                local_best_score = local_off_by
            end

            if local_off_by < best_score then
                swaps = local_swaps
                best_swaps = local_swaps
                best_score = local_off_by
            end

            if math.random(1, 100000) == 1 then
                swaps = best_swaps
                local_best_score = best_score
            end
        end
    end

    print(swaps_to_result(best_swaps))
end

main()
