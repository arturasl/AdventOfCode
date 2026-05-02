{-# LANGUAGE OverloadedStrings #-}

module Main where

import qualified Data.Set as S
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

solve :: T.Text -> Bool
solve s = with_in_between && has_repeated_pair
  where
    arr = T.unpack s
    pairs = S.toList . S.fromList $ zipWith (\a b -> T.pack [a, b]) arr (tail arr)
    has_repeated_pair = any ((>= 2) . (`T.count` s)) pairs
    with_in_between = any (\(a, _, b) -> a == b) $ zip3 arr (tail arr) (tail $ tail arr)

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) $ map T.strip $ T.lines contents
  print $ length $ filter solve lns
