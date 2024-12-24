local dump = require("dump")

local function solve(prob)
    local min_cost = nil
    for a_presses = 0, 100 do
        for b_presses = 0, 100 do
            local x = a_presses * prob.buttons["A"].dx + b_presses * prob.buttons["B"].dx
            local y = a_presses * prob.buttons["A"].dy + b_presses * prob.buttons["B"].dy
            if prob.prize.x == x and prob.prize.y == y then
                local cost = a_presses * 3 + b_presses * 1
                if min_cost == nil or min_cost > cost then
                    min_cost = cost
                end
            end
        end
    end
    return min_cost
end

local function read()
    local probs = {}
    local prob = {}
    for line in io.lines() do
        if line:gsub("%s+", "") ~= "" then
            local button, dx, dy = line:match("^Button ([AB]): X([+-]%d+), Y([+-]%d+)$")
            local prize_x, prize_y = line:match("^Prize: X=(%d+), Y=(%d+)$")
            if button ~= nil then
                dx, dy = tonumber(dx), tonumber(dy)
                assert(dx ~= nil and dy ~= nil)
                prob.buttons = prob.buttons or {}
                assert(prob.buttons[button] == nil)
                prob.buttons[button] = { dx = dx, dy = dy }
            else
                prize_x, prize_y = tonumber(prize_x), tonumber(prize_y)
                assert(prize_x ~= nil and prize_y ~= nil)
                prob.prize = { x = prize_x, y = prize_y }
                probs[#probs + 1] = prob
                prob = {}
            end
        end
    end
    assert(prob.prize == nil)
    return probs
end

local function main()
    local result = 0
    for _, prob in ipairs(read()) do
        result = result + (solve(prob) or 0)
    end
    print(result)
end

main()
