module Main where

import qualified Data.Char as Char
import Data.Text (pattern (:<))
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

hasIncreasingStraight :: T.Text -> Bool
hasIncreasingStraight s = any isStraight $ T.tails s
  where
    isStraight (a :< b :< c :< _) = succ a == b && succ b == c
    isStraight _ = False

noProhibitedChars :: T.Text -> Bool
noProhibitedChars s = not $ T.any (`T.elem` "iol") s

hasTwoSame :: T.Text -> Bool
hasTwoSame s = hasSingle || hasTwoDifferent
  where
    grps = map T.length $ T.group s
    hasSingle = any (>= 4) grps
    hasTwoDifferent = length (filter (>= 2) grps) >= 2

isValid :: T.Text -> Bool
isValid s = hasIncreasingStraight s && noProhibitedChars s && hasTwoSame s

nextPwd :: T.Text -> T.Text
nextPwd s = snd $ T.foldr inc (1, "") s
  where
    inc :: Char -> (Int, T.Text) -> (Int, T.Text)
    inc ch (acc, res) =
      let nxt = Char.ord ch + acc
       in if nxt == Char.ord 'z' + 1
            then (1, T.cons 'a' res)
            else (0, T.cons (Char.chr nxt) res)

nextOkPwd :: T.Text -> T.Text
nextOkPwd s = head . dropWhile (not . isValid) $ iterate nextPwd s

solve :: T.Text -> T.Text
solve = nextOkPwd . nextPwd . nextOkPwd

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  mapM_ (print . solve) lns
