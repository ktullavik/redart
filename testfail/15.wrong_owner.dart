

class NumKeeper {
    int number = 7;


    void changeNumber() {
        nk.number = 13;
    }
}


void main() {
    var nk = NumKeeper();
    nk.changeNumber();
    print(nk.number);
}



