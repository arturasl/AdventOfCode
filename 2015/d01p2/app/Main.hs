module Main where

import Data.Char (isSpace)

trim :: String -> String
trim = f . f
  where
    f = reverse . dropWhile isSpace

parens' :: String -> Int -> Int -> Int
parens' _ p (-1) = p
parens' [] _ _ = error "Did not reach basement"
parens' (')' : xs) p l = parens' xs (succ p) (pred l)
parens' ('(' : xs) p l = parens' xs (succ p) (succ l)
parens' (x : _) _ _ = error ("Unknown character: `" ++ [x] ++ "`")

parens :: String -> Int
parens s = parens' s 0 0

main :: IO ()
main = do
  contents <- getContents
  print (parens (trim contents))
