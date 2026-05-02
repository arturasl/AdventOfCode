{-# LANGUAGE OverloadedStrings #-}

module Main where

import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

solve :: T.Text -> Bool
solve s = enough_vovels && wo_prohibited && has_repeated
  where
    enough_vovels = (>= 3) . T.length $ T.filter (`T.elem` "aeiou") s
    wo_prohibited = not $ any (`T.isInfixOf` s) ["ab", "cd", "pq", "xy"]
    has_repeated = any (uncurry (==)) $ T.zip s (T.tail s)

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) $ map T.strip $ T.lines contents
  print $ length $ filter solve lns
