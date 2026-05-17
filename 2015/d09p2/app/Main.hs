module Main where

import qualified Data.Bits as Bits
import qualified Data.Map as Map
import qualified Data.Set as Set
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

data Edge = Edge {to :: Int, cost :: Int} deriving (Show)

trc :: (Show a) => a -> a
trc x = traceShow x x

data Ctx = Ctx {dist :: Int, rdist :: Int, memo :: Map.Map (Int, Int) Int, its :: Int} deriving (Show)

shortest :: Int -> (Int, Int) -> Map.Map Int [Edge] -> Ctx -> Ctx
shortest n state edges ctx
  | wasHereBetter = ctx
  | didVisitAll = ctx {rdist = dist ctx}
  | otherwise = nextBest
  where
    (visited, cur) = state
    addToVisited v i = v Bits..|. (1 `Bits.shiftL` i)
    visitedAfter = addToVisited visited cur
    didVisitAll = visitedAfter == (1 `Bits.shiftL` n) - 1
    wasHereBetter = Map.findWithDefault minBound state (memo ctx) >= dist ctx
    nextMemo = Map.insert state (dist ctx) (memo ctx)
    outEdges = Map.findWithDefault [] cur edges
    isNotVisited to = visitedAfter Bits..&. addToVisited 0 to == 0
    unvisitedOutEdges = filter (\Edge {to, cost = _} -> isNotVisited to) outEdges
    bestForEdge Edge {to, cost} actx =
      let s = shortest n (visitedAfter, to) edges actx {dist = dist ctx + cost, its = succ $ its actx}
       in s {rdist = max (rdist s) (rdist actx)}
    nextBest = foldr bestForEdge (ctx {memo = nextMemo}) unvisitedOutEdges

solve :: [T.Text] -> Int
solve lns = traceShow (T.unpack "Its: " ++ show (its resultCtx)) $ rdist resultCtx
  where
    parsed =
      map
        ( \l -> case T.words l of
            [f, "to", t, "=", c] -> (f, t, read (T.unpack c) :: Int)
            _ -> error . T.unpack $ T.append "Could not parse: " l
        )
        lns
    uniqueCities = Set.toList . Set.fromList $ map (\(f, _, _) -> f) parsed ++ map (\(_, t, _) -> t) parsed
    cityToIdx = Map.fromList $ zip uniqueCities [1 :: Int ..]
    fwdEdges = map (\(f, t, c) -> (cityToIdx Map.! f, Edge {to = cityToIdx Map.! t, cost = c})) parsed
    backEdges = map (\(f, Edge {to, cost}) -> (to, Edge {to = f, cost = cost})) fwdEdges
    zeroToAll = map (\to -> (0, Edge {to = to, cost = 0})) [1 .. Map.size cityToIdx]
    edges = Map.fromListWith (++) $ map (\(f, e) -> (f, [e])) (fwdEdges ++ backEdges ++ zeroToAll)
    numNodes = succ $ Map.size cityToIdx
    resultCtx = shortest numNodes (0, 0) edges Ctx {dist = 0, rdist = minBound, memo = Map.empty, its = 0}

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
