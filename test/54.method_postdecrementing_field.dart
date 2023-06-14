
class Banana {
  int ripeness = 55;


  Banana() {}


  void unripen() {
    ripeness--;
    print(ripeness);
    assert(ripeness == 54);
  }

}


void main() {
  var banana = Banana();
  banana.unripen();
}
