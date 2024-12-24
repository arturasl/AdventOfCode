local regex = require("regex")

local function startswith(str, start)
    return str:sub(1, #start) == start
end

local instrustions, err = regex.matches(io.read("*a"), "do\\(\\)|don't\\(\\)|mul\\(\\d{1,3},\\d{1,3}\\)")
assert(instrustions, err)

local should_mul = true
local result = 0
for _, instruction in ipairs(instrustions) do
    if startswith(instruction, "do(") then
        should_mul = true
    elseif startswith(instruction, "don't(") then
        should_mul = false
    elseif startswith(instruction, "mul(") then
        if should_mul then
            local local_result = 1
            for str_num in instruction:gmatch("%d+") do
                local_result = local_result * tonumber(str_num)
            end
            result = result + local_result
        end
    else
        assert(false, instruction)
    end
end

print(result)
