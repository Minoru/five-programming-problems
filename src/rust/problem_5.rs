/*
### Problem 5 ###

> Write a program that outputs all possibilities to put + or - or
> nothing between the numbers 1, 2, ..., 9 (in this order) such that
> the result is always 100. For example: 1 + 2 + 34 – 5 + 67 – 8 + 9 =
> 100.
*/

#[derive(Clone, Copy, PartialEq, Debug)]
enum Operator {
    Plus,
    Minus,
    Nothing
}

/// Given an array of operators, change the last one to the "next state", and if necessary, update
/// the other ones too.
fn advance(operators: &mut [Operator]) {
    let last = operators.len() - 1;
    match operators[last] {
        Operator::Plus => {
            operators[last] = Operator::Minus;
        },
        Operator::Minus => {
            operators[last] = Operator::Nothing;
        },
        Operator::Nothing => {
            operators[last] = Operator::Plus;
            if operators.len() > 1 {
                advance(&mut operators[..last]);
            }
        },
    }
}

/// Given an array of operators, calculate the sum of digits with these operators applied.
fn evaluate(operators: &[Operator]) -> i32 {
    let mut number = 0;
    for i in (0..operators.len()).rev() {
        if operators[i] == Operator::Nothing {
            let mut num = number.to_string();
            num.prepend(&i.to_string());
            number = num.parse().unwrap();
        }
    }
}

fn to_string(operators: &[Operator]) -> String {
    let mut result = String::new();
    for (n, op) in [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().zip(operators) {
        result.push_str(&n.to_string());
        match op {
            Operator::Plus => result.push('+'),
            Operator::Minus => result.push('-'),
            Operator::Nothing => {},
        }
    }
    result.push('9');
    result
}

fn main() {
    let mut state = [Operator::Plus; 8];
    loop {
        if evaluate(&state) == 100 {
            println!("{}  =  {}", to_string(&state), evaluate(&state));
        }

        advance(&mut state);

        // We reached the initial state again, stop the loop.
        if state.iter().all(|x| *x == Operator::Nothing) {
            break;
        }
    }
}
