

void main() {
  // Should print:
  // abc: ab: bc: c

  var a = "a";
  var b = "b";
  var c = "c";
  print("abc: ${a + "b: ${b + "c: ${c}"}"}");
}
