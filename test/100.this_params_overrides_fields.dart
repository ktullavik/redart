class Foo {
  int i = 13;
  
  Foo(this.i);
}


void main() {
  var f = Foo(7);
  print(f.i);
  assert(f.i == 7);
}
