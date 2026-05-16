module Main where

import Data.Map qualified as Map
import Data.Sequence qualified as Seq
import Data.Text qualified as T
import Data.Text.IO qualified as TIO

data Ins
  = Inc {reg :: T.Text}
  | Half {reg :: T.Text}
  | Triple {reg :: T.Text}
  | Jump {oft :: Int}
  | JumpEven {reg :: T.Text, oft :: Int}
  | JumpOne {reg :: T.Text, oft :: Int}
  deriving (Show)

data Ctx = Ctx
  { regs :: Map.Map T.Text Int,
    pos :: Int,
    prog :: Seq.Seq Ins
  }
  deriving (Show)

ofsetToNum :: T.Text -> Int
ofsetToNum t =
  ( case T.head t of
      '+' -> 1
      '-' -> -1
      s -> error $ "Unknown sign: " ++ [s]
  )
    * (read . T.unpack $ T.tail t)

parseLine' :: [T.Text] -> Ins
parseLine' ["hlf", r] = Half {reg = r}
parseLine' ["tpl", r] = Triple {reg = r}
parseLine' ["inc", r] = Inc {reg = r}
parseLine' ["jmp", o] = Jump {oft = ofsetToNum o}
parseLine' ["jie", r, o] = JumpEven {reg = r, oft = ofsetToNum o}
parseLine' ["jio", r, o] = JumpOne {reg = r, oft = ofsetToNum o}
parseLine' wrds = error . T.unpack $ T.unwords wrds

parseLine :: T.Text -> Ins
parseLine t = parseLine' . T.words $ T.replace "," "" t

updateReg :: Ctx -> T.Text -> (Int -> Int) -> Ctx
updateReg ctx r f = ctx {regs = newRegs}
  where
    curVal = Map.findWithDefault 0 r (regs ctx)
    newRegs = Map.insert r (f curVal) (regs ctx)

nextIns :: Ctx -> Ctx
nextIns ctx@Ctx {pos = curPos} = ctx {pos = succ curPos}

maybeJump :: Ctx -> T.Text -> Int -> (Int -> Bool) -> Ctx
maybeJump ctx r o f = ctx {pos = if f curVal then curPos + o else curPos + 1}
  where
    curPos = pos ctx
    curVal = Map.findWithDefault 0 r (regs ctx)

exec :: Ctx -> Ctx
exec ctx@Ctx {pos = curPos, prog = curProg}
  | not (0 <= curPos && curPos < progLen) = ctx
  | Half {reg} <- curIns = exec . nextIns $ updateReg ctx reg (`div` 2)
  | Triple {reg} <- curIns = exec . nextIns $ updateReg ctx reg (* 3)
  | Inc {reg} <- curIns = exec . nextIns $ updateReg ctx reg succ
  | Jump {oft} <- curIns = exec $ maybeJump ctx "null" oft (const True)
  | JumpEven {reg, oft} <- curIns = exec $ maybeJump ctx reg oft even
  | JumpOne {reg, oft} <- curIns = exec $ maybeJump ctx reg oft (== 1)
  where
    progLen = length curProg
    curIns = curProg `Seq.index` curPos

solve :: [T.Text] -> Int
solve lns = regs (exec startCtx) Map.! "b"
  where
    prog = Seq.fromList $ map parseLine lns
    startCtx = Ctx {regs = Map.empty, pos = 0, prog}

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
