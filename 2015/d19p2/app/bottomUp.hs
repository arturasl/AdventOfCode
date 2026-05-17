module Main where

import Data.Bifunctor qualified as Bi
import Data.List qualified as List
import Data.Map.Strict qualified as Map
import Data.Set qualified as Set
import Data.Text qualified as T
import Data.Text.IO qualified as TIO
import Debug.Trace (traceShow)
import System.IO (BufferMode (NoBuffering), hSetBuffering, stderr)

trc :: (Show a) => a -> a
trc x = traceShow x x

data Ctx = Ctx {memo :: !(Map.Map T.Text Int), its :: !Int} deriving (Show)

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

type PriorityKey = (Int, Int, Int)

empty :: PriorityKey
empty = (0, 0, 0)

simpleDiff :: T.Text -> T.Text -> PriorityKey
-- simpleDiff cur target = (-samePrefixLen, 0, -srtVal)
-- simpleDiff cur target = (-(samePrefixLen * srtVal * sameSuffixLen), 0, 0)
simpleDiff cur target = (-srtVal, samePrefixLen, sameSuffixLen)
  where
    samePrefixLen = length . takeWhile (uncurry (==)) $ T.zip cur target
    sameSuffixLen = length . takeWhile (uncurry (==)) $ T.zip (T.reverse cur) (T.reverse target)
    srtLetters w = List.sort $ T.unpack w
    srtVal = length . takeWhile (uncurry (==)) $ zip (srtLetters cur) (srtLetters target)

-- CRnSiRnFYCaRnFArArFArAl
search' :: T.Text -> Set.Set (PriorityKey, T.Text) -> [(T.Text, T.Text)] -> Ctx -> Ctx
search' target origSearchSpace rules ctx@Ctx {memo, its}
  | its > 10000000 = ctx
  | Set.null origSearchSpace = ctx
  | T.length t >= T.length target = search' target searchSpace rules Ctx {memo, its = nextIts}
  | otherwise = search' target nextSearchSpace rules Ctx {memo = nextMemo, its = nextIts}
  where
    t = snd $ Set.elemAt 0 origSearchSpace
    searchSpace = Set.drop 1 origSearchSpace
    nextIts =
      ( if its `mod` 100 == 0
          then
            traceShow
              ( "its: "
                  ++ show its
                  ++ ", space: "
                  ++ show (length searchSpace)
                  ++ ", memo: "
                  ++ show (length memo)
                  ++ ", ans: "
                  ++ show (Map.findWithDefault (-1) target memo)
                  ++ ", longest: "
                  ++ show (maximum . map T.length $ Map.keys memo)
                  ++ ", shortest: "
                  ++ show (minimum . map (\k -> (T.length k, k)) $ Map.keys memo)
                  ++ ", furthest: "
                  ++ show (maximum $ Map.elems memo)
                  ++ ", cur: "
                  ++ T.unpack t
              )
          else id
      )
        $ its + 1
    dist = memo Map.! t
    nextTs = applyRules t rules
    addSearchSpace = filter (\(nt, nd) -> nd < Map.findWithDefault (nd + 1) nt memo) $ map (,dist + 1) (Set.toList nextTs)
    nextMemo = Map.fromList addSearchSpace `Map.union` memo
    nextSearchSpace = Set.fromList (map (\(f, _) -> (simpleDiff f target, f)) addSearchSpace) `Set.union` searchSpace

search :: T.Text -> [(T.Text, T.Text)] -> Ctx
search t rules = search' t (Set.singleton (empty, "e")) rules $ Ctx {memo = Map.singleton "e" 0, its = 0}

solve :: [T.Text] -> Int
-- solve lns = traceShow (applyRules "CaCaCaCaCa" swappedRules) 0
-- its: 131056, res: 16
solve lns = traceShow (its resultCtx) (Map.findWithDefault (-1) molecule $ memo resultCtx)
  where
    (ruleStrs, molecule) = (init lns, last lns)
    rules = map parseRule ruleStrs
    resultCtx = search molecule rules

main :: IO ()
main = do
  hSetBuffering stderr NoBuffering
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
