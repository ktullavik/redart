use std::collections::HashMap;
use utils::dprint;
use object::Object;
use node::Node;


pub struct Instance {
    pub id: String,
    pub classname: String,
    pub fields: HashMap<String, Object>,
    pub marked: bool
}


impl Instance {

    pub fn new(id: String, classname: String) -> Instance {
        Instance {
            id,
            classname,
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
}


pub struct Class {
    pub name: String,
    pub classid: String,
    pub fields: Vec<(String, String, Node)>,
    pub methods: HashMap<String, Object>,
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
    this: String,
}


impl ObjSys {

    pub fn new() -> ObjSys {
        ObjSys {
            classmap: HashMap::new(),
            instancemap: HashMap::new(),
            this: String::from(""),
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


    pub fn get_instance_mut(&mut self, id: &str) -> &mut Instance {
        return self.instancemap.get_mut(id).unwrap()
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


    pub fn mark(&mut self, refid: &str) {

        let p = self.instancemap.get_mut(refid).unwrap();

        if p.marked {
            return;
        }
        p.marked = true;

        let mut childs: Vec<String> = Vec::new();
        for (_, obj) in p.fields.iter() {
            if let Object::Reference(refid) = obj {
                childs.push(refid.clone());
            }
        }

        for cid in childs {
            self.mark(cid.as_str());
        }
    }


    pub fn sweep(&mut self) {

        // FOR EASIER DEBUG USE THIS INSTEAD:

        let mut del: Vec<String> = Vec::new();
        for (k, v) in self.instancemap.iter() {
            if !v.marked {
                del.push(k.clone());
            }
        }
        for k in del {
            println!("Garbagecollecting: {}", k);
            self.instancemap.remove(&k);
        }

        // self.instancemap.retain(|_, v| {
            // v.marked
        // });
    }


    pub fn clearmark(&mut self) {

        let mut clear: Vec<String> = Vec::new();

        for k in self.instancemap.keys() {
            clear.push(k.clone());
        }

        for k in clear {
            self.instancemap.get_mut(&k).unwrap().marked = false;
        }
    }



}

