
class Animal {
    bool alive = false;
}


class Dog extends Animal {

}


void main() {
    var dog = Dog();
    dog.alive = true;
    var isAlive = dog.alive;
    print("Is dog alive: ${isAlive}");
}



