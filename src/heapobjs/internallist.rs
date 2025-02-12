use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::object::Object;
use crate::objsys::RefKey;


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


    pub fn get_el(&self, index: usize) -> Object {
        self.els[index].clone()
    }


    pub fn set_el(&mut self, index: usize, val: Object) {
        self.els[index] = val;
    }


    pub fn add(&mut self, el: Object) {
        self.els.push(el);
    }


    pub fn add_all(&mut self, iterable: Vec<Object>) {
        for el in iterable {
            self.els.push(el);
        }
    }


    pub fn insert(&mut self, index: usize, el: Object) {
        self.els.insert(index, el);
    }


    pub fn remove_at(&mut self, index: usize) -> Object {
        self.els.remove(index)
    }


    pub fn remove_last(&mut self) -> Object {
        if self.els.len() > 0 {
            return self.els.pop().unwrap()
        }
        Object::Null
    }

    
    pub fn remove_range(&mut self, start: usize, end: usize) {
        self.els.drain(start .. end);
    }


    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.els.shuffle(&mut rng);
    }


    pub fn foreach(&self, f: &dyn Fn(&Object)) {

        for obj in &self.els {
            f(obj);
        }
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
