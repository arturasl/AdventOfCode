local function visit(y, x, expected, map, visited)
	assert(not visited[y][x] and map[y][x] == expected)

	visited[y][x] = true
	local area, perimeter = 1, 0

	for _, d in ipairs({ { y = -1, x = 0 }, { y = 1, x = 0 }, { y = 0, x = -1 }, { y = 0, x = 1 } }) do
		local ny, nx = y + d.y, x + d.x
		if not (1 <= ny and ny <= #map) or not (1 <= nx and nx <= #map[1]) or map[ny][nx] ~= expected then
			perimeter = perimeter + 1
		elseif not visited[ny][nx] then
			local other_area, other_perimiter = visit(ny, nx, expected, map, visited)
			area = area + other_area
			perimeter = perimeter + other_perimiter
		end
	end

	return area, perimeter
end

local function main()
	local map = {}
	for line in io.lines() do
		map[#map + 1] = {}
		for i = 1, #line do
			map[#map][i] = line:sub(i, i)
		end
	end

	local visited = {}
	for i = 1, #map do
		visited[i] = {}
	end

	local result = 0
	for y = 1, #map do
		for x = 1, #map[y] do
			if not visited[y][x] then
				local area, perimeter = visit(y, x, map[y][x], map, visited)
				result = result + area * perimeter
			end
		end
	end
	print(result)
end

main()
