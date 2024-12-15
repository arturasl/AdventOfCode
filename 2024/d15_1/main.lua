local inspect = require("inspect")

local function print_data(data)
	io.write("############# ", data.step, " #############\n")
	for y = 1, #data.map do
		for x = 1, #data.map[y] do
			if data.robot.y == y and data.robot.x == x then
				if data.step == #data.moves + 1 then
					io.write("@")
				else
					io.write(data.moves[data.step])
				end
			else
				io.write(data.map[y][x])
			end
		end
		io.write("\n")
	end
	io.write("\n")
end

local function read()
	local data = { map = {}, robot = { y = nil, x = nil }, moves = {}, step = 0 }
	for line in io.lines() do
		line = line:gsub("%s+", "")
		if line ~= "" then
			if string.find("<>v^", line:sub(1, 1)) ~= nil then
				for i = 1, #line do
					data.moves[#data.moves + 1] = line:sub(i, i)
					assert(string.find("<>v^", data.moves[#data.moves]))
				end
			else
				data.map[#data.map + 1] = {}
				for i = 1, #line do
					local char = line:sub(i, i)
					if char == "@" then
						assert(data.robot.y == nil and data.robot.x == nil)
						data.robot.y = #data.map
						data.robot.x = i
						char = "."
					end

					assert(string.find("#.O", char) ~= nil, char)
					table.insert(data.map[#data.map], char)
				end
			end
		end
	end
	return data
end

local function main()
	local data = read()

	local move_to_d = {
		["v"] = { y = 1, x = 0 },
		["^"] = { y = -1, x = 0 },
		["<"] = { y = 0, x = -1 },
		[">"] = { y = 0, x = 1 },
	}

	while true do
		data.step = data.step + 1
		-- print_data(data)
		if data.step == #data.moves + 1 then
			break
		end

		local d = move_to_d[data.moves[data.step]]
		local ny, nx = data.robot.y, data.robot.x

		while true do
			ny, nx = ny + d.y, nx + d.x
			if data.map[ny][nx] ~= "O" then
				break
			end
		end

		if data.map[ny][nx] == "." then
			while true do
				local prev_y, prev_x = ny - d.y, nx - d.x
				data.map[ny][nx] = data.map[prev_y][prev_x]
				ny, nx = prev_y, prev_x
				if data.map[ny][nx] == "." then
					break
				end
			end
			data.robot.y, data.robot.x = data.robot.y + d.y, data.robot.x + d.x
		end
	end

	local result = 0
	for y = 1, #data.map do
		for x = 1, #data.map[y] do
			if data.map[y][x] == "O" then
				result = result + (y - 1) * 100 + x - 1
			end
		end
	end
	print(result)
end

main()
