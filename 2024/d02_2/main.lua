local num_safe = 0

for line in io.lines() do
    line = line:gsub("^%s+", ""):gsub("%s+$", "")
    if line ~= "" then
        local lst = {}
        for str_num in line:gmatch("(%d+)") do
            lst[#lst + 1] = tonumber(str_num)
        end

        local is_safe = false

        for i = 0, #lst do
            local fixed_lst = {}
            for j = 1, #lst do
                if j ~= i then
                    fixed_lst[#fixed_lst + 1] = lst[j]
                end
            end

            local sign = 0
            is_safe = true
            for j = 2, #fixed_lst do
                local diff = fixed_lst[j] - fixed_lst[j - 1]
                local abs_diff = math.abs(diff)
                local local_sign = diff < 0 and -1 or 1
                is_safe = is_safe and abs_diff >= 1 and abs_diff <= 3
                is_safe = is_safe and (sign == 0 or sign == local_sign)
                sign = local_sign
            end

            if is_safe then
                break
            end
        end

        num_safe = num_safe + (is_safe and 1 or 0)
    end
end

print(num_safe)
