

class File {
    String filename;


    File(this.filename);


    String readAsString() {
        return __IO_FILE_READ_AS_STRING(filename);
    }

}


