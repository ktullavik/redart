

class Wasp {


    int get stung {
        return 1;
    }
}


void main() {
    var w = Wasp();
    print(w.stung);
    assert(w.stung == 1);
}

