module Main where

import qualified Data.List as List
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

parseLine :: T.Text -> [Int]
parseLine l = init ints
  where
    elsStr = T.splitOn ":" l !! 1
    els = map T.strip $ T.splitOn "," elsStr
    tToI t = read $ T.unpack t
    ints = map (\e -> tToI (T.words e !! 1)) els

elsAddingTo :: Int -> Int -> [[Int]]
elsAddingTo n l
  | l == 0 = [replicate n 0]
  | n == 0 = []
  | otherwise = concatMap (\ll -> map (ll :) $ elsAddingTo (pred n) (l - ll)) [0 .. l]

solve :: [T.Text] -> Int
solve lns = maximum scoreAttempts
  where
    propsPerIngrediant = map parseLine lns
    numIngredients = length propsPerIngrediant
    ingredientMulAttempts = elsAddingTo numIngredients 100
    applyMullToIngredientProps mul = map (mul *)
    ingredientAttemps = map (\mulPerIngredient -> zipWith applyMullToIngredientProps mulPerIngredient propsPerIngrediant) ingredientMulAttempts
    scoreAttempts = map ((product . map (max 0 . sum)) . List.transpose) ingredientAttemps

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
