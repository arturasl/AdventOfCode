module Main where

import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)
import Text.Regex.TDFA (getAllTextMatches, (=~))

trc :: (Show a) => a -> a
trc x = traceShow x x

solve :: T.Text -> Int
solve ln = sum $ map read strNums
  where
    strNums = getAllTextMatches (T.unpack ln =~ ("-?[0-9]+" :: T.Text)) :: [String]

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  mapM_ (print . solve) lns
