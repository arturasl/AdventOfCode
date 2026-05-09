module Main where

import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

data Deer = Deer {speed :: Int, flightTime :: Int, restTime :: Int, points :: Int} deriving (Show)

parseLine :: T.Text -> Deer
parseLine l = parseLine' $ T.words l
  where
    tToI t = read $ T.unpack t
    parseLine' :: [T.Text] -> Deer
    parseLine' [_, "can", "fly", speed, "km/s", "for", flightTime, "seconds,", "but", "then", "must", "rest", "for", restTime, "seconds."] =
      Deer {speed = tToI speed, flightTime = tToI flightTime, restTime = tToI restTime, points = 0}
    parseLine' o = error . T.unpack $ T.unwords o

timeToPos :: Int -> Deer -> Int
timeToPos t Deer {speed, flightTime, restTime} = pos
  where
    fullFlights = t `div` (flightTime + restTime)
    timeLeft = t `mod` (flightTime + restTime)
    pos = (fullFlights * flightTime + min timeLeft flightTime) * speed

solve :: [T.Text] -> Int
solve lns = maximum $ pointsVecs !! 2503
  where
    deers = map parseLine lns
    positionsVecs = map (\t -> map (timeToPos t) deers) [1 ..]
    isWinningVecs = map (\v -> let winningPos = maximum v in map (\p -> fromEnum (p == winningPos)) v) positionsVecs
    zeroVec = replicate (length deers) 0
    pointsVecs = scanl (zipWith (+)) zeroVec isWinningVecs

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
