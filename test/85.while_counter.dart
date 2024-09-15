

void main() {

    int i = 0;
    while (i < 10) {
        i++;
    }
    print("Should be 10: ${i}");
    assert(i == 10);
}