import 'dart:io';


void main() {
    var f = File("/etc/rc.conf");
    var s = f.readAsString();
    print(s);
}

