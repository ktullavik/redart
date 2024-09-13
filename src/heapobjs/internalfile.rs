use std::fs::File;
use crate::objsys::RefKey;


pub struct InternalFile {
    pub id: RefKey,
    pub file: File,
    pub marked: bool
}


impl InternalFile {

    pub fn new(filename: String) -> InternalFile {
        
        InternalFile {
            id: RefKey(nuid::next()),
            file: File::open(filename).unwrap(),
            marked: false
        }
    }
}
