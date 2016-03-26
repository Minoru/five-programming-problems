def problem_3():
    result = [0, 1]

    while len(result) < 100:
        result.append(result[-1] + result[-2])

    return result
