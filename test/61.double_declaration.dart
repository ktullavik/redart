
// Dart error:
//
// lib/main.dart:4:7:
// Error: 'i' is already declared in this scope.
// var i = 3;
// ^
// lib/main.dart:3:7:
// Info: Previous declaration of 'i'.
// var i = 2;
//     ^
// Error: Compilation failed.


void main() {
  var i = 2;
  var i = 3;
  print(i);
}
