use std::collections::HashMap;
use utils::dprint;
use object::Object;
use node::Node;


pub struct Instance {
    pub id: String,
    pub classname: String,
    pub fields: HashMap<String, Object>
}


impl Instance {

    pub fn new(id: String, classname: String) -> Instance {
        Instance {
            id,
            classname,
            fields: HashMap::new()
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
}


pub struct Class {
    pub name: String,
    pub classid: String,
    pub fields: Vec<(String, String, Node)>,
    pub methods: HashMap<String, Object>
}


impl Class {

    pub fn new(name: String) -> Class {

        Class {
            name,
            classid: nuid::next(),
            fields: Vec::new(),
            methods: HashMap::new()
        }
    }


    pub fn add_field(&mut self, ftype: String, fname: String, initexpr: Node) {
        self.fields.push((ftype.clone(), fname.clone(), initexpr));
        dprint(format!("Inserted to fieldtable: {}", fname));
    }


    pub fn add_method(&mut self, name: String, m: Object) {
        self.methods.insert(name.clone(), m);
        dprint(format!("Inserted to methodtable: {}", name));
    }


    pub fn get_method(&self, methname: &str) -> Object {

        if let Object::Function(_, _, _, _) = &self.methods[methname] {
            return self.methods[methname].clone();
        }
        panic!("No such method")
    }


    pub fn instantiate(&self) -> Instance {
        return Instance::new(nuid::next(), self.name.clone());
    }
}


pub struct ObjSys {
    classmap: HashMap<String, Class>,
    instancemap: HashMap<String, Instance>,
    this: String
}


impl ObjSys {

    pub fn new() -> ObjSys {
        ObjSys {
            classmap: HashMap::new(),
            instancemap: HashMap::new(),
            this: String::from("")
        }
    }


    pub fn new_class(&self, name: String) -> Class {
        return Class::new(name);
    }


    pub fn register_class(&mut self, class: Class) {
        self.classmap.insert(class.name.clone(), class);
    }


    pub fn get_class(&self, name: &str) -> &Class {
        self.classmap.get(name).unwrap()
    }


    pub fn register_instance(&mut self, instance: Instance) -> Object {
        let id = instance.id.clone();
        self.instancemap.insert(id.clone(), instance);
        return Object::Reference(id);
    }


    pub fn get_instance(&self, id: &str) -> &Instance {

        if self.instancemap.contains_key(id) {
            return self.instancemap.get(id).unwrap()
        }

        dprint("Registered instances: ");
        for (k, _) in &self.instancemap {
            dprint(format!("    {}", k));
        }
        panic!("Could not get this instance: {}", id);
    }


    pub fn has_instance(&self, id: &str) -> bool {
        self.instancemap.contains_key(id)
    }


    pub fn has_this(&self) -> bool {
        self.has_instance(self.this.as_str())
    }


    pub fn get_this_instance_mut(&mut self) -> &mut Instance {
        return self.instancemap.get_mut(self.this.as_str()).unwrap();
    }


    pub fn get_this(&self) -> String {
        return self.this.clone();
    }


    pub fn set_this(&mut self, instance_id: String) {
        self.this = instance_id;
    }


    // pub fn clear_this(&mut self) {
    //     println!("REAL CLEAR");
    //     self.this = String::from("");
    // }

}
