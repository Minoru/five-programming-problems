def problem_1_for_loop(numbers):
    result = 0

    for n in numbers:
        result += n

    return result

def problem_1_while_loop(numbers):
    result = i = 0

    while i < len(numbers):
        result += numbers[i]
        i += 1

    return result

def problem_1_recursion(numbers):
    # Don't run this on large lists: Python doesn't have tail recursion
    # optimization!

    def helper(accumulator):
        try:
            return helper(accumulator + numbers.pop())
        except IndexError:
            return accumulator

    return helper(0)
