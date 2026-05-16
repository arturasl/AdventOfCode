module Main where

import Combinatorics qualified as C
import Data.Bits qualified as Bits
import Data.List qualified as List
import Data.Ord qualified as O
import Data.Text qualified as T
import Data.Text.IO qualified as TIO
import Debug.Trace (traceShow)

splitable2 :: [Int] -> Bool
splitable2 els = even s && r `Bits.testBit` (s `div` 2)
  where
    s = sum els
    r = foldl (\msk el -> msk Bits..|. (msk `Bits.shift` el)) (1 :: Integer) els

splitable3 :: [Int] -> Bool
splitable3 els = s `mod` 3 == 0 && okTwos
  where
    s = sum els
    sExpected = s `div` 3
    okTwos = any (\(l, r) -> sum l == sExpected && splitable2 r) $ C.partitions els

data El = El {alreadyUsed :: [Int], sAlreadyUsed :: Int, canUse :: [Int], sCanUseAndSkipped :: Int, skipped :: [Int]}

data CompareKey = CompareKey {elLen :: Int, elQe :: Integer} deriving (Show, Eq, Ord)

createKey :: [Int] -> CompareKey
createKey els = CompareKey {elLen = length els, elQe = product $ map toInteger els}

find' :: [El] -> CompareKey -> CompareKey
find' [] r = r
find' (El {alreadyUsed = au, sAlreadyUsed = sau, canUse = cu, sCanUseAndSkipped = ss, skipped = s} : searchSpace) r
  | createKey au >= r = find' searchSpace r
  | 3 * sau > ss = find' searchSpace r
  | 3 * sau == ss =
      if splitable3 (s ++ cu)
        then
          traceShow (show (createKey au) ++ ", au: " ++ show au ++ ", cu: " ++ show cu ++ ", s: " ++ show s)
            . find' searchSpace
            . min r
            $ createKey au
        else
          find' searchSpace r
  | null cu = find' searchSpace r
  | otherwise =
      find'
        ( El {alreadyUsed = c : au, sAlreadyUsed = sau + c, canUse = ncu, sCanUseAndSkipped = ss - c, skipped = s}
            : El {alreadyUsed = au, sAlreadyUsed = sau, canUse = ncu, sCanUseAndSkipped = ss, skipped = c : s}
            : searchSpace
        )
        r
  where
    c = head cu
    ncu = tail cu

find :: [Int] -> Integer
find weights = elQe $ find' [El {alreadyUsed = [], sAlreadyUsed = 0, canUse = weights, sCanUseAndSkipped = sum weights, skipped = []}] CompareKey {elLen = length weights, elQe = 0}

solve :: [T.Text] -> Integer
solve lns = find weights
  where
    weights = List.sortBy (O.comparing O.Down) $ map (read . T.unpack) lns :: [Int]

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
