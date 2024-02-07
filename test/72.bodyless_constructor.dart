
class Turtle {
    bool isNinja = true;

    Turtle();

    void yell() {
        print("Haaaiaaa");
    }
}


class Egg {
    var content;

    Egg(this.content);
}


void main() {
    var turtle = Turtle();
    turtle.yell();
    var egg = Egg(turtle);
    assert(egg.content.isNinja);
}
