module Main where

import qualified Data.Hash.MD5 as H
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (trace)

dbg :: (Show a) => a -> a
dbg x = trace (show x) x

isOkHash :: T.Text -> Bool
isOkHash s = all (== '0') $ take 6 md5
  where
    md5 = H.md5s $ H.Str $ T.unpack s

solve :: T.Text -> Int
solve s = length $ takeWhile not oks
  where
    attempts = map (\d -> T.append s (T.pack $ show d)) [0 :: Int ..]
    oks = map isOkHash attempts

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) $ map T.strip $ T.lines contents
  print $ map solve lns
