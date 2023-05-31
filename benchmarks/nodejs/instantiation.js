// this benchmark stresses instance creation and constructorializer calling.

class Foo {
    constructor() {}
}

var i = 0;
while (i < 500000) {
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    new Foo();
    i = i + 1;
}
