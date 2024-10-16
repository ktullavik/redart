
void main() {
    var li = ["Ferrari", "Jaguar"];
    li.insert(0, "Lotus");
    li.insert(3, "Volvo");
    li.insert(1, "Toyota");
    assert(li[0] == "Lotus");
    assert(li[1] == "Toyota");
    assert(li[2] == "Ferrari");
    assert(li[3] == "Jaguar");
    assert(li[4] == "Volvo");
}
