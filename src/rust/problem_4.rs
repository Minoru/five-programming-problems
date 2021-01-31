/*
### Problem 4 ###

> Write a function that given a list of non negative integers,
> arranges them such that they form the largest possible number. For
> example, given [50, 2, 1, 9], the largest formed number is 95021.
*/

fn rearrange(numbers: &[u64]) -> Vec<u64> {
    let mut result = numbers.to_owned();
    result.sort_by_key(|x| {
        let as_string = x.to_string();
        let first_character = &as_string[..1];
        first_character
            .parse::<u8>()
            .expect("First character of a number is not a number")
    });
    result.reverse();
    result
}

fn main() {
    assert_eq!(rearrange(&[50, 2, 1, 9]), [9, 50, 2, 1]);
    assert_eq!(rearrange(&[1, 2, 3, 4]), [4, 3, 2, 1]);
}
