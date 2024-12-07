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
			table.insert(must_follow[lhs], rhs)
		else
			checks[#checks + 1] = {}
			local offset = 1
			for num in line:gmatch("%d+") do
				assert(checks[#checks][num] == nil)
				checks[#checks][num] = offset
				offset = offset + 1
			end
		end
	end
end

print(dump(must_follow))
print(dump(checks))

local total = 0
for _, check in ipairs(checks) do
	local ok = true
	for num, offset in pairs(check) do
		for _, follow in ipairs(must_follow[num] or {}) do
			ok = ok and offset < (check[follow] or offset + 1)
		end
	end

	if ok then
		local sorted = {}
		for num, offset in pairs(check) do
			sorted[offset] = num
		end

		assert(#sorted % 2 == 1, #sorted)
		total = total + sorted[(#sorted + 1) // 2]
	end
end

print(total)
