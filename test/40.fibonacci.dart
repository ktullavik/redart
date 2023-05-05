
void main() {
  print("Should be 55:")
  print(fibo(10));
}

void fibo(n) {
  if (n > 1) {
    return fibo(n - 1) + fibo(n - 2);
  }
  return n;
}
