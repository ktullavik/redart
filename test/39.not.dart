
void main() {
  print("Should be false:");
  print(!true);
  print("Should be true:");
  print(!false);
  print("Should be false:");
  print(!(true && true));
  print("Should be false:");
  print(!true || !true);
  print("Should be true:");
  print(!false || (true && !true));
}
