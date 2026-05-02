{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TupleSections #-}

module Main where

import Data.Bifunctor as Bi
import qualified Data.Bits as B
import Data.Char (isDigit)
import qualified Data.Map as M
import Data.Maybe
import qualified Data.Set as S
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Debug.Trace (traceShow)

msk :: Int
msk = (1 `B.shiftL` 16) - 1

data Var
  = VarConst Int
  | VarNamed T.Text
  deriving (Show)

data Op
  = OpSet Var
  | OpAnd Var Var
  | OpOr Var Var
  | OpLShift Var Var
  | OpRShift Var Var
  | OpNot Var
  deriving (Show)

data Ctx = Ctx
  { getRDeps :: M.Map T.Text (S.Set T.Text),
    getRToOp :: M.Map T.Text Op,
    getRToInt :: M.Map T.Text Int
  }
  deriving (Show)

toInt :: T.Text -> Int
toInt = read . T.unpack

parseVar :: T.Text -> Var
parseVar s
  | T.all isDigit s = VarConst $ toInt s
  | otherwise = VarNamed s

parseIns :: T.Text -> (T.Text, Op)
parseIns s
  | [v, "->", r] <- wrds = (r, OpSet (parseVar v))
  | ["NOT", v, "->", r] <- wrds = (r, OpNot (parseVar v))
  | [lhs, op, rhs, "->", r] <- wrds =
      ( r,
        ( case op of
            "AND" -> OpAnd
            "OR" -> OpOr
            "LSHIFT" -> OpLShift
            "RSHIFT" -> OpRShift
            _ -> error . T.unpack $ T.append "Unknown binary operator: " op
        )
          (parseVar lhs)
          (parseVar rhs)
      )
  | otherwise = error . T.unpack $ T.append "Could not parse:" s
  where
    wrds = T.words s

maybeResolve :: Var -> M.Map T.Text Int -> Maybe Int
maybeResolve (VarConst v) _ = Just v
maybeResolve (VarNamed n) m = n `M.lookup` m

varName :: Var -> Maybe T.Text
varName (VarConst _) = Nothing
varName (VarNamed n) = Just n

maybeEval' :: Op -> (Var -> Maybe Int) -> Maybe Int
maybeEval' (OpSet v) r = r v
maybeEval' (OpNot v) r = fmap B.complement (r v)
maybeEval' (OpAnd lhs rhs) r = liftA2 (B..&.) (r lhs) (r rhs)
maybeEval' (OpOr lhs rhs) r = liftA2 (B..|.) (r lhs) (r rhs)
maybeEval' (OpLShift lhs rhs) r = liftA2 B.shiftL (r lhs) (r rhs)
maybeEval' (OpRShift lhs rhs) r = liftA2 B.shiftR (r lhs) (r rhs)

maybeEval :: Op -> M.Map T.Text Int -> Maybe Int
maybeEval o m = fmap (B..&. msk) (maybeEval' o (`maybeResolve` m))

maybeEvalAll :: [(T.Text, Op)] -> M.Map T.Text Int -> M.Map T.Text Int
maybeEvalAll ops known =
  M.fromList $
    mapMaybe
      ( \(r, o) -> case maybeEval o known of
          Just v -> Just (r, v)
          _ -> Nothing
      )
      ops

collapse :: [T.Text] -> Ctx -> M.Map T.Text Int
collapse [] ctx = getRToInt ctx
collapse (r : rs) (Ctx r_to_deps r_to_op r_to_int) =
  collapse (M.keys solved ++ rs) (Ctx new_r_to_deps r_to_op new_r_to_int)
  where
    attempt = S.toList $ M.findWithDefault S.empty r r_to_deps
    attempt_w_op = map (\a -> (a, r_to_op M.! a)) attempt
    solved = maybeEvalAll attempt_w_op r_to_int
    new_r_to_int = r_to_int `M.union` solved
    new_r_to_deps = M.insert r S.empty r_to_deps

createRDeps :: [(T.Text, Op)] -> M.Map T.Text (S.Set T.Text)
createRDeps els = M.fromListWith S.union snd_set
  where
    op_to_vars o =
      catMaybes
        ( case o of
            (OpSet v) -> [varName v]
            (OpNot v) -> [varName v]
            (OpAnd lhs rhs) -> [varName lhs, varName rhs]
            (OpOr lhs rhs) -> [varName lhs, varName rhs]
            (OpLShift lhs rhs) -> [varName lhs, varName rhs]
            (OpRShift lhs rhs) -> [varName lhs, varName rhs]
        )
    pairs = concatMap (\(r, o) -> map (,r) $ op_to_vars o) els
    snd_set = map (second S.singleton) pairs

solve :: [T.Text] -> Int
solve lns = final_r_to_int M.! "a"
  where
    parsed =
      map
        ( ( \(r, o) -> case r of
              "b" -> (r, OpSet $ VarConst 3176)
              _ -> (r, o)
          )
            . parseIns
        )
        lns
    r_to_op = M.fromList parsed
    r_to_deps = createRDeps parsed
    solved = maybeEvalAll parsed M.empty
    final_r_to_int = collapse (M.keys solved) (Ctx r_to_deps r_to_op solved)

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
