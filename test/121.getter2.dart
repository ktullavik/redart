
class Drug {
    String effect1 = "dizzy";


    String get effect {
        return effect1;
    }


    void take() {
        print(effect);
        assert(effect == "dizzy");
    }

}


void main() {
    var d = Drug();
    d.take();
}

