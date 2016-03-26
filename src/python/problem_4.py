def problem_4(numbers):
    strings = map(str, numbers)
    return int(''.join(sorted(strings, reverse=True)))
