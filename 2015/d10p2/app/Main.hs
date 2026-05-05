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

solve :: T.Text -> [T.Text]
solve ln = traceShow (T.length $ lookAndSay ln !! 50) $ take 10 $ lookAndSay ln

--                                                   3113322113
--                                                 132123222113
--                                           111312111213322113
--                                       3113111231121123222113
--                                 1321133112132112211213322113
--                       11131221232112111312212221121123222113
--             311311221112131221123113112211322112211213322113
-- 132113212231121113112221121321132122211322212221121123222113

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ map solve lns
