use crate::{object::Object, objsys::RefKey};


pub struct InternalList {
    pub id: RefKey,
    pub els: Vec<Object>,
    pub marked: bool
}

impl InternalList {
    
    pub fn new() -> InternalList {
        InternalList {
            id: RefKey(nuid::next()),
            els: Vec::new(),
            marked: false
        }
    }

    pub fn set_elements(&mut self, new_els: Vec<Object>) {
        self.els = new_els;
    }


    pub fn add(&mut self, el: Object) {
        self.els.push(el);
    }


    pub fn remove_last(&mut self) -> Object {
        if self.els.len() > 0 {
            return self.els.pop().unwrap()
        }
        Object::Null
    }


    pub fn to_string(&self) -> String {
        let mut s = String::from("[");

        if self.els.len() == 0 {
            return String::from("[]");
        }

        let mut i = 0;
        while i < self.els.len() - 1 {
            s.push_str(format!("{}", self.els[i]).as_str());
            s.push_str(", ");
            i += 1;
        }
        s.push_str(format!("{}", self.els[i]).as_str());
        s.push_str("]");
        return s;
    }
}
