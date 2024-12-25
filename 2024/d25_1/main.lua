local function parse(block, data)
    assert(#block == 7, #block)
    for i = 1, #block do
        assert(#block[i] == 5, #block[i])
    end

    local is_key = true
    for i = 1, #block[#block] do
        is_key = is_key and block[#block][i] == "#"
    end

    local is_lock = true
    for i = 1, #block[1] do
        is_lock = is_lock and block[1][i] == "#"
    end

    assert(is_key ~= is_lock and (is_key or is_lock))

    local filling = is_key and data.keys or data.locks
    filling[#filling + 1] = {}
    for x = 1, #block[1] do
        local num_blocks = 0
        for y = 1, #block do
            num_blocks = num_blocks + (block[y][x] == "#" and 1 or 0)
        end
        table.insert(filling[#filling], num_blocks - 1)
    end
end

local function read()
    local data = { keys = {}, locks = {} }
    local cur = {}
    for line in io.lines() do
        line = line:gsub("%s+", " "):gsub("^%s+", ""):gsub("%s$", "")
        if line ~= "" then
            cur[#cur + 1] = {}
            for i = 1, #line do
                table.insert(cur[#cur], line:sub(i, i))
            end
        elseif #cur ~= 0 then
            parse(cur, data)
            cur = {}
        end
    end

    if #cur ~= 0 then
        parse(cur, data)
    end

    return data
end

local function main()
    local data = read()

    local result = 0
    for _, key in ipairs(data.keys) do
        for _, lock in ipairs(data.locks) do
            local ok = true
            for x = 1, #key do
                ok = ok and key[x] + lock[x] <= 5
            end
            result = result + (ok and 1 or 0)
        end
    end
    print(result)
end

main()
