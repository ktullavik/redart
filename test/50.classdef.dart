
class Banana {
  String bananatype;
  int ripeness = 0;


  Banana(t) {
    bananatype = t;
  }

  void ripen() {
    ripeness++;
    print("ripeness should be 1:");
    print(ripeness);
    print("type should be Barangan:");
    print(bananatype);
  }

  bool isRipe() {
    return ripeness == 10;
  }
}


void main() {
  var banana = Banana("Barangan");
  banana.ripen();
}

