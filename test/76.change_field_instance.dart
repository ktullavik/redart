
class Gnu {
    int horns = 2;
}


class Beastie {
    Gnu gnu = Gnu();

    void change() {
        print("Change");
        gnu = Gnu();
    }
}


void main() {

    Beastie beastie = Beastie();
    beastie.gnu.horns = 1;
    beastie.change();

    print("Horns: ");
    print(beastie.gnu.horns);
    assert(beastie.gnu.horns == 2);
}
