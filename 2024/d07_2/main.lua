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

        local dp = { [#nums + 1] = { [expected] = 1 } }

        for i = #nums, 1, -1 do
            dp[i] = {}
            local str_num = tostring(nums[i])
            for next, cnt in pairs(dp[i + 1]) do
                local function update(val)
                    dp[i][val] = (dp[i][val] or 0) + cnt
                end

                if next - nums[i] >= 0 then
                    update(next - nums[i])
                end

                if next % nums[i] == 0 then
                    update(next / nums[i])
                end

                local str_next = tostring(next)
                if #str_next >= #str_num and str_next:sub(-#str_num) == str_num then
                    local left = str_next:sub(1, -(#str_num + 1))
                    update(left == "" and 0 or tonumber(left))
                end
            end
        end

        result = result + (dp[1][0] ~= nil and expected or 0)
    end
end

print(result)
