
class ListKeeper {
    List games = ["C&C", "Theme Hospital"];


    void changeGames() {
        lk.games[0] = "Minecraft";
        lk.games[1] = "Two Point Hospital";
    }
}


void main() {
    var lk = ListKeeper();
    lk.changeGames();
    print(lk.games);
}
