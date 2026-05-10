module Main where

import qualified Data.Map as Map
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

fit :: Int -> [Int] -> Int -> Map.Map Int Int
fit 0 _ u = Map.singleton u 1
fit _ [] _ = Map.empty
fit n (x : xs) u
  | n < 0 = Map.empty
  | otherwise = Map.unionWith (+) (fit (n - x) xs (succ u)) (fit n xs u)

solve :: [T.Text] -> Int
solve lns = numCombinationsPerLen Map.! shortestLen
  where
    containers = map (\l -> read $ T.unpack l :: Int) lns
    numCombinationsPerLen = fit 150 containers 0
    shortestLen = head $ Map.keys numCombinationsPerLen

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
