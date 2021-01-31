/*
### Problem 2 ###

> Write a function that combines two lists by alternatingly taking
> elements. For example: given the two lists `[a, b, c]` and
> `[1, 2, 3]`, the function should return `[a, 1, b, 2, c, 3]`.
*/

// Rust vectors contain values of fixed type, so we can't solve the problem exactly. To make up for
// it, I wrote a *generic* function that will work for any element type.
//
// The requirement for Clone could be avoided if we passed Vec<T> instead of &[T]. Unfortunately,
// the problem wasn't formulated precisely enough to be language-independent, and this is not
// a language interview where I could ask for clarification. So I implement one of the possible
// signatures that such a function could have.
fn combine<T: Copy + Clone>(first: &[T], second: &[T]) -> Vec<T> {
    let mut result = vec![];
    // We only iterate until one of the arrays ends.
    let max_length = first.len().min(second.len());
    for i in 0..max_length {
        result.push(first[i].clone());
        result.push(second[i].clone());
    }
    result
}

fn main() {
    {
        let first = [1, 2, 3];
        let second = [100, 200, 300];
        let expected = [1, 100, 2, 200, 3, 300];
        assert_eq!(combine(&first, &second), expected);
    }

    {
        let first = [3.14, 15.92, 65.4];
        let second = [2.71, 82.8, 18.28];
        let expected = [3.14, 2.71, 15.92, 82.8, 65.4, 18.28];
        assert_eq!(combine(&first, &second), expected);
    }

    // The problem statement doesn't mention what should become of lists of differing length, so
    // I won't test this case :)
}
