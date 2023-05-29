
// Dart error:
//
// lib/main.dart:3:3:
// Error: Setter not found: 'a'.
// a = 8;
// ^
// lib/main.dart:4:9:
// Error: Undefined name 'a'.
// print(a);
//       ^
// Error: Compilation failed.

void main {
  a = 1;
  print(a);
}

