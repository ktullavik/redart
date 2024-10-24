
class Animal {
    bool alive = true;
}


class Dog extends Animal {

}


void main() {
    var dog = Dog();
    var isAlive = dog.alive;
    print("Is dog alive: ${isAlive}");
}

