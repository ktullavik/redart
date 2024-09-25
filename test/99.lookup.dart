
class Bar {
    String hello = "hello";
}

class Foo {
    List li = [Bar()];
}


void main() {
    var foo = Foo();
    var b = foo.li[0];
    var s = b.hello;
    print(s);
    assert(s == "hello");
}

