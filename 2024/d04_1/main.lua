local matrix = {}
while true do
    local line = io.read("*l")
    if line == nil then
        break
    end

    line = line:gsub("^%s+", "")
    if line ~= "" then
        table.insert(matrix, line)
    end
end

local height = #matrix
local width = #matrix[1]
local found = 0

for y = 1, height do
    for x = 1, width do
        if matrix[y]:sub(x, x) == "X" then
            for dy = -1, 1 do
                for dx = -1, 1 do
                    if dy ~= 0 or dx ~= 0 then
                        local yy, xx = y, x
                        local read = ""
                        for _ = 1, 4 do
                            if 1 <= yy and yy <= height and 1 <= xx and xx <= width then
                                read = read .. matrix[yy]:sub(xx, xx)
                            end
                            yy = yy + dy
                            xx = xx + dx
                        end

                        found = found + (read == "XMAS" and 1 or 0)
                    end
                end
            end
        end
    end
end

print(found)
