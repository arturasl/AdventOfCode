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

local function main()
	local data = read()

	local queue = heap.minHeap(function(lhs, rhs)
		return lhs.cost < rhs.cost
	end)

	local move_to_d = {
		{ y = 0, x = 1 },
		{ y = 1, x = 0 },
		{ y = 0, x = -1 },
		{ y = -1, x = 0 },
	}
	queue:insert({
		cost = 0,
		move_dir = 1,
		pos = data.start_pos,
	})

	local visited = mk:new()

	while true do
		local cur_state = queue:pop()
		assert(cur_state ~= nil, "End was not reached")

		if same_pos(cur_state.pos, data.end_pos) then
			print(cur_state.cost)
			break
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

			if
				(1 <= next_state.pos.y and next_state.pos.y <= #data.map)
				and (1 <= next_state.pos.x and next_state.pos.x <= #data.map[next_state.pos.y])
				and data.map[next_state.pos.y][next_state.pos.x] == "."
				and visited:get(next_state.pos.y, next_state.pos.x, next_state.move_dir) == nil
			then
				visited:put(next_state.pos.y, next_state.pos.x, next_state.move_dir, true)
				queue:insert(next_state)
			end
		end
	end
end

main()
