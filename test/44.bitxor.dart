
void main() {
  //    110010  //  50
  //  ^ 011110  //  30
  //  = 101100  //  44
  int a = 50;
  int b = 30;
  int c = a ^ b;
  print(c);
  assert(c==44);
}

