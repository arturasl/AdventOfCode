local function solve(prob)
    local b_top = (prob.buttons["A"].dx * prob.prize.y - prob.buttons["A"].dy * prob.prize.x)
    local b_bottom = (prob.buttons["A"].dx * prob.buttons["B"].dy - prob.buttons["A"].dy * prob.buttons["B"].dx)
    assert(b_bottom ~= 0)
    if b_top % b_bottom ~= 0 then
        return nil
    end
    local b_presses = b_top // b_bottom

    local a_top = (prob.prize.x - prob.buttons["B"].dx * b_presses)
    local a_bottom = prob.buttons["A"].dx
    assert(a_bottom ~= 0)
    if a_top % a_bottom ~= 0 then
        return nil
    end
    local a_presses = a_top // a_bottom
    return 3 * a_presses + b_presses
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
                prob.prize = { x = prize_x + 10000000000000, y = prize_y + 10000000000000 }
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
