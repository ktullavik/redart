
void main() {

  var i = 0;
  var c = 0;
  for (i=0; i<10; i++) {
    print(i);
    c = c + i;
  }
  assert(c == 45);

}
