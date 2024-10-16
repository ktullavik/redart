

void main() {

    var li1 = [1,2,3,4,5,6,7,8,9];
    var li2 = [1,2,3,4,5,6,7,8,9];
    var li3 = [1,2,3,4,5,6,7,8,9];
    var li4 = [1,2,3,4,5,6,7,8,9];
    
    li1.removeRange(0, 9);
    assert(li1.toString() == "[]");

    li2.removeRange(8, 9);
    assert(li2.toString() == "[1, 2, 3, 4, 5, 6, 7, 8]");

    li3.removeRange(0, 0);
    assert(li3.toString() == "[1, 2, 3, 4, 5, 6, 7, 8, 9]");

    li4.removeRange(0, 3);
    assert(li4.toString() == "[4, 5, 6, 7, 8, 9]");
}
