local inspect = require("inspect")
local heap = require("binaryheap")
local mk = require("multikey")

local function read()
	local data = { map = {}, start_pos = {}, end_pos = {} }
	for line in io.lines() do
		line = line:gsub("%s+", "")
		if line ~= "" then
			data.map[#data.map + 1] = {}
			for i = 1, #line do
				local char = line:sub(i, i)
				if char == "S" or char == "E" then
					local pos = char == "S" and data.start_pos or data.end_pos
					assert(pos.y == nil and pos.x == nil)
					pos.y, pos.x = #data.map, i
					char = "."
				end
				table.insert(data.map[#data.map], char)
			end
		end
	end
	return data
end

local function add_pos(lhs, rhs)
	return { y = lhs.y + rhs.y, x = lhs.x + rhs.x }
end

local function find_shortest_paths(map, start_pos)
	local move_to_d = {
		{ y = 0, x = 1 },
		{ y = 1, x = 0 },
		{ y = 0, x = -1 },
		{ y = -1, x = 0 },
	}

	local queue = heap.minHeap(function(lhs, rhs)
		return lhs.cost < rhs.cost
	end)
	local visited = mk:new()

	queue:insert({ cost = 0, pos = start_pos })
	visited:put(start_pos.y, start_pos.x, 0)

	while true do
		local cur_state = queue:pop()
		if cur_state == nil then
			break
		end

		for _, delta in ipairs(move_to_d) do
			local next_state = {
				cost = cur_state.cost + 1,
				pos = add_pos(cur_state.pos, delta),
			}

			if
				(1 <= next_state.pos.y and next_state.pos.y <= #map)
				and (1 <= next_state.pos.x and next_state.pos.x <= #map[next_state.pos.y])
				and map[next_state.pos.y][next_state.pos.x] == "."
				and visited:get(next_state.pos.y, next_state.pos.x) == nil
			then
				visited:put(next_state.pos.y, next_state.pos.x, next_state.cost)
				queue:insert(next_state)
			end
		end
	end

	return visited
end

local function calc_saved(data, y, x, ny, nx)
	local new_len = data.till_start:get(y, x) + data.till_end:get(ny, nx) + math.abs(y - ny) + math.abs(x - nx)
	local saved = data.till_end:get(data.start_pos.y, data.start_pos.x) - new_len
	return saved
end

local function main()
	local cheat_len = 2
	local should_save_at_least = 100

	local data = read()

	data.till_end = find_shortest_paths(data.map, data.end_pos)
	data.till_start = find_shortest_paths(data.map, data.start_pos)

	local cheats = {}
	for y = 1, #data.map do
		for x = 1, #data.map[y] do
			if data.map[y][x] == "." then
				for dy = -cheat_len, cheat_len do
					for dx = -cheat_len, cheat_len do
						if math.abs(dy) + math.abs(dx) == cheat_len then
							local ny, nx = y + dy, x + dx
							if
								(1 <= ny and ny <= #data.map)
								and (1 <= nx and nx <= #data.map[ny])
								and data.map[ny][nx] == "."
							then
								local saved = calc_saved(data, y, x, ny, nx)
								if saved > 0 then
									cheats[saved] = (cheats[saved] or 0) + 1
								end
							end
						end
					end
				end
			end
		end
	end

	local cheat_counts = {}
	for saved, cnt in pairs(cheats) do
		cheat_counts[#cheat_counts + 1] = { saved = saved, cnt = cnt }
	end
	table.sort(cheat_counts, function(lhs, rhs)
		if lhs.cnt ~= rhs.cnt then
			return lhs.cnt > rhs.cnt
		end
		return lhs.saved < rhs.saved
	end)

	for _, cheat in pairs(cheat_counts) do
		print("count: " .. cheat.cnt .. ", saved: " .. cheat.saved)
	end

	local result = 0
	for _, cheat in pairs(cheat_counts) do
		if cheat.saved >= should_save_at_least then
			result = result + cheat.cnt
		end
	end
	print(result)
end

main()
