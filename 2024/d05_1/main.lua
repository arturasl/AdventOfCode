local dump = require("dump")

local must_follow = {}
local checks = {}
for line in io.lines() do
	line = line:gsub("^%s+", ""):gsub("%s+$", "")
	if line ~= "" then
		local lhs, rhs = line:match("^(%d+)|(%d+)$")
		if lhs ~= nil then
			assert(rhs ~= nil)
			must_follow[lhs] = must_follow[lhs] or {}
			must_follow[lhs][rhs] = true
		else
			checks[#checks + 1] = { offset_to_num = {}, num_to_offset = {} }
			local offset = 1
			for num in line:gmatch("%d+") do
				assert(checks[#checks].num_to_offset[num] == nil)
				table.insert(checks[#checks].offset_to_num, num)
				checks[#checks].num_to_offset[num] = offset
				offset = offset + 1
			end
		end
	end
end

print(dump(must_follow))
print(dump(checks))

local total = 0
for _, check in ipairs(checks) do
	local corrected = {}
	for k, v in pairs(check.offset_to_num) do
		corrected[k] = v
	end

	table.sort(corrected, function(lhs, rhs)
		if (must_follow[lhs] or {})[rhs] ~= nil then
			return true
		end

		if (must_follow[rhs] or {})[lhs] ~= nil then
			return false
		end

		return check.num_to_offset[lhs] < check.num_to_offset[rhs]
	end)

	local ok = true
	for i = 1, #corrected do
		ok = ok and corrected[i] == check.offset_to_num[i]
	end

	if not ok then
		assert(#corrected % 2 == 1, #corrected)
		total = total + corrected[(#corrected + 1) // 2]
	end
end

print(total)
