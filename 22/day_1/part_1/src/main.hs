import Data.List
import Data.Maybe

main = do
    lines <- (fmap lines . readFile) "input"
    (print . maximum) [sum [read i :: Int | i <- g, not (null i)] | g <- grp lines]
        where grp [] = [[[]]]
              grp lst = fst (spl lst) : grp (tail (snd (spl lst)))
                  where spl l = splitAt (fromMaybe 0 (elemIndex [] l)) l
