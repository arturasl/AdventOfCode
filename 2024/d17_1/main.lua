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

local function calc_combo_str(num)
    if 0 <= num and num <= 3 then
        return tostring(num)
    end
    if 4 <= num and num <= 6 then
        return string.char(string.byte("A") + (num - 4))
    end

    assert(false, num)
end

local function calc_combo_val(str, data)
    if str:match("^%d+$") then
        return tonumber(str)
    end
    local val = data.registers[str]
    assert(val ~= nil)
    return val
end

local function main()
    local data = read()

    local result = {}
    while data.pointer < #data.program do
        assert(data.pointer + 1 < #data.program)
        assert(0 <= data.pointer)

        local op = data.program[data.pointer + 1]
        local val = data.program[data.pointer + 2]
        local str_combo = calc_combo_str(val)
        local combo = calc_combo_val(str_combo, data)
        if op == 0 then
            assert(combo >= 0)
            data.registers["A"] = data.registers["A"] >> combo
            print(("A = A >> %s"):format(str_combo))
        elseif op == 1 then
            data.registers["B"] = data.registers["B"] ~ val
            print(("B = B xor %d"):format(val))
        elseif op == 2 then
            data.registers["B"] = combo & 7
            print(("B = %s & 0b111"):format(str_combo))
        elseif op == 3 then
            if data.registers["A"] ~= 0 then
                assert(val % 2 == 0)
                data.pointer = val - 2
            end
            print(("jnz A to %d"):format(val))
        elseif op == 4 then
            data.registers["B"] = data.registers["B"] ~ data.registers["C"]
            print("B = B xor C")
        elseif op == 5 then
            result[#result + 1] = combo & 7
            print(("print %s & 0b111"):format(str_combo))
        elseif op == 6 then
            assert(combo >= 0)
            data.registers["B"] = data.registers["A"] >> combo
            print(("B = A >> %s"):format(str_combo))
        elseif op == 7 then
            assert(combo >= 0)
            data.registers["C"] = data.registers["A"] >> combo
            print(("C = A >> %s"):format(str_combo))
        else
            assert(false)
        end

        data.pointer = data.pointer + 2
    end

    print(join_arr(result, ","))
end

main()
