
// dart run prints:
//
// lib/main.dart:9:9: Error: Undefined name 'a'.
// print(a);
//       ^


void main() {
  int a = 1;
  foo();
}

void foo() {
  print("I should crash now!");
  print(a);
}
