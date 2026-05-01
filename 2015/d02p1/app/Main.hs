{-# LANGUAGE OverloadedStrings #-}

module Main where

import Data.Text (Text)
import qualified Data.Text as T
import qualified Data.Text.IO as TIO

toInt :: Text -> Int
toInt s = read (T.unpack s)

toTpl :: [Int] -> (Int, Int, Int)
toTpl [l, w, h] = (l, w, h)
toTpl l = error $ "Could not convert" ++ show l

fromTpl :: (Int, Int, Int) -> [Int]
fromTpl (l, w, h) = [l, w, h]

area :: (Int, Int, Int) -> Int
area (l, w, h) = 2 * l * w + 2 * w * h + 2 * h * l

slack :: (Int, Int, Int) -> Int
slack (l, w, h) = minimum [l * w, l * h, w * h]

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) $ map T.strip $ T.lines contents
  let dims = map (toTpl . map toInt . T.splitOn "x") lns
  let areas = map (\d -> area d + slack d) dims
  print $ sum areas
