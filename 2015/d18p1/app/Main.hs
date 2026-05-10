module Main where

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

around :: Dims -> Point -> Grid
around (h, w) (y, x) =
  Set.fromList
    [ (ny, nx)
    | dy <- [-1 .. 1],
      let ny = y + dy,
      0 <= ny && ny < h,
      dx <- [-1 .. 1],
      let nx = x + dx,
      0 <= nx && nx < w,
      dy /= 0 || dx /= 0
    ]

collectInteresting :: Dims -> Grid -> Grid
collectInteresting dims g = Set.unions . map (around dims) $ Set.toList g

nextState :: Dims -> Grid -> Grid
nextState dims g = Set.fromList . Maybe.mapMaybe nextStateFor $ Set.toList interesting
  where
    interesting = collectInteresting dims g
    checkIsOn pos = pos `Set.member` g
    countOnAround pos = sum . map (fromEnum . checkIsOn) . Set.toList $ around dims pos
    nextStateFor pos
      | isOn && (onAround == 2 || onAround == 3) = Just pos
      | not isOn && onAround == 3 = Just pos
      | otherwise = Nothing
      where
        onAround = countOnAround pos
        isOn = checkIsOn pos

solve :: [T.Text] -> Int
solve lns = length (allStates !! 100)
  where
    dims = (length lns, T.length $ head lns)
    initGrid =
      Set.fromList . concatMap Maybe.catMaybes $
        zipWith
          ( \l y ->
              zipWith
                (\c x -> if c == '#' then Just (y, x) else Nothing)
                (T.unpack l)
                [0 :: Int ..]
          )
          lns
          [0 :: Int ..]
    allStates = iterate (nextState dims) initGrid

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
