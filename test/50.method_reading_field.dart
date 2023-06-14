
class Banana {
  int ripeness = 50;


  Banana() {}


  void printRipeness() {
    print("Ripeness should be 50:");
    print(ripeness);
    assert(50 == ripeness);
  }

}


void main() {
  var banana = Banana();
  banana.printRipeness();
}
