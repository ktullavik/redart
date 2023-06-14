

void main() {

  var a = 9;

  if (true) {
    var a = 4;
    print(a);
    assert(a == 4);
  }

  print(a);
  assert(a == 9);
}
