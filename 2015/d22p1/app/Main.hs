module Main where

import Data.Function (on)
import Data.List qualified as List
import Data.Maybe
import Data.Text qualified as T
import Data.Text.IO qualified as TIO
import Debug.Trace (traceShow)

trc :: (Show a) => a -> a
trc t = traceShow t t

data Stats = Stats {life :: Int, armour :: Int, mana :: Int, manaUsed :: Int} deriving (Show)

empty :: Stats
empty = Stats {life = 0, armour = 0, mana = 0, manaUsed = 0}

data PType = Knight | Boss deriving (Show, Eq, Ord)

data EType = Shield | Poison | Recharge deriving (Show, Eq, Ord)

data Effect = Effect
  { tp :: EType,
    timeLeft :: Int
  }
  deriving (Show)

data Player = Player
  { tp :: PType,
    stats :: Stats,
    effects :: [Effect]
  }
  deriving (Show)

unMaybePair :: (Maybe a, Maybe b) -> Maybe (a, b)
unMaybePair (ma, mb) = (,) <$> ma <*> mb

changeStats :: Stats -> Player -> Player
changeStats
  Stats {life = cLife, armour = cArmour, mana = cMana, manaUsed = cManaUsed}
  p@Player {stats = Stats {life = oLife, armour = oArmour, mana = oMana, manaUsed = oManaUsed}} =
    p
      { stats =
          Stats
            { life = oLife + if cLife < 0 then min (cLife + oArmour) (-1) else cLife,
              armour = oArmour + cArmour,
              mana = oMana + cMana,
              manaUsed = oManaUsed + cManaUsed
            }
      }

reducingMana :: Int -> (Player, Player) -> ((Player, Player) -> Maybe (Player, Player)) -> Maybe (Player, Player)
reducingMana manaCost (k, b) f
  | (mana . stats) k >= manaCost = f (changeStats (empty {mana = -manaCost, manaUsed = manaCost}) k, b)
  | otherwise = Nothing

applyAction :: Int -> (Stats, Stats) -> (Player, Player) -> Maybe (Player, Player)
applyAction manaCost (changeKStats, changeBStats) players =
  reducingMana
    manaCost
    players
    (\(k, b) -> Just (changeStats changeKStats k, changeStats changeBStats b))

applyMagicMissle :: (Player, Player) -> Maybe (Player, Player)
applyMagicMissle = applyAction 53 (empty, empty {life = -4})

applyDrain :: (Player, Player) -> Maybe (Player, Player)
applyDrain = applyAction 73 (empty {life = 2}, empty {life = -2})

applyHit :: Int -> (Player, Player) -> Maybe (Player, Player)
applyHit damage = applyAction 0 (empty, empty {life = -damage})

addAffect :: Int -> Effect -> PType -> (Player, Player) -> Maybe (Player, Player)
addAffect manaCost e toWhom players =
  reducingMana
    manaCost
    players
    (\(k, b) -> unMaybePair (addTo Knight k, addTo Boss b))
  where
    addTo t p@Player {effects}
      | t /= toWhom = Just p
      | any (\te -> te.tp == e.tp) effects = Nothing
      | otherwise = Just p {effects = List.sortBy (compare `on` (.tp)) $ e : effects}

addPoison :: (Player, Player) -> Maybe (Player, Player)
addPoison = addAffect 173 Effect {tp = Poison, timeLeft = 6} Boss

addRecharge :: (Player, Player) -> Maybe (Player, Player)
addRecharge = addAffect 229 Effect {tp = Recharge, timeLeft = 5} Knight

addShield :: (Player, Player) -> Maybe (Player, Player)
addShield = addAffect 113 Effect {tp = Shield, timeLeft = 6} Knight

applyEffect :: Player -> Effect -> Player
applyEffect p Effect {tp = Shield, timeLeft} = case timeLeft of
  6 -> changeStats (empty {armour = 7}) p
  1 -> changeStats (empty {armour = -7}) p
  _ -> p
applyEffect p Effect {tp = Recharge} = changeStats (empty {mana = 101}) p
applyEffect p Effect {tp = Poison} = changeStats (empty {life = -3}) p

applyExistingEffects :: Player -> Player
applyExistingEffects p@Player {effects} = playerAfterEffects {effects = leftEffects}
  where
    playerAfterEffects = foldl applyEffect p effects
    leftEffects = filter (\Effect {timeLeft} -> timeLeft > 0) $ map (\e@Effect {timeLeft} -> e {timeLeft = timeLeft - 1}) effects

getActions :: Player -> [(Player, Player) -> Maybe (Player, Player)]
getActions Player {tp = Knight} = [applyMagicMissle, applyDrain, addPoison, addRecharge, addShield]
getActions Player {tp = Boss} = [applyHit 9]

data Ctx = Ctx {minMana :: Int, searchSpace :: [(Player, Player)]} deriving (Show)

search :: Ctx -> Ctx
search ctx@Ctx {searchSpace = []} = ctx
search Ctx {minMana, searchSpace = ((k, b) : searchSpace)}
  | k.stats.manaUsed >= minMana = justContinue
  | k.stats.life <= 0 || kAfterEffects.stats.life <= 0 = handleDeath k b
  | b.stats.life <= 0 || bAfterEffects.stats.life <= 0 = handleDeath b k
  | otherwise = search Ctx {minMana, searchSpace = newStates ++ searchSpace}
  where
    justContinue = search Ctx {minMana, searchSpace}
    (kAfterEffects, bAfterEffects) = (applyExistingEffects k, applyExistingEffects b)
    newStates = map (\(k', b') -> (b', k')) . mapMaybe (\f -> f (kAfterEffects, bAfterEffects)) $ getActions kAfterEffects
    handleDeath l r = if l.tp == Knight then justContinue else search Ctx {minMana = min minMana r.stats.manaUsed, searchSpace}

solve :: [T.Text] -> Int
solve lns = minMana (search Ctx {minMana = 2000, searchSpace = [(knight, boss)]})
  where
    knight = Player {tp = Knight, stats = Stats {life = 50, armour = 0, mana = 500, manaUsed = 0}, effects = []}
    bLife = read . T.unpack . last $ T.splitOn ": " (head lns)
    boss = Player {tp = Boss, stats = Stats {life = bLife, armour = 0, mana = 0, manaUsed = 0}, effects = []}

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
