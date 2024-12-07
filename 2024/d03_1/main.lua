local result = 0
for lhs, rhs in io.read("*a"):gmatch("mul%((%d%d?%d?),(%d%d?%d?)%)") do
	result = result + tonumber(lhs) * tonumber(rhs)
end
print(result)
