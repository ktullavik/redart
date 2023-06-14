
void main() {
  int i = returner();
  print(i);
  assert(i == 22);
}


int returner() {
  return 22;
}

