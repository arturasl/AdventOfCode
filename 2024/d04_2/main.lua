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

for y = 1, height - 2 do
	for x = 1, width - 2 do
		local d1, d2 = {}, {}
		for i = 0, 2 do
			table.insert(d1, matrix[y + i]:sub(x + i, x + i))
			table.insert(d2, matrix[y + 2 - i]:sub(x + i, x + i))
		end

		if d1[2] == "A" and d2[2] == "A" then
			table.sort(d1)
			table.sort(d2)
			if d1[2] .. d1[3] == "MS" and d2[2] .. d2[3] == "MS" then
				found = found + 1
			end
		end
	end
end

print(found)
