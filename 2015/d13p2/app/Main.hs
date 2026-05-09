module Main where

import qualified Data.List as List
import qualified Data.Map as Map
import qualified Data.Set as Set
import Data.Text (pattern (:>))
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Data.Tuple.Utils (fst3)
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

type Graph = Map.Map (T.Text, T.Text) Int

parseTriples :: [T.Text] -> [(T.Text, Int, T.Text)]
parseTriples = map parseTriple
  where
    parseTriple l = parseTriple' $ T.words l
    parseTriple' :: [T.Text] -> (T.Text, Int, T.Text)
    parseTriple' [f, "would", gainOrLose, amount, "happiness", "units", "by", "sitting", "next", "to", t :> '.'] =
      ( f,
        ( case gainOrLose of
            "gain" -> id
            "lose" -> negate
            _ -> error $ T.unpack gainOrLose
        )
          (read (T.unpack amount)),
        t
      )
    parseTriple' l = error . T.unpack $ T.unwords l

calcCost :: Graph -> [T.Text] -> Int
calcCost g l = calcFwdCost withLast + calcFwdCost (reverse withLast)
  where
    withLast = l ++ [head l]
    calcFwdCost els = sum (zipWith (curry (g Map.!)) els (tail els))

solve :: [T.Text] -> Int
solve lns = maximum . map (\n -> calcCost graph ("" : n)) $ List.permutations originalNodes
  where
    triplesWoMe = parseTriples lns
    originalNodes = Set.toList . Set.fromList $ map fst3 triplesWoMe
    triples = List.concat (triplesWoMe : map (\n -> [(n, 0, ""), ("", 0, n)]) originalNodes)
    graph = Map.fromList $ map (\(f, a, t) -> ((f, t), a)) triples

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
