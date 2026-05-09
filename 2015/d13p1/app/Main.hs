module Main where

import qualified Data.List as List
import qualified Data.List.Extra as LE
import qualified Data.Map as Map
import Data.Text (pattern (:>))
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Data.Tuple.Utils (fst3)
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

type Graph = Map.Map T.Text (Map.Map T.Text Int)

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

triplesToGraph :: [(T.Text, Int, T.Text)] -> Graph
triplesToGraph triples = secondMap
  where
    groupedTriples = LE.groupSortOn fst3 triples
    firstMap = Map.fromList $ map (\lst -> (fst3 $ head lst, List.sortOn fst $ map (\(_, a, t) -> (t, a)) lst)) groupedTriples
    secondMap = Map.map Map.fromList firstMap

getEdgeCost :: Graph -> T.Text -> T.Text -> Int
getEdgeCost g f t = (g Map.! f) Map.! t

calcFwdCost :: Graph -> [T.Text] -> Int
calcFwdCost _ [] = 0
calcFwdCost _ [_] = 0
calcFwdCost g (a : b : l) = getEdgeCost g a b + calcFwdCost g (b : l)

calcCost :: Graph -> [T.Text] -> Int
calcCost g l = calcFwdCost g withLast + calcFwdCost g (reverse withLast)
  where
    withLast = l ++ [head l]

solve :: [T.Text] -> Int
solve lns = maximum . map (\n -> calcCost graph (head nodes : n)) . List.permutations $ tail nodes
  where
    triples = parseTriples lns
    graph = triplesToGraph triples
    nodes = Map.keys graph

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
