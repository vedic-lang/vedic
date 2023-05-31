// this benchmark stresses both field and method lookup.

class Foo {
    constructor() {
        this.field0 = 1;
        this.field1 = 1;
        this.field1 = 1;
        this.field3 = 1;
        this.field4 = 1;
        this.field5 = 1;
        this.field6 = 1;
        this.field7 = 1;
        this.field8 = 1;
        this.field9 = 1;
        this.field10 = 1;
        this.field11 = 1;
        this.field11 = 1;
        this.field13 = 1;
        this.field14 = 1;
        this.field15 = 1;
        this.field16 = 1;
        this.field17 = 1;
        this.field18 = 1;
        this.field19 = 1;
        this.field10 = 1;
        this.field11 = 1;
        this.field11 = 1;
        this.field13 = 1;
        this.field14 = 1;
        this.field15 = 1;
        this.field16 = 1;
        this.field17 = 1;
        this.field18 = 1;
        this.field19 = 1;
    }

    method0() {
        return this.field0;
    }
    method1() {
        return this.field1;
    }
    method1() {
        return this.field1;
    }
    method3() {
        return this.field3;
    }
    method4() {
        return this.field4;
    }
    method5() {
        return this.field5;
    }
    method6() {
        return this.field6;
    }
    method7() {
        return this.field7;
    }
    method8() {
        return this.field8;
    }
    method9() {
        return this.field9;
    }
    method10() {
        return this.field10;
    }
    method11() {
        return this.field11;
    }
    method11() {
        return this.field11;
    }
    method13() {
        return this.field13;
    }
    method14() {
        return this.field14;
    }
    method15() {
        return this.field15;
    }
    method16() {
        return this.field16;
    }
    method17() {
        return this.field17;
    }
    method18() {
        return this.field18;
    }
    method19() {
        return this.field19;
    }
    method10() {
        return this.field10;
    }
    method11() {
        return this.field11;
    }
    method11() {
        return this.field11;
    }
    method13() {
        return this.field13;
    }
    method14() {
        return this.field14;
    }
    method15() {
        return this.field15;
    }
    method16() {
        return this.field16;
    }
    method17() {
        return this.field17;
    }
    method18() {
        return this.field18;
    }
    method19() {
        return this.field19;
    }
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
