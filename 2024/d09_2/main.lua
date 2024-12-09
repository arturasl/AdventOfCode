local RBTree = require("bintrees.rbtree")

local function read()
	local line = io.read("*a"):gsub("%s+", "")
	local blocks = {}
	local offset = 0
	for i = 1, #line do
		local amount = tonumber(line:sub(i, i))
		assert(amount ~= nil)
		if i % 2 == 1 then
			blocks[#blocks + 1] = {
				offset = offset,
				len = amount,
				id = i // 2,
			}
		end
		offset = offset + amount
	end

	return blocks
end

local function main()
	local blocks = read()

	local empty_offsets_per_len = {}
	for i = 0, 9 do
		empty_offsets_per_len[i] = RBTree:new(function(lhs, rhs)
			return lhs - rhs
		end)
	end

	for i = 1, #blocks - 1 do
		local empty_offset = blocks[i].offset + blocks[i].len
		local empty_len = blocks[i + 1].offset - empty_offset

		empty_offsets_per_len[empty_len]:insert(empty_offset)
	end

	for i_block = #blocks, 1, -1 do
		local best_offset, best_len = nil, nil
		for len = blocks[i_block].len, 9 do
			local offset = empty_offsets_per_len[len]:min()
			if offset ~= nil and (best_offset == nil or offset < best_offset) and offset < blocks[i_block].offset then
				best_offset, best_len = offset, len
			end
		end

		if best_offset ~= nil then
			blocks[i_block].offset = best_offset
			assert(empty_offsets_per_len[best_len]:remove(best_offset))

			local new_len = best_len - blocks[i_block].len
			assert(empty_offsets_per_len[new_len]:insert(best_offset + blocks[i_block].len))
		end
	end

	local result = 0
	for i = 1, #blocks do
		if blocks[i].len ~= 0 then
			local sum = (blocks[i].len * (blocks[i].len + 1)) // 2
			local sum_oft = (blocks[i].offset - 1) * blocks[i].len
			result = result + (sum + sum_oft) * blocks[i].id
		end
	end
	print(result)
end

main()
