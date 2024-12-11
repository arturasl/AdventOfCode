local steps = nil
local function internal_steps(num, steps_left)
	if steps_left == 0 then
		return 1
	end

	if num == 0 then
		return steps(1, steps_left - 1)
	end

	local str_num = tostring(num)
	if #str_num % 2 == 0 then
		return steps(tonumber(str_num:sub(1, #str_num // 2)), steps_left - 1)
			+ steps(tonumber(str_num:sub(#str_num // 2 + 1, #str_num)), steps_left - 1)
	end

	return steps(num * 2024, steps_left - 1)
end
steps = require("multikey.memoize")(internal_steps)

local function main()
	local nums = {}
	for str_num in io.read("*a"):gmatch("%d+") do
		nums[#nums + 1] = tonumber(str_num)
	end

	local result = 0
	for _, num in ipairs(nums) do
		result = result + steps(num, 25)
	end

	print(result)
end

main()
