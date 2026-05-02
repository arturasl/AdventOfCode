{-# LANGUAGE OverloadedStrings #-}

module Main where

import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)
import Text.Regex.TDFA

data Action = ActionOn | ActionToggle | ActionOff deriving (Enum, Show)

toInt :: T.Text -> Int
toInt = read . T.unpack

parse :: T.Text -> (Action, Int, Int, Int, Int)
parse s
  | (_, _, _, [action, b, l, t, r]) <- parts =
      ( case action of
          "turn on" -> ActionOn
          "toggle" -> ActionToggle
          "turn off" -> ActionOff
          _ -> error $ "Unparsable action: " ++ T.unpack action,
        toInt b,
        toInt l,
        toInt t,
        toInt r
      )
  | otherwise = error $ "Were not able to match: " ++ T.unpack s
  where
    parts = (s =~ ("^(turn on|toggle|turn off) ([[:digit:]]+),([[:digit:]]+) through ([[:digit:]]+),([[:digit:]]+)$" :: T.Text)) :: (T.Text, T.Text, T.Text, [T.Text])

inScope :: (Int, Int) -> (Action, Int, Int, Int, Int) -> Bool
inScope (y, x) (_, b, l, t, r) = b <= y && y <= t && l <= x && x <= r

runAction :: (Action, Int, Int, Int, Int) -> Bool -> Bool
runAction (ActionOn, _, _, _, _) _ = True
runAction (ActionOff, _, _, _, _) _ = False
runAction (ActionToggle, _, _, _, _) b = not b

runActions :: (Int, Int) -> [(Action, Int, Int, Int, Int)] -> Bool
runActions pos actions = foldr runAction False $ reverse $ filter (inScope pos) actions

solve :: [T.Text] -> Int
solve lns = length turned_on
  where
    actions = map parse lns
    turned_on = filter id [runActions (y, x) actions | y <- [0 .. 999], x <- [0 .. 999]]

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) $ map T.strip $ T.lines contents
  print $ solve lns
