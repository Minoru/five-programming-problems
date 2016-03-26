-- Functional programming uses different building blocks, so instead of fop and
-- while loops I'm going to use folds.
--
-- I'm only using foldl' because foldl only differs in performance, and while
-- foldr operates differently, the code and the result, in this case, will be
-- the same.

import Data.List (foldl')

problem_1_fold, problem_1_recursion1, problem_1_recursion2 :: Num a => [a] -> a

problem_1_fold = foldl' (\acc x -> acc + x) 0

problem_1_recursion1 [] = 0
problem_1_recursion1 (x:xs) = x + problem_1_recursion1 xs

problem_1_recursion2 = helper 0
  where
  helper acc [] = acc
  helper acc (x:xs) =
    let acc' = acc + x
    in acc' `seq` helper acc' xs
