
void main() {
  rec(5);
}

void rec(n) {
  print(n);
  if (n > 0) {
    rec(n-1);
  }
}
