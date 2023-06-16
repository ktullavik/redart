

bool broke() {
  return true;
}


void main() {

  var i = 0;
  do {
    print("It's fine!");
    i++;
  }
  while (broke() && i < 10);
}

