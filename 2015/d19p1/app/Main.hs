module Main where

import qualified Data.Bifunctor as Bi
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
applyRule t (search, replace) = Set.fromList oks
  where
    splits = zip (T.inits t) (T.tails t)
    okSplits = filter (\(_, suffix) -> search `T.isPrefixOf` suffix) splits
    searchLen = T.length search
    okSplitWoSearch = map (Bi.second (T.drop searchLen)) okSplits
    oks = map (\(prefix, suffix) -> T.concat [prefix, replace, suffix]) okSplitWoSearch

solve :: [T.Text] -> Int
solve lns = length distincOutcomes
  where
    (ruleStrs, molecule) = (init lns, last lns)
    rules = map parseRule ruleStrs
    distincOutcomes = Set.unions $ map (applyRule molecule) rules

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
