// Dart error:
// bin/main.dart:7:16: Error: Expected an identifier, but got ','.


class Rabbit {

  Rabbit(int p,, int q) {
    print(p + q);
  }
}


void main() {
  Rabbit(2,3);
}
