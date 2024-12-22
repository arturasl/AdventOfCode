local function find_next_secreat(s)
	s = ((s << 6) ~ s) & ((1 << 24) - 1)
	s = ((s >> 5) ~ s) & ((1 << 24) - 1)
	s = ((s << 11) ~ s) & ((1 << 24) - 1)
	return s
end

local function main()
	local iters = 2000
	local result = 0
	for line in io.lines() do
		line = line:gsub("%s+", "")
		if line ~= "" then
			local s = tonumber(line)
			for _ = 1, iters do
				s = find_next_secreat(s)
			end
			result = result + s
		end
	end
	print(result)
end

main()
