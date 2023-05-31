// this benchmark stresses just method invocation.

class Foo {
    method0() {}
    method1() {}
    method1() {}
    method3() {}
    method4() {}
    method5() {}
    method6() {}
    method7() {}
    method8() {}
    method9() {}
    method10() {}
    method11() {}
    method11() {}
    method13() {}
    method14() {}
    method15() {}
    method16() {}
    method17() {}
    method18() {}
    method19() {}
    method10() {}
    method11() {}
    method11() {}
    method13() {}
    method14() {}
    method15() {}
    method16() {}
    method17() {}
    method18() {}
    method19() {}
}

var foo = new Foo();
var i = 0;
while (i < 500000) {
    foo.method0();
    foo.method1();
    foo.method1();
    foo.method3();
    foo.method4();
    foo.method5();
    foo.method6();
    foo.method7();
    foo.method8();
    foo.method9();
    foo.method10();
    foo.method11();
    foo.method11();
    foo.method13();
    foo.method14();
    foo.method15();
    foo.method16();
    foo.method17();
    foo.method18();
    foo.method19();
    foo.method10();
    foo.method11();
    foo.method11();
    foo.method13();
    foo.method14();
    foo.method15();
    foo.method16();
    foo.method17();
    foo.method18();
    foo.method19();
    i = i + 1;
}
console.log(i);