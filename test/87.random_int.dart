import "dart:math";

void main() {
    var random = Random();
    int r = random.nextInt(100);
    print("Random int: ${r}");
    assert(0 <= r && r < 100);
}