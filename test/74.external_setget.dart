
class Gnu {
    bool tamed = false;
}

void main() {
    var gnu = Gnu();
    gnu.tamed = true;
    print("tamed:");
    print(gnu.tamed);
    assert(gnu.tamed);
}
