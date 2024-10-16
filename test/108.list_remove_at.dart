
void main() {
    var li = [1,2,3,4];
    li.removeAt(0);
    assert("${li}" == "[2, 3, 4]");
    li.removeAt(2);
    assert("${li}" == "[2, 3]");
}
