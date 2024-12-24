local mk = require("multikey")
local inspect = require("inspect")

local function read()
    local robots = {}
    for line in io.lines() do
        if line:gsub("%s+", "") ~= "" then
            local px, py, dx, dy = line:match("^p=(-?%d+),(-?%d+) v=(-?%d+),(-?%d+)$")
            robots[#robots + 1] = {
                pos = { y = tonumber(py), x = tonumber(px) },
                d = { y = tonumber(dy), x = tonumber(dx) },
            }
        end
    end
    return robots
end

local function main()
    -- local width, height = 11, 7
    -- local time = 100
    local width, height = 101, 103
    local robots = read()

    for time = 0, 10000 do
        local taken = mk.new()
        for _, robot in ipairs(robots) do
            taken:put((robot.pos.y + robot.d.y * time) % height, (robot.pos.x + robot.d.x * time) % width, true)
        end

        local have_around = 0
        for _, y, x in taken:tuples() do
            local cnt = 0
            for dy = -1, 1 do
                for dx = -1, 1 do
                    if dy ~= 0 and dx ~= 0 then
                        cnt = cnt + (taken:get(y + dy, x + dx) ~= nil and 1 or 0)
                    end
                end
            end
            have_around = have_around + (cnt > 0 and 1 or 0)
        end

        local frac = have_around * 100. / #robots
        if have_around * 100. / #robots > 40 then
            print("############### ", time, " (", frac, "%) ###############")
            for y = 0, height - 1 do
                for x = 0, width - 1 do
                    if taken:get(y, x) then
                        io.write("x")
                    else
                        io.write(".")
                    end
                end
                io.write("\n")
            end
        end
    end
end

main()
