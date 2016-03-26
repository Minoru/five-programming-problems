data Op = Plus | Minus

instance Show Op where
  show Plus = " + "
  show Minus = " - "

-- We're encoding the fact that the three is going to grow to the left right
-- into the type
data Expr = I Int | N Op Expr Int

instance Show Expr where
  show (I i) = show i
  show (N op expr i) = show expr ++ show op ++ show i

problem_5 :: [String]
problem_5 = (map show) . (filter (\x -> (eval x) == 100)) $ helper [I 1] [2..9]
  where
  helper :: [Expr] -> [Int] -> [Expr]
  helper options [] = options
  helper options (x:xs) =
    let options' = concatMap (buildOptions x) options
    in helper options' xs

  buildOptions x expr@(I i) =
    (I (i*10+x)) : [ N op expr x | op <- [Plus, Minus]]
  buildOptions x expr@(N op e i) =
    (N op e (i*10+x)) : [ N op expr x | op <- [Plus, Minus]]

eval :: Expr -> Int
eval (I i) = i
eval (N Plus expr i) = (eval expr) + i
eval (N Minus expr i) = (eval expr) - i
