{-# LANGUAGE OverloadedStrings #-}

module Main where

import Data.List (sort)
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

ribbon :: (Int, Int, Int) -> Int
ribbon (l, w, h) = wrap + bow
  where
    wrap = sum $ take 2 $ sort [2 * l, 2 * w, 2 * h]
    bow = product [l, w, h]

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) $ map T.strip $ T.lines contents
  let dims = map (toTpl . map toInt . T.splitOn "x") lns
  print $ sum $ map ribbon dims
