
class Animalia {
    bool canSwim;
    String description;

    Animalia(this.canSwim, this.description);
}


class Cnidaria extends Animalia {
    Cnidaria(canSwim) :
        super(canSwim, "sea creature");
}


void main() {
    var animal = Cnidaria(false);
    assert(animal.canSwim == false);
    assert(animal.description == "sea creature");
}
