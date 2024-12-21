local heap = require("binaryheap")
local mk = require("multikey")

local function read()
	local data = { map = {}, start_pos = { y = nil, x = nil }, end_pos = { y = nil, x = nil } }
	for line in io.lines() do
		line = line:gsub("%s+", "")
		if line ~= "" then
			data.map[#data.map + 1] = {}
			for i = 1, #line do
				local char = line:sub(i, i)
				if char == "S" or char == "E" then
					local pos = { data.start_pos, data.end_pos }
					pos = pos[char == "S" and 1 or 2]
					assert(pos.y == nil and pos.x == nil)
					pos.y = #data.map
					pos.x = i
					char = "."
				end
				assert(string.find("#.", char) ~= nil, char, 1, true)
				table.insert(data.map[#data.map], char)
			end
		end
	end
	assert(data.start_pos.y ~= nil and data.end_pos.y ~= nil)
	return data
end

local function same_pos(lhs, rhs)
	return lhs.y == rhs.y and lhs.x == rhs.x
end

local function mod(x, m)
	return ((x - 1) % m) + 1
end

local function add_pos(lhs, rhs)
	return { y = lhs.y + rhs.y, x = lhs.x + rhs.x }
end

local function sz(m)
	local result = 0
	for _ in m:tuples() do
		result = result + 1
	end
	return result
end

local move_to_d = {
	{ y = 0, x = 1 },
	{ y = 1, x = 0 },
	{ y = 0, x = -1 },
	{ y = -1, x = 0 },
}

local function find_shortest_paths(data, visited)
	local queue = heap.minHeap(function(lhs, rhs)
		return lhs.cost < rhs.cost
	end)

	queue:insert({
		cost = 0,
		move_dir = 1,
		pos = data.start_pos,
	})

	visited:put(data.start_pos.y, data.start_pos.x, 1, {
		cost = 0,
		parents = mk:new(),
	})

	local cheapest = nil

	while true do
		local cur_state = queue:pop()
		if cur_state == nil then
			break
		end

		if same_pos(cur_state.pos, data.end_pos) then
			if cheapest == nil or cheapest > cur_state.cost then
				cheapest = cur_state.cost
			end
		end

		for _, next in ipairs({
			{ d = cur_state.move_dir, cost = 1 },
			{ d = mod(cur_state.move_dir + 1, #move_to_d), cost = 1000 },
			{ d = mod(cur_state.move_dir - 1, #move_to_d), cost = 1000 },
		}) do
			local next_state = {
				cost = cur_state.cost + next.cost,
				move_dir = next.d,
				pos = next.d == cur_state.move_dir and add_pos(cur_state.pos, move_to_d[next.d])
					or { y = cur_state.pos.y, x = cur_state.pos.x },
			}

			local prev = visited:get(next_state.pos.y, next_state.pos.x, next_state.move_dir)
			if
				(1 <= next_state.pos.y and next_state.pos.y <= #data.map)
				and (1 <= next_state.pos.x and next_state.pos.x <= #data.map[next_state.pos.y])
				and data.map[next_state.pos.y][next_state.pos.x] == "."
				and (
					prev == nil
					or (
						prev.cost == next_state.cost
						and prev.parents:get(cur_state.pos.y, cur_state.pos.x, cur_state.move_dir) == nil
					)
				)
			then
				prev = prev or { cost = next_state.cost, parents = mk:new() }
				prev.parents:put(cur_state.pos.y, cur_state.pos.x, cur_state.move_dir, true)
				visited:put(next_state.pos.y, next_state.pos.x, next_state.move_dir, prev)
				queue:insert(next_state)
			end
		end
	end

	return cheapest
end

local function reverse_visit(pos, move_dir, shortest_paths, visited)
	local path = shortest_paths:get(pos.y, pos.x, move_dir)
	if path == nil then
		return
	end

	if visited:get(pos.y, pos.x, move_dir) then
		return
	end
	visited:put(pos.y, pos.x, move_dir, true)

	for _, y, x, p_move_dir in path.parents:tuples() do
		reverse_visit({ y = y, x = x }, p_move_dir, shortest_paths, visited)
	end
end

local function main()
	local data = read()

	local shortest_paths = mk:new()
	local cheapest = find_shortest_paths(data, shortest_paths)

	local visited = mk:new()
	for move_dir = 1, #move_to_d do
		local cost = (shortest_paths:get(data.end_pos.y, data.end_pos.x, move_dir) or { cost = cheapest + 1 }).cost
		if cost == cheapest then
			reverse_visit(data.end_pos, move_dir, shortest_paths, visited)
		end
	end

	local unique_poses = mk:new()
	for _, y, x, _ in visited:tuples() do
		unique_poses:put(y, x, true)
	end
	print(sz(unique_poses))
end

main()
