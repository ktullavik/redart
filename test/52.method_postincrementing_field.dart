
class Banana {
  int ripeness = 51;


  Banana() {}


  void ripen() {
    ripeness++;
    print(ripeness);
  }

}


void main() {
  var banana = Banana();
  banana.ripen();
}
