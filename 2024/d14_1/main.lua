local dump = require("dump")

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
    local time = 100
    local robots = read()

    for _, robot in ipairs(robots) do
        robot.pos.y = (robot.pos.y + robot.d.y * time) % height
        robot.pos.x = (robot.pos.x + robot.d.x * time) % width
    end

    local quadrants = {
        [0] = { [0] = 0, [1] = 0 },
        [1] = { [0] = 0, [1] = 0 },
    }

    assert(width % 2 == 1)
    assert(height % 2 == 1)
    for _, robot in ipairs(robots) do
        if robot.pos.x ~= width // 2 and robot.pos.y ~= height // 2 then
            local q1 = (robot.pos.x > width // 2) and 1 or 0
            local q2 = (robot.pos.y > height // 2) and 1 or 0
            quadrants[q1][q2] = quadrants[q1][q2] + 1
        end
    end

    local safety = 1
    for _, ys in pairs(quadrants) do
        for _, cnt in pairs(ys) do
            safety = safety * cnt
        end
    end
    print(safety)
end

main()
