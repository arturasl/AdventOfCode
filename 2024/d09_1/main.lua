local function read()
    local line = io.read("*a"):gsub("%s+", "")
    local blocks = {}
    local offset = 0
    for i = 1, #line do
        local amount = tonumber(line:sub(i, i))
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

    local empties = {}
    for i = 1, #blocks - 1 do
        local empty_offset = blocks[i].offset + blocks[i].len
        local empty_len = blocks[i + 1].offset - empty_offset
        empties[#empties + 1] = {
            offset = empty_offset,
            len = empty_len,
        }
    end

    local i_block = #blocks
    local i_empty = 1

    while i_empty <= #empties and i_block >= 1 and empties[i_empty].offset < blocks[i_block].offset do
        if empties[i_empty].len == 0 then
            i_empty = i_empty + 1
        elseif blocks[i_block].len == 0 then
            i_block = i_block - 1
        else
            local i_new = #blocks + 1
            blocks[i_new] = {}
            blocks[i_new].len = math.min(empties[i_empty].len, blocks[i_block].len)
            blocks[i_new].offset = empties[i_empty].offset
            blocks[i_new].id = blocks[i_block].id

            empties[i_empty].len = empties[i_empty].len - blocks[i_new].len
            empties[i_empty].offset = empties[i_empty].offset + blocks[i_new].len
            blocks[i_block].len = blocks[i_block].len - blocks[i_new].len
        end
    end

    -- table.sort(blocks, function(lhs, rhs)
    -- 	return lhs.offset < rhs.offset
    -- end)
    -- for i = 1, #blocks do
    -- 	for _ = 1, blocks[i].len do
    -- 		io.write(blocks[i].id)
    -- 	end
    -- end
    -- io.write("\n")

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
