
class Banana {
  int ripeness = 51;


  Banana() {}


  void printRipeness() {
    print("ripeness should be 51:");
    print(ripeness);
  }

}


void main() {
  var banana = Banana();
  banana.printRipeness();
}
