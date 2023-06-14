
void main() {
  print("Should be false:");
  print(!true);
  assert(true);
  print("Should be true:");
  print(!false);
  assert(!false);
  print("Should be false:");
  print(!(true && true));
  assert(true && true);
  print("Should be false:");
  print(!true || !true);
  assert(!(!true || !true));
  print("Should be true:");
  print(!false || (true && !true));
  assert(!false || (true && !true));
}
