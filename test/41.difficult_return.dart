
void main() {
  print(bastard());
}

void bastard() {
  if (1 < 0) {
    return false;
  }
  else if (0 > -1) {
    return true;
  }
  else if (!false) {
    return false;
  }
  else {
    return false;
  }
}
