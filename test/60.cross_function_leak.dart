

void main() {
  int a = 1;
  foo();
}

void foo() {
  print("I should crash now!");
  print(a);
}
