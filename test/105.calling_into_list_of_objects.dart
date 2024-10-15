
class Duck {
    void tweet() {
        print("Quack");
    }
}

class Goose {
    void tweet() {
        print("Geeeek");
    }
}

void main() {
    List li = [Duck(), Goose()];
    li[0].tweet();
    li[1].tweet();
}
