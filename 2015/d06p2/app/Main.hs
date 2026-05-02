{-# LANGUAGE OverloadedStrings #-}

module Main where

import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)
import Text.Regex.TDFA ((=~))

data Action = ActionOn | ActionToggle | ActionOff deriving (Enum, Show)

data Coord = Coord {getX :: !Int, getY :: !Int} deriving (Show)

data Instruction = Instruction !Action !Coord !Coord deriving (Show)

toInt :: T.Text -> Int
toInt = read . T.unpack

matchGroups :: T.Text -> T.Text -> [T.Text]
matchGroups r s
  | T.null match = error $ "Could not match `" ++ T.unpack s ++ "`"
  | otherwise = groups
  where
    (_, match, _, groups) = s =~ r :: (T.Text, T.Text, T.Text, [T.Text])

parse :: T.Text -> Instruction
parse s
  | [action, b, l, t, r] <- parts =
      Instruction
        ( case action of
            "turn on" -> ActionOn
            "toggle" -> ActionToggle
            "turn off" -> ActionOff
            _ -> error $ "Unparsable action: " ++ T.unpack action
        )
        (Coord (toInt b) (toInt l))
        (Coord (toInt t) (toInt r))
  | otherwise = error $ "Were not able to match: " ++ T.unpack s
  where
    parts = matchGroups "^(turn on|toggle|turn off) ([[:digit:]]+),([[:digit:]]+) through ([[:digit:]]+),([[:digit:]]+)$" s

inScope :: Coord -> Instruction -> Bool
inScope (Coord px py) (Instruction _ (Coord lx ty) (Coord rx by)) =
  px >= lx && px <= rx && py >= ty && py <= by

runIns :: Int -> Instruction -> Int
runIns v (Instruction ActionOn _ _) = succ v
runIns v (Instruction ActionOff _ _) = max (pred v) 0
runIns v (Instruction ActionToggle _ _) = v + 2

runInsLst :: Coord -> [Instruction] -> Int
runInsLst pos ins = foldl runIns 0 $ filter (inScope pos) ins

solve :: [T.Text] -> Int
solve lns = sum [runInsLst (Coord y x) ins | y <- [0 .. 999], x <- [0 .. 999]]
  where
    ins = map parse lns

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) $ map T.strip $ T.lines contents
  print $ solve lns
