
void main() {

  var c = 0;
  for (var i=0; i<10; i++) {
    print(i);
    c = c+c;
    c++;
  }
  assert(c == 45);

}
