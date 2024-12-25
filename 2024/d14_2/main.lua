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

local function draw_robots(time, robots, height, width)
    print("############### ", time, " ###############")

    local map = {}
    for y = 1, height do
        map[y] = {}
        for x = 1, width do
            map[y][x] = "."
        end
    end

    for _, robot in ipairs(robots) do
        map[robot.y + 1][robot.x + 1] = "x"
    end

    for y = 1, height do
        for x = 1, width do
            io.write(map[y][x])
        end
        io.write("\n")
    end
    print(time)
end

local function neighbour_based(robots, height, width)
    local around = {
        { y = 0, x = 1 },
        { y = -1, x = 0 },
        { y = -1, x = 1 },
        { y = -1, x = -1 },
    }

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
        for i, robot in ipairs(next_robots) do
            local ok = false
            for _, d in ipairs(around) do
                local ny, nx = robot.y + d.y + 1, robot.x + d.x + 1
                ok = ok or (1 <= ny and ny <= height and 1 <= nx and nx <= width and map[ny][nx])
            end
            have_around = have_around + (ok and 1 or 0)
            if have_around + (#next_robots - i) < best_score then
                break
            end
        end

        if best_score < have_around then
            best_score = have_around
            draw_robots(time, next_robots, height, width)
        end

        for _, robot in ipairs(next_robots) do
            map[robot.y + 1][robot.x + 1] = false
        end
    end
end

local function stats_based(robots, height, width)
    local next_robots = {}

    local best_score = height * width * #robots
    for time = 0, width * height + 1 do
        for i, robot in ipairs(robots) do
            next_robots[i] = {
                y = (robot.pos.y + robot.d.y * time) % height,
                x = (robot.pos.x + robot.d.x * time) % width,
            }
        end

        local avg_y, avg_x = 0, 0
        for _, robot in ipairs(next_robots) do
            avg_y = avg_y + robot.y
            avg_x = avg_x + robot.x
        end
        avg_y = avg_y / #robots
        avg_x = avg_x / #robots

        local score = 0
        for _, robot in ipairs(next_robots) do
            score = score + math.abs(avg_y - robot.y) + math.abs(avg_x - robot.x)
        end

        if score < best_score then
            best_score = score
            -- draw_robots(time, next_robots, height, width)
            print(time)
        end
    end
end

local function main()
    -- local width, height = 11, 7
    -- local time = 100
    local width, height = 101, 103
    local robots = read()

    -- neighbour_based(robots, height, width)
    stats_based(robots, height, width)
end

main()
