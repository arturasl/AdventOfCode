module Main where

import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

fit :: Int -> [Int] -> Int
fit 0 _ = 1
fit _ [] = 0
fit n (x : xs)
  | n < 0 = 0
  | otherwise = fit (n - x) xs + fit n xs

solve :: [T.Text] -> Int
solve lns = fit 150 containers
  where
    containers = map (\l -> read $ T.unpack l :: Int) lns

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
