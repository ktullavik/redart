
class Banana {
  int ripeness = 57;


  Banana() {}


  void unripen() {
    --ripeness;
    print(ripeness);
    assert(ripeness == 56);
  }

}


void main() {
  var banana = Banana();
  banana.unripen();
}
