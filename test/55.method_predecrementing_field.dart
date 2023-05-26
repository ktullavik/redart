
class Banana {
  int ripeness = 56;


  Banana() {}


  void unripen() {
    --ripeness;
    print(ripeness);
  }

}


void main() {
  var banana = Banana();
  banana.unripen();
}
