local function find_next_secreat(s)
	s = ((s << 6) ~ s) & ((1 << 24) - 1)
	s = ((s >> 5) ~ s) & ((1 << 24) - 1)
	s = ((s << 11) ~ s) & ((1 << 24) - 1)
	return s
end

local function main()
	local iters = 2000
	local seq_len = 4

	local seq_to_total_cost = {}
	for line in io.lines() do
		line = line:gsub("%s+", "")
		if line ~= "" then
			local s = tonumber(line)
			local seq = {}
			local seen_for_seller = {}

			local key = 0
			for cur_len = 1, iters do
				local next_s = find_next_secreat(s)
				table.insert(seq, (next_s % 10) - (s % 10))
				key = key * 100 + seq[#seq] + 10
				if cur_len >= seq_len then
					if seen_for_seller[key] == nil then
						seen_for_seller[key] = true
						seq_to_total_cost[key] = (seq_to_total_cost[key] or 0) + (next_s % 10)
					end
					key = key - (seq[#seq - (seq_len - 1)] + 10) * 1000000
				end

				s = next_s
			end
		end
	end

	local best = -1
	for _, cost in pairs(seq_to_total_cost) do
		best = math.max(best, cost)
	end
	print(best)
end

main()
