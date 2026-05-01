{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE PatternSynonyms #-}
{-# LANGUAGE ViewPatterns #-}

module Main where

import qualified Data.Set as Set
import qualified Data.Text as T
import qualified Data.Text.IO as TIO

pattern (:<) :: Char -> T.Text -> T.Text
pattern c :< rest <- (T.uncons -> Just (c, rest))

nextPos :: Char -> (Int, Int) -> (Int, Int)
nextPos '>' (cy, cx) = (cy, cx + 1)
nextPos '<' (cy, cx) = (cy, cx - 1)
nextPos '^' (cy, cx) = (cy - 1, cx)
nextPos 'v' (cy, cx) = (cy + 1, cx)
nextPos c _ = error $ "Unknown character" ++ [c]

solve' :: T.Text -> (Int, Int) -> Set.Set (Int, Int) -> Int
solve' "" _ visited = Set.size visited
solve' (c :< s) cur visited = solve' s next_pos (Set.insert next_pos visited)
  where
    next_pos = nextPos c cur
solve' _ _ _ = error "?"

solve :: T.Text -> Int
solve s = solve' s (0, 0) (Set.singleton (0, 0))

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) $ map T.strip $ T.lines contents
  print $ map solve lns
