

class ListKeeper {
    List games = ["C&C", "Theme Hospital"];


    void changeGames() {
        games[0] = "Minecraft";
        games[1] = "Two Point Hospital";
    }
}


void main() {
    var lk = ListKeeper();
    lk.changeGames();
    print(lk.games);
    assert(lk.games[0] == "Minecraft");
    assert(lk.games[1] == "Two Point Hospital");
}

