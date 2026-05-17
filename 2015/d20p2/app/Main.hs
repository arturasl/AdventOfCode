module Main where

import Data.IntMap qualified as IntMap
import Data.Text qualified as T
import Data.Text.IO qualified as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

defaultMul :: Int
defaultMul = 11

defaultLives :: Int
defaultLives = 50

data Ctx = Ctx {presents :: Int, futurePresents :: IntMap.IntMap Int, house :: Int}

genFuturePresents :: Int -> IntMap.IntMap Int
genFuturePresents n = IntMap.fromDistinctAscList $ [(i * n, defaultMul * n) | i <- [1 .. defaultLives]]

genNext :: Ctx -> Ctx
genNext (Ctx {futurePresents, house}) =
  Ctx
    { presents = nextPresentMap IntMap.! nextHouse,
      futurePresents = IntMap.delete nextHouse nextPresentMap,
      house = nextHouse
    }
  where
    nextHouse = succ house
    nextPresentMap = IntMap.unionWith (+) futurePresents $ genFuturePresents nextHouse

solve :: T.Text -> Int
solve ln = length $ takeWhile ((target >) . presents) ctxs
  where
    target = read $ T.unpack ln :: Int
    ctxs = iterate genNext Ctx {presents = 0, futurePresents = IntMap.empty, house = 0}

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  mapM_ (print . solve) lns
