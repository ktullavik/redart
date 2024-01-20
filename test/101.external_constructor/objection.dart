import "speaker.dart";


class Objection {
    Speaker speaker;


    Objection() {
        speaker = Speaker();
    }


    void object() {
        print("Objecting");
        speaker.speak();
    }
}

