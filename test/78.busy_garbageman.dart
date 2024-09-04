
class Thrash {

    Thrash() {

    }
}


void f() {
    var t1 = Thrash();
    var t2 = Thrash();
    var t3 = Thrash();
}


void main() {
    f();
    print("All that thrash!");
}
