module Main where

import Combinatorics qualified as C
import Data.Text qualified as T
import Data.Text.IO qualified as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc x = traceShow x x

data ShopItem = ShopItem {cost :: Int, damage :: Int, armour :: Int} deriving (Show)

data Shop = Shop {items :: [ShopItem], minBuy :: Int, maxBuy :: Int} deriving (Show)

data Player = Player {pHitPoint :: Int, pDamage :: Int, pArmour :: Int} deriving (Show)

shops :: [Shop]
shops =
  [ Shop
      { items =
          [ ShopItem {cost = 8, damage = 4, armour = 0},
            ShopItem {cost = 10, damage = 5, armour = 0},
            ShopItem {cost = 25, damage = 6, armour = 0},
            ShopItem {cost = 40, damage = 7, armour = 0},
            ShopItem {cost = 74, damage = 8, armour = 0}
          ],
        minBuy = 1,
        maxBuy = 1
      },
    Shop
      { items =
          [ ShopItem {cost = 13, damage = 0, armour = 1},
            ShopItem {cost = 31, damage = 0, armour = 2},
            ShopItem {cost = 53, damage = 0, armour = 3},
            ShopItem {cost = 75, damage = 0, armour = 4},
            ShopItem {cost = 102, damage = 0, armour = 5}
          ],
        minBuy = 0,
        maxBuy = 1
      },
    Shop
      { items =
          [ ShopItem {cost = 25, damage = 1, armour = 0},
            ShopItem {cost = 50, damage = 2, armour = 0},
            ShopItem {cost = 100, damage = 3, armour = 0},
            ShopItem {cost = 20, damage = 0, armour = 1},
            ShopItem {cost = 40, damage = 0, armour = 2},
            ShopItem {cost = 80, damage = 0, armour = 3}
          ],
        minBuy = 0,
        maxBuy = 2
      }
  ]

ceilDiv :: Int -> Int -> Int
ceilDiv a b = a `div` b + if a `mod` b /= 0 then 1 else 0

wouldKnightWin :: Player -> Player -> Bool
wouldKnightWin knight boss = knightMoves <= bossMoves
  where
    moves a d = pHitPoint d `ceilDiv` max (pDamage a - pArmour d) 1
    knightMoves = moves knight boss
    bossMoves = moves boss knight

generateBuys :: Player -> [Shop] -> [(Player, Int)]
generateBuys p [] = [(p, 0)]
generateBuys p (s : shps) = [equiptAll cp b co | b <- buys, (cp, co) <- cond]
  where
    cond = generateBuys p shps
    buys = concatMap (`C.tuples` items s) [minBuy s .. maxBuy s]
    equipt it pl = pl {pDamage = pDamage pl + damage it, pArmour = pArmour pl + armour it}
    equiptAll pl its c = foldr (\i (cp, co) -> (equipt i cp, co + cost i)) (pl, c) its

solve :: [T.Text] -> Int
solve lns = bestCost
  where
    bossStats = map ((\x -> read x :: Int) . T.unpack . last . T.splitOn ": ") lns
    boss = Player {pHitPoint = head bossStats, pDamage = bossStats !! 1, pArmour = bossStats !! 2}
    knight = Player {pHitPoint = 100, pDamage = 0, pArmour = 0}
    knights = generateBuys knight shops
    okKnights = filter (\(k, _) -> wouldKnightWin k boss) knights
    bestCost = minimum $ map snd okKnights

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
