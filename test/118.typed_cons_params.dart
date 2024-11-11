
class Dog {

    Dog(bool friendly, String color) {
        if (friendly) {
            print("Dog is friendly and ${color}");
        }
    }
}


void main() {
    Dog d = Dog(true, "black");
}

