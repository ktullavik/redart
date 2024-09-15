
void main() {

    var i = 0;
    for (var x in ["dog", "cat", "fish"]) {
        print(x);
        i = i + 1;
    }
    print("Should be 3: ${i}");
}
