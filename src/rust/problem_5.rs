/*
### Problem 5 ###

> Write a program that outputs all possibilities to put + or - or
> nothing between the numbers 1, 2, ..., 9 (in this order) such that
> the result is always 100. For example: 1 + 2 + 34 – 5 + 67 – 8 + 9 =
> 100.
*/

use std::fmt::{Formatter, Display};

#[derive(Clone, Copy)]
enum Operation {
    Plus,
    Minus
}

#[derive(Clone)]
enum Tree {
    Op {
        op: Operation,
        lhs: Box<Tree>,
        rhs: i32,
    },
    Number {
        number: i32
    },
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Tree::Number { number } => { write!(f, "{}", number)?; },
            Tree::Op { op, lhs, rhs } => {
                write!(f, "{}", lhs)?;
                match op {
                    Operation::Plus => {
                        write!(f, " + ")?;
                    },

                    Operation::Minus => {
                        write!(f, " - ")?;
                    },
                }
                write!(f, "{}", rhs)?;
            }
        }

        Ok(())
    }
}

fn evaluate(tree: &Tree) -> i32 {
    match tree {
        Tree::Number { number } => *number,

        Tree::Op { op, lhs, rhs } => {
            let lhs_value = evaluate(&lhs);
            match op {
                Operation::Plus => {
                    lhs_value + rhs
                },

                Operation::Minus => {
                    lhs_value - rhs
                },
            }
        },
    }
}

fn main() {
    let mut trees = vec![
        Tree::Number {
            number: 1,
        },
    ];
    for digit in 2..=9 {
        let mut new_trees = vec![];

        for current_tree in trees {
            new_trees.push(Tree::Op { op: Operation::Plus, lhs: Box::new(current_tree.clone()), rhs: digit});
            new_trees.push(Tree::Op { op: Operation::Minus, lhs: Box::new(current_tree.clone()), rhs: digit});

            let mut new_tree = current_tree;
            match &mut new_tree {
                Tree::Op { rhs, ..} => {
                    *rhs *= 10;
                    *rhs += digit;
                },
                Tree::Number { number } => {
                    *number *= 10;
                    *number += digit;
                },
            }
            new_trees.push(new_tree);
        }

        trees = new_trees;
    }

    for tree in trees {
        if evaluate(&tree) == 100 {
            println!("{}  =  100", tree);
        }
    }
}
