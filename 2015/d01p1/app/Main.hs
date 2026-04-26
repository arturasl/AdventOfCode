module Main where

parens :: String -> Int
parens [] = 0
parens ('(' : xs) = 1 + parens xs
parens (')' : xs) = (-1) + parens xs
parens (_ : xs) = parens xs

main :: IO ()
main = do
  contents <- getContents
  print (parens contents)
