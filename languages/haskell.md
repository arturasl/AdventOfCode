# Functions

```hs
area :: Int -> Int -> Int
area h, w = h * w
-- Note: function can be called with less arguments to curry a new function
-- e.g. ((area 2) 3) == 6
-- Note: one can force infix calls with backticks
-- e.g. 2 `area` 3 == 6
```

```hs
-- Anonymous.
\ h, w -> h * w
```

```hs
-- Where statement with guards.
toTpl :: Int -> Int -> (Int, Int)
toTpl a, b
  | (0, _) <- ab = error "Zero as a not allowed"
  | otherwise = ab
  where ab = (a, b)
```

# Conditions

```hs
case "a" of
  "a" -> 1
  "b" -> 2
  _ -> error ":("
```

# Strings

## Regex

````hs
import Text.Regex.TDFA ((=~))
"a" =~ ".*" :: Bool -- Returns whether string matched regex
-- Returns:
-- Unmatched on left side.
-- Unmatched on right side.
-- Matched ($0)
-- Groups
"a" =~ "(.*)" :: (String, String, String, [String])
-- Helper:
matchGroups :: T.Text -> T.Text -> [T.Text]
matchGroups r s
  | T.null match = error $ "Could not match `" ++ T.unpack s ++ "`, re: `" ++ T.unpack r ++ "`"
  | otherwise = groups
  where
    (_, match, _, groups) = s =~ r :: (T.Text, T.Text, T.Text, [T.Text])
-- Getting all matches and their subgroups:
[ (k, v) | (_ : k : v : _) <- (other =~ ("([a-z]+): ([0-9]+)" :: T.Text) :: [[T.Text]]) ]
```

## Text

```hs
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE PatternSynonyms #-}
import Data.Text (pattern Empty, pattern (:<))
import qualified Data.Text as T

T.lines "a\nb\n"
T.words "a b n" -- ["a", "b", "c"]
T.strip "  b  " -- "b"
T.null "" -- True (is empty)
T.empty -- "" (empty)
T.unpack $ T.pack "abc" -- String to text and back
"a" T.isInfixOf "bab" -- True, "a" is found in "bab"
````

````hs
# Errors

```hs
error $ "Could not convert: " ++ show l -- Throw.

-- For an intermediate printf debugging.
import Debug.Trace (traceShow)
myfn :: String -> Int
myfn s = traceShow s $ length s -- Print s and return the value.
````

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

# Types

## Numeric

```hs
fromEnum True -- enum to int
a :: Int -- A simple int (goes to negative on overflow).
maxBound :: Int -- Give top bound for "bounded" types (Int, Enum, Char, etc.)
a :: Integer -- Arbitrary length integers
toInteger 1 -- Convert normal integer to arbitrary length one.
```

## Record

```hs
data Point = Create Float Float deriving (Show, Eq)
--    ^       ^                           ^-- type classes.
--    |       |--- value constructor -- function that will create a Point,
--    |             e.g. (Create 1 2) :: Point. Can also be used to pattern
--    |             match with (Create _ x)
--    --- type constructor (without parameters)
data Car = Car {name :: String, mileage :: Int} -- Record type.
-- let c = Car {name = "abc", mileage = 1}
-- (name c) == "abc"

-- Pattern match and update single field.
update :: Car -> Car
update ctx@Car {name = curName} = ctx(name = curName ++ "hello")

data Point a = Create a a
--         ^-- type constructor with single parameter
data (Ord a) => Point a = Create a a
--        ^-- type class cosntraint
data Weekend = Saturday | Sunday deriving (Bounded, Enum, Eq, Ord, Show)
type MyWeekend = Weekend -- type synonym / alias.
```

# Sequences

```hs
scanl (+) 0 [1..4] -- [0,1,3,6,10]
foldl (+) 0 [1..4] -- 10
[1, 2, 3] !! 1 -- 2 (first element)
take 2 [1, 2, 3] -- [1, 2]
head [1, 2, 3] -- 1
length [1, 2, 3] -- 3
List.concat [[1, 2, 3], [4]] -- [1, 2, 3, 4]
[1, 2, 3] ++ [4] -- [1, 2, 3, 4]
any even [1, 2, 3] -- True
filter even [1, 2, 3] -- [2]
iterate inc 0 -- Create an infinite list by applying inc to the previous result.
tails [1, 2, 3] -- [[1, 2, 3], [2, 3], [3]] Create all suffixes.
[c | (c, i) <- zip "abcdef" [(0 :: Int) ..], let alias = i, even alias] -- List comprehension.
```

```hs
import qualified Data.Map as M
M.empty -- Create empty
M.null M.empty -- Check if empty
(M.fromList [("a", 1)]) M.! "a" -- Lookup, throwing if does not exist
M.findWithDefault 0 "a" M.empty -- Lookup, using default of 0
M.lookup "a" M.empty -- Lookup, returning Nothing if does not exist
M.empty `M.union` M.empty -- Merge two maps (M.unionWith for custom merging function)
```
