import "dart:math";

void main() {
    var random = Random();
    var r = random.nextDouble();
    print("Random double: ${r}");
    assert(0 <= r && r <= 1);
}
