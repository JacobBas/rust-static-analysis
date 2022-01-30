# Need to get stuff working for a few different things:
# 1. function definitions
# 2. function calls within functions
# 3. function calls wihtin classes
# 4. class definitions and methods
# 5. class calls within functions
# 6. class method calls within functions
#
# I really want to focus on the function stuff though
# since that is going to be the most important for understanding
# the network of calls within a codebase.

# we would need to look into both assignment and call
# so that we can see all of the possible places that function calls
# are actually happening. That's why it may make the most sense
# to work our way through the tree one node at a time to get a sense
# of all of the data that is actually in there.

# I'm really interested to see how long this will actually take to
# complete on larger code bases.

# defining some functions
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
