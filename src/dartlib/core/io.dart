

class File {
    __InternalFile _internalFile;
    String filename;


    File(this.filename) {
        // This sets _internalFile
        __IO_FILE_CREATE(this, filename);
    }


    String readAsString() {
        return __IO_FILE_READ_AS_STRING(_internalFile);
    }

}


