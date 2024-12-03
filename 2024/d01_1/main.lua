local lhs_lst, rhs_lst = {}, {}
while true do
	local lhs, rhs = io.read("*n", "*n")
	if lhs == nil then
		assert(rhs == nil)
		break
	end

	lhs_lst[#lhs_lst + 1] = lhs
	rhs_lst[#rhs_lst + 1] = rhs
end

table.sort(lhs_lst)
table.sort(rhs_lst)

local result = 0
for i = 1, #lhs_lst do
	result = result + math.abs(lhs_lst[i] - rhs_lst[i])
end

print(result)
