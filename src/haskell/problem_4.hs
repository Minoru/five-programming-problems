import Data.List (sort)

problem_4 :: [Integer] -> Integer
problem_4 = read . concat . reverse . sort . (map show)
