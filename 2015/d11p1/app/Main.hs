module Main where

import qualified Data.Char as Char
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

hasIncreasingStraight :: T.Text -> Bool
hasIncreasingStraight s = any (\(a, b, c) -> isNext a b && isNext b c) $ zip3 chrs (tail chrs) (drop 2 chrs)
  where
    chrs = T.unpack s
    isNext a b = Char.ord a + 1 == Char.ord b

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

nextPwds :: T.Text -> [T.Text]
nextPwds s = s : nextPwds (nextPwd s)

nextOkPwd :: T.Text -> T.Text
nextOkPwd s = head . dropWhile (not . isValid) $ nextPwds s

solve :: [T.Text] -> [T.Text]
solve = map nextOkPwd

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
