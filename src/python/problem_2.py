def problem_2(first, second):
    result = []

    i = j = 0
    while i < len(first) and j < len(second):
        result += first[i]
        i += 1

        result += second[j]
        j += 1

    result += first[i:]
    result += second[j:]

    return result
