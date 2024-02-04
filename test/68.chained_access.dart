
class A {

    A() {

    }

    void printHello() {
        print("Hello");
    }
}


class B {
    A a;

    B() {
        a = A();
    }
}


class C {
    B b;

    C() {
        b = B();
    }
}


void main() {
    var c = C();
    c.b.a.printHello();
}
