
class Banana {
  int ripeness = 52;


  Banana() {}


  void ripen() {
    ripeness++;
    print(ripeness);
    assert(ripeness == 53);
  }

}


void main() {
  var banana = Banana();
  banana.ripen();
}
