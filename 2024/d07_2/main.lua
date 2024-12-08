local result = 0
for line in io.lines() do
	line = line:gsub("^%s+", ""):gsub("%s+$", "")
	if line ~= "" then
		local str_expected, rest = line:match("(%d+):(.*)")
		local expected = tonumber(str_expected)

		local nums = {}
		for str_num in rest:gmatch("%d+") do
			nums[#nums + 1] = tonumber(str_num)
		end

		local dp = { [1] = { [nums[1]] = 1 } }

		for i = 2, #nums do
			local concat_of_rest = ""
			for j = i + 1, #nums do
				concat_of_rest = concat_of_rest .. tostring(nums[j])
			end

			dp[i] = {}
			for prev, cnt in pairs(dp[i - 1]) do
				local function update(val)
					if val > expected then
						return
					end
					if tonumber(tostring(val) .. concat_of_rest) < expected then
						return
					end
					dp[i][val] = (dp[i][val] or 0) + cnt
				end

				update(prev + nums[i])
				update(prev * nums[i])
				update(tonumber(string.format("%d%d", prev, nums[i])))
			end
		end

		result = result + (dp[#nums][expected] ~= nil and expected or 0)
	end
end

print(result)
