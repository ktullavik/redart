
class Bird {
    void sound() {
        print("chirp");
    }
}


class Crow extends Bird {}


void main() {
    var c = Crow();
    c.sound();
}
