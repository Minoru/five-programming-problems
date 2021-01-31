/*
### Problem 3 ###

> Write a function that computes the list of the first 100 Fibonacci
> numbers. By definition, the first two numbers in the Fibonacci
> sequence are 0 and 1, and each subsequent number is the sum of the
> previous two. As an example, here are the first 10 Fibonnaci
> numbers: 0, 1, 1, 2, 3, 5, 8, 13, 21, and 34.
*/

fn first_100_fibonacci_numbers() -> Vec<u128> {
    let mut result = vec![0, 1];
    for i in 2..100 {
        result.push(result[i-1] + result[i-2]);
    }
    result
}

fn main() {
    // I can't remember all the first one hundred Fibonacci numbers *just now*, and I don't have
    // time to Google them, so let's check just the ones that are in the spec.
    let result = first_100_fibonacci_numbers();
    assert_eq!(result.len(), 100);
    assert_eq!(&result[..10], [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
}
