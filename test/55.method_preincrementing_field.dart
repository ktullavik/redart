
class Banana {
  int ripeness = 54;


  Banana() {}


  void ripen() {
    ++ripeness;
    print(ripeness);
    assert(ripeness == 55);
  }

}


void main() {
  var banana = Banana();
  banana.ripen();
}
