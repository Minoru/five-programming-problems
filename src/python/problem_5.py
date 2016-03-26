def problem_5():
    options = ["1"]
    for number in range(2, 10):
        options = [ option + operator + str(number)
                for operator in [" + ", " - ", ""]
                for option in options]
    return list(filter(lambda s: eval(s) == 100, options))
