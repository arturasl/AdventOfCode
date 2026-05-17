module Main where

import qualified Data.List as List
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

lookAndSay :: T.Text -> [T.Text]
lookAndSay t = t : lookAndSay tNext
  where
    tNext = T.pack . (concatMap (\g -> show (length g) ++ [head g]) . List.group) $ T.unpack t

solve :: T.Text -> Int
solve ln = T.length $ lookAndSay ln !! 40

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  mapM_ (print . solve) lns
