use std::collections::HashMap;

use crate::{object::Object, objsys::RefKey};


pub enum MaybeObject {
    Some(Object),
    None
}


pub struct Instance {
    pub id: RefKey,
    pub classname: String,
    pub parent: MaybeObject,
    pub fields: HashMap<String, Object>,
    pub marked: bool
}


impl Instance {

    pub fn new(classname: String, parent: MaybeObject) -> Instance {
        Instance {
            id: RefKey(nuid::next()),
            classname,
            parent,
            fields: HashMap::new(),
            marked: false
        }
    }


    pub fn set_field(&mut self, name: String, value: Object) {
        self.fields.insert(name, value);
    }

    pub fn get_field(&self, name: String) -> &Object {
        self.fields.get(name.as_str()).unwrap()
    }

    pub fn has_field(&self, name: String) -> bool {
        self.fields.contains_key(name.as_str())
    }


    // pub fn print_fields(&self) {
    //     println!("FIELDS FOR {} of type {}", self.id, self.classname);
    //     for (fieldname, val) in &self.fields {
    //         println!("{} = {}", fieldname, val);
    //     }
    // }

}
