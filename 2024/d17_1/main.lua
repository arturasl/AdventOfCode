local inspect = require("inspect")

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

local function main()
    local data = read()
    local first_print = true

    while data.pointer < #data.program do
        assert(data.pointer + 1 < #data.program)
        assert(0 <= data.pointer)

        local op = data.program[data.pointer + 1]
        local val = data.program[data.pointer + 2]
        local combo = calc_combo_val(val, data)
        if op == 0 then
            assert(combo >= 0)
            data.registers["A"] = data.registers["A"] // (1 << combo)
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
            if not first_print then
                io.write(",")
            end
            first_print = false
            io.write(combo % 8)
        elseif op == 6 then
            assert(combo >= 0)
            data.registers["B"] = data.registers["A"] // (1 << combo)
        elseif op == 7 then
            assert(combo >= 0)
            data.registers["C"] = data.registers["A"] // (1 << combo)
        else
            assert(false)
        end

        data.pointer = data.pointer + 2
    end

    io.write("\n")
end

main()
