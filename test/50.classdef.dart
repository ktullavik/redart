
class Banana {
  String type;
  int ripeness = 0;


  Banana(String t) {
    type = t;
  }

  void ripen() {
    ripeness++;
  }

  bool isRipe() {
    return ripeness == 10;
  }
}
