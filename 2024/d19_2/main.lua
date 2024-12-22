local inspect = require("inspect")

local function read()
	local data = { available = {}, test = {} }
	for line in io.lines() do
		line = line:gsub("%s+", "")
		if line ~= "" then
			if #data.available == 0 then
				for pattern in line:gmatch("[^,]+") do
					table.insert(data.available, pattern)
				end
			else
				table.insert(data.test, line)
			end
		end
	end
	return data
end

local available = nil
local function make_in_ways(pattern)
	if pattern == "" then
		return 1
	end

	local result = 0
	for _, use in ipairs(available) do
		if pattern:match("^" .. use) then
			result = result + make_in_ways(pattern:gsub("^" .. use, ""))
		end
	end

	return result
end
make_in_ways = require("multikey.memoize")(make_in_ways)

local function main()
	local data = read()
	available = data.available

	local result = 0
	for _, pattern in ipairs(data.test) do
		local ways = make_in_ways(pattern)
		print(pattern, ways)
		result = result + ways
	end

	print(result)
end

main()
