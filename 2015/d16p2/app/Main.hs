module Main where

import qualified Data.Map as Map
import qualified Data.Maybe as Maybe
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)
import Text.Regex.TDFA ((=~))

trc :: (Show a) => a -> a
trc x = traceShow x x

data Sue = Sue {idx :: Int, items :: Map.Map T.Text Int} deriving (Show)

targetItems :: Map.Map T.Text (Int -> Bool)
targetItems =
  Map.fromList
    [ ("children", (== 3)),
      ("cats", (> 7)),
      ("samoyeds", (== 2)),
      ("pomeranians", (< 3)),
      ("akitas", (== 0)),
      ("vizslas", (== 0)),
      ("goldfish", (< 5)),
      ("trees", (> 3)),
      ("cars", (== 2)),
      ("perfumes", (== 1))
    ]

matchGroups :: T.Text -> T.Text -> [T.Text]
matchGroups r s
  | T.null match = error $ "Could not match `" ++ T.unpack s ++ "`, re: `" ++ T.unpack r ++ "`"
  | otherwise = groups
  where
    (_, match, _, groups) = s =~ r :: (T.Text, T.Text, T.Text, [T.Text])

parseLine :: T.Text -> Sue
parseLine t = Sue {idx = tToI idx, items}
  where
    (idx, other) =
      case matchGroups "^Sue ([[:digit:]]*): (.*)$" t of
        [i, o] -> (i, o)
        _ -> error "Should not happen"
    tToI s = (read $ T.unpack s) :: Int
    items = Map.fromList $ map (\l -> (l !! 1, tToI $ l !! 2)) (other =~ ("([[:alpha:]]*): ([[:digit:]]*),?" :: T.Text) :: [[T.Text]])

solve :: [T.Text] -> Int
solve lns
  | [idx] <- validSues = idx
  | otherwise = error $ "Expected single, but found: " ++ show validSues
  where
    sues = map parseLine lns
    validSue sue = and . Map.elems $ Map.intersectionWith (\f a -> f a) targetItems $ items sue
    validSues = Maybe.mapMaybe (\s -> if validSue s then Just (idx s) else Nothing) sues

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
