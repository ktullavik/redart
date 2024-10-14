
class Gnu {
    bool tamed = false;
}


class Beastie {
    Gnu gnu = Gnu();

    void tame() {
        gnu.tamed = true;
    }
}


void main() {
    Beastie beastie = Beastie();
    beastie.tame();
    assert(beastie.gnu.tamed);
}
