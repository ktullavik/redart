
void main() {

  int i = 0;
  int sum = 0;
  while (i < 10) {
    sum = sum + i;
    i++;
  }

  print("sum: ${sum}");
  assert(sum == 45, "Sum thing wrong!");
}

