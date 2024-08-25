
class Beastie {
    int heads = 1;

    int addHeads() {
        var heads = 5;
        return heads;
    }
}


void main() {
    var b = Beastie();
    var c = b.addHeads();
    assert(c == 5);
    assert(b.heads == 1);
}
