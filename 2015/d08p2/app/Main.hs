{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE PatternSynonyms #-}

module Main where

import Data.Text (pattern Empty, pattern (:<))
import qualified Data.Text as T
import qualified Data.Text.IO as TIO

unescape' :: T.Text -> T.Text
unescape' Empty = T.empty
unescape' ('\\' :< xs) = T.cons '\\' . T.cons '\\' $ unescape' xs
unescape' ('"' :< xs) = T.cons '\\' . T.cons '"' $ unescape' xs
unescape' (x :< xs) = T.cons x $ unescape' xs

unescape :: T.Text -> T.Text
unescape s = T.concat ["\"", unescape' s, "\""]

solve :: [T.Text] -> Int
solve lns = slens unescaped - slens lns
  where
    slens ls = sum $ map T.length ls
    unescaped = map unescape lns

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
