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
			dp[i] = {}
			for prev, cnt in pairs(dp[i - 1]) do
				local function update(val)
					dp[i][val] = (dp[i][val] or 0) + cnt
				end

				update(prev + nums[i])
				update(prev * nums[i])
			end
		end

		result = result + (dp[#nums][expected] ~= nil and expected or 0)
	end
end

print(result)
