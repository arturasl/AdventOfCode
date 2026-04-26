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
