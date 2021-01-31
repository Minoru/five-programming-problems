/*
### Problem 1 ###

> Write three functions that compute the sum of the numbers in a given
> list using a for-loop, a while-loop, and recursion.
*/

fn sum_using_for_loop(numbers: &[i64]) -> i64 {
    numbers.iter().sum()
}

fn sum_using_while_loop(numbers: &[i64]) -> i64 {
    // Rust doesn't have a while loop, so I'll emulate it.

    let mut index = 0;
    let mut sum = 0;
    loop {
        if index >= numbers.len() {
            break;
        }

        sum += numbers[index];
        index += 1;
    }
    sum
}

fn sum_using_recursion(numbers: &[i64]) -> i64 {
    if numbers.is_empty() {
        0
    } else {
        numbers[0] + sum_using_recursion(&numbers[1..])
    }
}

fn main() {
    {
        let input = [1, -2, 3];
        let expected = 2;
        assert_eq!(sum_using_for_loop(&input), expected);
        assert_eq!(sum_using_while_loop(&input), expected);
        assert_eq!(sum_using_recursion(&input), expected);
    }

    {
        let input = [3, 14, -15, 92, -65, 4];
        let expected = 33;
        assert_eq!(sum_using_for_loop(&input), expected);
        assert_eq!(sum_using_while_loop(&input), expected);
        assert_eq!(sum_using_recursion(&input), expected);
    }
}
