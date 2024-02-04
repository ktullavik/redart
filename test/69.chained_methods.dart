
class C {

    C() {}


    void c() {
        print("I should print twice.");
    }
}



class B {
    var c = C();
    B() {}


    void b() {
        return c;
    }
}



class A {
    var b = B();
    A() {}


    void a() {
        return b;
    }
}


void main() {
    var ob = A();
    ob.a().b().c();
    A().a().b().c();
}
