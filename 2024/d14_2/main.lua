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

    local around = {}
    for dy = -1, 1 do
        for dx = -1, 1 do
            if dy ~= 0 and dx ~= 0 then
                around[#around + 1] = { y = dy, x = dx }
            end
        end
    end

    local next_robots = {}

    local map = {}
    for y = 1, height do
        map[y] = {}
        for x = 1, width do
            map[y][x] = false
        end
    end

    local best_score = 0
    for time = 0, width * height + 1 do
        for i, robot in ipairs(robots) do
            next_robots[i] = {
                y = (robot.pos.y + robot.d.y * time) % height,
                x = (robot.pos.x + robot.d.x * time) % width,
            }
        end

        for _, robot in ipairs(next_robots) do
            map[robot.y + 1][robot.x + 1] = true
        end

        local have_around = 0
        for _, robot in ipairs(next_robots) do
            local cnt = 0
            for _, d in ipairs(around) do
                local ny, nx = robot.y + d.y + 1, robot.x + d.x + 1
                if 1 <= ny and ny <= height and 1 <= nx and nx <= width then
                    cnt = cnt + (map[ny][nx] and 1 or 0)
                end
            end
            have_around = have_around + (cnt > 0 and 1 or 0)
        end

        local frac = have_around * 100. / #robots
        if best_score < frac then
            best_score = frac
            print("############### ", time, " (", frac, "%) ###############")
            for y = 1, height do
                for x = 1, width do
                    if map[y][x] then
                        io.write("x")
                    else
                        io.write(".")
                    end
                end
                io.write("\n")
            end
            print(time)
        end

        for _, robot in ipairs(next_robots) do
            map[robot.y + 1][robot.x + 1] = false
        end
    end
end

main()
