module Main where

import qualified Data.Aeson as J
import qualified Data.Aeson.KeyMap as KM
import Data.Maybe
import qualified Data.Scientific as Scientific
import qualified Data.Text as T
import qualified Data.Text.Encoding as TE
import qualified Data.Text.IO as TIO
import qualified Data.Vector as Vector
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

sumNums :: J.Value -> Int
sumNums (J.String _) = 0
sumNums (J.Number n) = fromJust $ Scientific.toBoundedInteger n
sumNums (J.Array v) = Vector.sum $ Vector.map sumNums v
sumNums (J.Object v)
  | "red" `elem` vals = 0
  | otherwise = sum $ map sumNums vals
  where
    vals = KM.elems v
sumNums v = error (show v)

solve :: T.Text -> Int
solve ln = sumNums $ fromJust parsed
  where
    parsed = J.decodeStrict (TE.encodeUtf8 ln) :: Maybe J.Value

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  mapM_ (print . solve) lns
