local lhs_counts, rhs_lst = {}, {}
while true do
	local lhs, rhs = io.read("*n", "*n")
	if lhs == nil then
		assert(rhs == nil)
		break
	end

	lhs_counts[lhs] = (lhs_counts[lhs] or 0) + 1
	rhs_lst[#rhs_lst + 1] = rhs
end

local result = 0
for i = 1, #rhs_lst do
	result = result + rhs_lst[i] * (lhs_counts[rhs_lst[i]] or 0)
end

print(result)
