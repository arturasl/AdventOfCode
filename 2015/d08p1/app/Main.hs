{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE PatternSynonyms #-}

module Main where

import Data.Char as Char
import Data.Text (pattern Empty, pattern (:<), pattern (:>))
import qualified Data.Text as T
import qualified Data.Text.IO as TIO

escape' :: T.Text -> T.Text
escape' Empty = T.empty
escape' ('\\' :< 'x' :< a :< b :< xs) = T.cons c $ escape' xs
  where
    ca = Char.digitToInt a
    cb = Char.digitToInt b
    c = Char.chr $ ca * 16 + cb
escape' ('\\' :< x :< xs) = T.cons x $ escape' xs
escape' (x :< xs) = T.cons x $ escape' xs

escape :: T.Text -> T.Text
escape ('"' :< (s :> '"')) = escape' s
escape s = error . T.unpack $ T.concat ["`", s, "` is not surrounded with \""]

solve :: [T.Text] -> Int
solve lns = slens lns - slens escaped
  where
    slens ls = sum $ map T.length ls
    escaped = map escape lns

main :: IO ()
main = do
  contents <- TIO.getContents
  let lns = filter (not . T.null) . map T.strip $ T.lines contents
  print $ solve lns
