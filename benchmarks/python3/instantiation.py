# This benchmark stresses instance creation and initializer calling.


class Foo:
    def __init__(self):
        pass


i = 0
while i < 500000:
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    Foo()
    i = i + 1
