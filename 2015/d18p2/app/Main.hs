module Main where

import qualified Data.Map as Map
import qualified Data.Maybe as Maybe
import qualified Data.Set as Set
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

type Dims = (Int, Int)

type Point = (Int, Int)

type Grid = Set.Set Point

around :: Dims -> Point -> [(Int, Int)]
around (h, w) (y, x) =
  [ (ny, nx)
  | dy <- [-1 .. 1],
    let ny = y + dy,
    0 <= ny && ny < h,
    dx <- [-1 .. 1],
    let nx = x + dx,
    0 <= nx && nx < w,
    dy /= 0 || dx /= 0
  ]

collectPings :: Dims -> Grid -> Map.Map (Int, Int) Int
collectPings dims g = Map.fromListWith (+) . map (,1) . concatMap (around dims) $ Set.toList g

addCorners :: Dims -> Grid -> Grid
addCorners (h, w) g = foldr Set.insert g [(0, 0), (0, w - 1), (h - 1, 0), (h - 1, w - 1)]

nextState :: Dims -> Grid -> Grid
nextState dims g = addCorners dims . Set.fromList . Maybe.mapMaybe nextStateFor $ Map.keys pings
  where
    pings = collectPings dims g
    checkIsOn pos = pos `Set.member` g
    nextStateFor pos
      | isOn && (onAround == 2 || onAround == 3) = Just pos
      | not isOn && onAround == 3 = Just pos
      | otherwise = Nothing
      where
        onAround = pings Map.! pos
        isOn = checkIsOn pos

solve :: [T.Text] -> Int
solve lns = length (allStates !! 100)
  where
    dims = (length lns, T.length $ head lns)
    initGrid =
      addCorners dims . Set.fromList $
        [ (y, x)
        | (ln, y) <- zip lns [0 :: Int ..],
          (c, x) <- zip (T.unpack ln) [0 :: Int ..],
          c == '#'
        ]
    allStates = iterate (nextState dims) initGrid

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
