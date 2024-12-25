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

local function expect_run_produces(data)
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
            if #produced > #data.program then
                return nil
            end
            if produced[#produced] ~= data.program[#produced] then
                return #produced - 1
            end
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

    return #produced
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

local best_a = nil
local best_correct_up_to = 0
local orig = { program = {} }

local function find(start, A)
    if best_a ~= nil and A >= best_a then
        return
    end

    local cur_correct_up_to = expect_run_produces(new_data(orig, A)) or 0

    if cur_correct_up_to >= best_correct_up_to then
        if cur_correct_up_to == #orig.program then
            best_a = A
            return
        end

        print(best_a, cur_correct_up_to, start, A)
        best_correct_up_to = cur_correct_up_to
    end

    for len = 1, math.min(63 - start, 7) do
        for x = 0, (1 << len) - 1 do
            local new_A = ((x & ((1 << len) - 1)) << start) | A

            local correct_up_to = expect_run_produces(new_data(orig, new_A)) or 0
            if correct_up_to > cur_correct_up_to then
                find(start + len, new_A)
            end
        end
    end
end
find = require("multikey.memoize")(find)

local function main()
    orig = read()
    find(0, 0)
    print(best_a)
end

main()
