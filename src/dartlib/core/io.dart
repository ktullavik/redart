

class File {
    String filename;


    File(this.filename) {

    }


    String readAsString() {
        return __IO_READ_AS_STRING(filename);
    }


    List readAsLines() {
        return __IO_READ_AS_LINES(filename);
    }

}


