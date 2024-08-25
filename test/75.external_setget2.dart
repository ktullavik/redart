
class Gnu {
    bool tamed = false;
    int horns = 2;
}


class Beastie {
    Gnu gnu = Gnu();

    void tame() {
        gnu.tamed = true;
    }

    void change() {
        print("Change");
        gnu = Gnu();
    }
}


void main() {
    Beastie beastie = Beastie();

    beastie.tame();
    beastie.gnu.horns = 1;

    print("Tamed: ");
    print(beastie.gnu.tamed);
    print("Horns: ");
    print(beastie.gnu.horns);
    
    assert(beastie.gnu.tamed);
    assert(beastie.gnu.horns == 1);    
}
