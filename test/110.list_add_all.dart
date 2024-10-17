
void main() {
    var li1 = [1,2,3];
    var li2 = [4,5,6];
    li1.addAll(li2);
    assert("${li1}" == "[1, 2, 3, 4, 5, 6]");
}

