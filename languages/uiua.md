```uiua
# Use swith to jump in between which numbers 11 is. This is done by first
# computing above how many of elements 11 is above:
# /+≥[10 100] 11
# /+[1 0]
# 1
# And then jumping to that swhich function.
⨬("small"|"medium"|"large")/+≥[10 100] 11
```

```uiua
# Split the string on `,`. This is done by undoing format "_,_", recursively
(well un-reduced).
°/$"_,_" "a,b,c"
```

```uiua
# Simple pattern matching -- check with what letter string starts and return
# branch number
"abc"
⍣( #
  1°$"a_"
| 2°$"b_"
| 3°$"c_"
| 4
)
⊙⋅
```

```uiua
# Puth the next three values to an array.
⊟₃ 1 2 3
```

```uiua
# Multiple the second value by 10
⍜(⊡2|×10) [1 2 3 4]
```

```uiua
# Add second and 4th values together.
+⋅⊙⋅∘ 1 2 3 4
```
