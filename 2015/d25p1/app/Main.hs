module Main where

import Data.Text qualified as T
import Data.Text.IO qualified as TIO
import Text.Regex.TDFA ((=~))

modPow :: Integer -> Integer -> Integer -> Integer
modPow _ 0 _ = 1
modPow n 1 _ = n
modPow n p m
  | even p = evenCase
  | otherwise = (n * evenCase) `mod` m
  where
    halfP = modPow n (p `div` 2) m
    evenCase = (halfP * halfP) `mod` m

calcNth :: Integer -> Integer
calcNth n = (20151125 * modPow 252533 (n - 1) m) `mod` m
  where
    m = 33554393

colRowToNth :: (Integer, Integer) -> Integer
colRowToNth (c, r) = ((cc - 1) * cc) `div` 2 + r
  where
    cc = c + r - 1

solve :: T.Text -> Integer
solve ln = calcNth $ colRowToNth (head nums, last nums)
  where
    nums = [read $ T.unpack x | [x] <- ln =~ ("[[:digit:]]+" :: T.Text) :: [[T.Text]]] :: [Integer]

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  mapM_ (print . solve) lns
