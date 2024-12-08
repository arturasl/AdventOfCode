local dump = require("dump")

local function sz(tbl)
	local total = 0
	for _, _ in pairs(tbl) do
		total = total + 1
	end
	return total
end

local function read()
	local result = { height = 0, map = {}, antenas = {} }
	for line in io.lines() do
		line = line:gsub("^%s+", ""):gsub("%s+$", "")
		if line ~= "" then
			result.width = result.width or #line
			result.height = result.height + 1
			result.map[#result.map + 1] = {}

			for i = 1, #line do
				local char = line:sub(i, i)
				result.map[#result.map][i] = char
				if char ~= "." then
					result.antenas[char] = result.antenas[char] or {}
					table.insert(result.antenas[char], {
						y = result.height,
						x = i,
					})
				end
			end
		end
	end

	return result
end

local function main()
	local data = read()
	local antinodes = {}

	print(dump(data.antenas))

	for _, positions in pairs(data.antenas) do
		for i = 1, #positions do
			for j = 1, #positions do
				if i ~= j then
					local dy = positions[j].y - positions[i].y
					local dx = positions[j].x - positions[i].x
					local y = positions[i].y
					local x = positions[i].x

					while true do
						if not (1 <= y and y <= data.height) then
							break
						end
						if not (1 <= x and x <= data.width) then
							break
						end

						antinodes[string.format("%d %d", y, x)] = true
						y = y + dy
						x = x + dx
					end
				end
			end
		end
	end

	print(dump(antinodes))
	print(sz(antinodes))
end

main()
