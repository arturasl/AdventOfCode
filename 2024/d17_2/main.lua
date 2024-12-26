local function read()
    local data = { program = {}, registers = {}, pointer = 0 }
    for line in io.lines() do
        line = line:gsub("%s+", "")
        if line ~= "" then
            local reg, val = line:match("^Register([ABC]):(%d*)$")
            local prog = line:match("^Program:(.*)$")
            if reg ~= nil then
                assert(data.registers[reg] == nil)
                data.registers[reg] = tonumber(val)
            elseif prog ~= nil then
                for str_num in prog:gmatch("%d+") do
                    table.insert(data.program, tonumber(str_num))
                end
            else
                assert(false)
            end
        end
    end
    return data
end

local function calc_combo_val(num, data)
    if 0 <= num and num <= 3 then
        return num
    end
    if 4 <= num and num <= 6 then
        local val = data.registers[string.char(string.byte("A") + (num - 4))]
        assert(val ~= nil)
        return val
    end

    assert(false, num)
end

local function run(data)
    local produced = {}
    while data.pointer < #data.program do
        assert(data.pointer + 1 < #data.program)
        assert(0 <= data.pointer)

        local op = data.program[data.pointer + 1]
        local val = data.program[data.pointer + 2]
        local combo = calc_combo_val(val, data)
        if op == 0 then
            assert(combo >= 0)
            data.registers["A"] = data.registers["A"] >> combo
        elseif op == 1 then
            data.registers["B"] = data.registers["B"] ~ val
        elseif op == 2 then
            data.registers["B"] = (combo % 8)
        elseif op == 3 then
            if data.registers["A"] ~= 0 then
                assert(val % 2 == 0)
                data.pointer = val - 2
            end
        elseif op == 4 then
            data.registers["B"] = data.registers["B"] ~ data.registers["C"]
        elseif op == 5 then
            produced[#produced + 1] = combo % 8
        elseif op == 6 then
            assert(combo >= 0)
            data.registers["B"] = data.registers["A"] >> combo
        elseif op == 7 then
            assert(combo >= 0)
            data.registers["C"] = data.registers["A"] >> combo
        else
            assert(false)
        end

        data.pointer = data.pointer + 2
    end

    return produced
end

local function new_data(orig, new_A)
    return {
        program = orig.program,
        registers = {
            ["A"] = new_A,
            ["B"] = orig.registers["B"],
            ["C"] = orig.registers["C"],
        },
        pointer = 0,
    }
end

local function find(orig)
    local best_a = nil
    local block_size = 3

    local function find_internal(want_last, A)
        if want_last == #orig.program + 1 then
            best_a = math.min(best_a or A, A)
            return
        end

        for x = 0, ((1 << block_size) - 1) do
            local cur_a = (A << block_size) | x
            local result = run(new_data(orig, cur_a))

            local ok = true
            local oft = #orig.program - want_last + 1
            for i = oft, #orig.program do
                ok = ok and result[i - oft + 1] == orig.program[i]
            end

            if ok then
                find_internal(want_last + 1, cur_a)
            end
        end
    end
    find_internal = require("multikey.memoize")(find_internal)

    find_internal(1, 0)
    return best_a
end

local function main()
    local data = read()
    print(find(data))
end

main()
