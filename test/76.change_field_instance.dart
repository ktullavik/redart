class Gnu {
    bool horns = 2;
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

    assert(beastie.gnu.horns == 2);

    beastie.gnu.horns = 1;
    assert(beastie.gnu.horns == 1);

    beastie.change();

    print("Horns: ");
    print(beastie.gnu.horns);
    assert(beastie.gnu.horns == 2);

    
}
