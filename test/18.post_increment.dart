//
// Testing with dartpad
// --------------------
//
// Inc/dec can only be applied to names, not directly to numbers,
// and not to parenthesized expressions:
//   i++ works.
//   2++ does not work.
//   (i)++ does not work.
//
// Inc/dec works as a prefix and postfix;
//   print(i++) prints the value before it is incremented.
//   print(++i) prints the value after it is incremented.
//
// Inc/dec is a statement in the sense that it is a shorthand
// assignment, but it's an expression with regards to where
// it can be used and since it evaluates to a value.
//

void main() {
  int i = 17;
  i++;
  print(i);
  assert(i == 18);
}

