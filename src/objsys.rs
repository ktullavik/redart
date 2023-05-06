use std::collections::HashMap;
use evaluator::Object;


pub struct Instance {
    pub fields: HashMap<String, Object>

}


impl Instance {

    pub fn new() -> Instance {
        Instance {
            fields: HashMap::new()
        }
    }


    pub fn set_field(&mut self, name: String, value: Object) {

    }

    pub fn get_field(&self, name: String) -> Object {
        return Object::Null;
    }

}


pub struct InstanceList {
    pub instance: HashMap<String, Instance>
}


impl InstanceList {

    pub fn new() -> InstanceList {

        InstanceList {
            instance: HashMap::new()
        }
    }
}


pub struct ClassList {
    pub class: HashMap<String, Class>
}


impl ClassList {

    pub fn new() -> ClassList {

        ClassList {
            class: HashMap::new()
        }
    }
}


pub struct Class {
    pub name: String,
    pub fields: Vec<(String, String)>,
    pub methods: HashMap<String, Object>
}


impl Class {

    pub fn new(name: String) -> Class {

        Class {
            name,
            fields: Vec::new(),
            methods: HashMap::new()
        }
    }


    pub fn add_field(&mut self, fieldtype: String, name: String) {
        self.fields.push((fieldtype, name));
    }


    pub fn add_method(&mut self, name: String, m: Object) {
        self.methods.insert(name, m);
    }


    pub fn exec_method(&self, name: &str, args: Vec<Object>) {
        if let Object::Function(name, node, params) = &self.methods[name] {

        }
    }


}