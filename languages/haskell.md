# Functions

```hs
area :: Int -> Int -> Int
area h, w = h * w
-- Note that function can be called with less arguments to curry a new function
-- e.g. ((area 2) 3) == 6
```

```hs
-- Anonymous.
\ h, w -> h * w
```

```hs
-- Where statement.
area :: Int -> Int -> Int
area h, w = hw
  where hw = h * w
```

# Errors

```hs
error $ "Could not convert: " ++ show l -- Throw.
```

# Matching

```hs
-- Split list into first element and rest
let x:xs = "abc" in print x -- a
let x:xs = "abc" in print xs -- bc
-- Preserve original item.
let full@(x:xs) = "abc" in print full -- abc
-- Exact amount of items.
let [x, _, _] = "abc" in print x -- a
-- Several items from the top
let a:b:xs = "abc" in print b -- b
```

# Sequences

```hs
scanl (+) 0 [1..4] -- [0,1,3,6,10]
foldl (+) 0 [1..4] -- 10
```
