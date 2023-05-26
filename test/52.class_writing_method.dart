
class Banana {
  int ripeness = 51;


  Banana() {}


  void ripen() {
    ripeness++;
    print("ripeness should be 52:");
    print(ripeness);
  }

}


void main() {
  var banana = Banana();
  banana.ripen();
}
