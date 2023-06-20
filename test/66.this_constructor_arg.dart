
class Fox {
  String say;

  Fox(this.say) {}


  void whatDoesTheFoxSay() {
    print(say);
  }

}


int main() {
  print("What the fox says:")
  var fox = Fox("ringdingding");
  fox.whatDoesTheFoxSay();
}
