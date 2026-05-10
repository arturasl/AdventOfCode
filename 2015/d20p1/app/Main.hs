module Main where

import Data.List qualified as List
import Data.Text qualified as T
import Data.Text.IO qualified as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

calcDivisorSum :: [(Int, Int)] -> Int
calcDivisorSum [] = 1
calcDivisorSum ((p, n) : xs) = snd $ foldr (\_ (po, ac) -> (po * p, ac + po * other)) (1, 0) [0 .. n]
  where
    other = calcDivisorSum xs

getPrimeDivisors' :: [Int] -> Int -> [Int]
getPrimeDivisors' [] _ = error "Not enough primes"
getPrimeDivisors' primes@(p : leftPrimes) n
  | n `mod` p == 0 = p : getPrimeDivisors' primes (n `div` p)
  | p * p < n = getPrimeDivisors' leftPrimes n
  | n > 1 = [n]
  | otherwise = []

getPrimeDivisors :: [Int] -> Int -> [(Int, Int)]
getPrimeDivisors primes n = map (\l -> (head l, length l)) . List.group $ getPrimeDivisors' primes n

inHouse :: [Int] -> Int -> Int
inHouse primes h = (10 *) . calcDivisorSum $ getPrimeDivisors primes h

solve :: T.Text -> Int
solve ln = succ . length $ takeWhile ((traget >) . inHouse primes) [1 ..]
  where
    primes = foldl (\cur n -> cur ++ ([n | not (any (\p -> n `mod` p == 0) cur)])) [2] [3 .. 100000] :: [Int]
    traget = read $ T.unpack ln :: Int

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ map solve lns
