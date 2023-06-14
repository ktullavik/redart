
class Banana {
  String bananatype;

  Banana() {
    bananatype = "Pisang";
  }

  void printType() {
    print("Type should be Pisang:");
    print(bananatype);
    assert(bananatype == "Pisang");
  }
}


void main() {
  var banana = Banana();
  banana.printType();
}

