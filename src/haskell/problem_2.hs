-- Haskell doesn't have native heterogenous lists, so we won't be able to
-- replicate the example from the problem statement. I don't consider this
-- cheating as problems are more about basic ability to code rather than
-- mastery of HLists or other solutions to heterogenous lists problem
problem_2 :: [a] -> [a] -> [a]
problem_2 [] [] = []
problem_2 [] ys = ys
problem_2 xs [] = xs
problem_2 (x:xs) (y:ys) = x : y : problem_2 xs ys
