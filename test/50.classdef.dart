
class Banana {
  String type;
  int ripeness = 0;


  Banana(String type) {
    this.type = type;
  }

  void ripen() {
    ripeness++;
  }

  bool isRipe() {
    return ripeness == 10;
  }
}

