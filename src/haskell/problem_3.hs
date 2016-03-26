problem_3_take1, problem_3_take2 :: [Integer]

problem_3_take1 = take 100 fib
  where
  fib = 0 : 1 : zipWith (+) fib (tail fib)

-- This second solution is less idiomatic but is easier to understand for
-- non-Haskellers.
--
-- Don't use it in production, though: performance is dismal.
problem_3_take2 = take 100 fib
  where
  fibonacci n = (fib !! (n-1)) + (fib !! (n-2))
  fib = 0 : 1 : map fibonacci [2..]
