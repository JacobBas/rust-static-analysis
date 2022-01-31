def addition(a, b):
    """adds two numbers together"""
    return a + b


def function_print():
    """just a function that is run randomly"""
    print("inside a new function")


def func1():
    """function definition 1"""
    function_print()
    a = 20
    b = 25
    x = addition(a, b)
    return x


def func2():
    """function definition 2"""
    function_print()
    return func1()


def func3():
    """function definition 3"""
    function_print()
    return func2()


def func4():
    """function definition 4"""
    function_print()
    return func3()


if __name__ == "__main__":
    print(func4())
