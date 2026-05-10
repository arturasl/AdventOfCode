module Main where

import qualified Data.Bifunctor as Bi
import qualified Data.Map as Map
import qualified Data.Set as Set
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

parseRule :: T.Text -> (T.Text, T.Text)
parseRule r
  | [lhs, "=>", rhs] <- wrds = (lhs, rhs)
  | otherwise = error $ "Failed to parse: " ++ T.unpack r
  where
    wrds = T.words r

applyRule :: T.Text -> (T.Text, T.Text) -> Set.Set T.Text
applyRule t (sr, rp) = Set.fromList oks
  where
    splits = zip (T.inits t) (T.tails t)
    okSplits = filter (\(_, suffix) -> sr `T.isPrefixOf` suffix) splits
    searchLen = T.length sr
    okSplitWoSearch = map (Bi.second (T.drop searchLen)) okSplits
    oks = map (\(prefix, suffix) -> T.concat [prefix, rp, suffix]) okSplitWoSearch

applyRules :: T.Text -> [(T.Text, T.Text)] -> Set.Set T.Text
applyRules t rules = Set.unions $ map (applyRule t) rules

search' :: [(T.Text, Int)] -> [(T.Text, T.Text)] -> Map.Map T.Text Int -> Map.Map T.Text Int
search' [] _ memo = memo
search' ((t, dist) : searchSpace) rules memo
  | T.null t || "e" `T.isInfixOf` t = search' searchSpace rules memo
  | otherwise = search' nextSearchSpace rules nextMemo
  where
    nextTs = applyRules t rules
    addSearchSpace = filter (\(nt, nd) -> Map.findWithDefault (nd + 1) nt memo >= nd) $ map (,dist + 1) (Set.toList nextTs)
    nextMemo = Map.fromList addSearchSpace `Map.union` memo
    nextSearchSpace = searchSpace ++ addSearchSpace

search :: T.Text -> [(T.Text, T.Text)] -> Map.Map T.Text Int
search t rules = search' [(t, 0)] rules $ Map.singleton t 0

solve :: [T.Text] -> Int
solve lns = traceShow (Map.take 10 $ search molecule swappedRules) 0
  where
    (ruleStrs, molecule) = (init lns, last lns)
    rules = map parseRule ruleStrs
    swappedRules = map (\(l, r) -> (r, l)) rules

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
