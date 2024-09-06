use std::fmt;
use std::collections::HashMap;
use utils::dprint;
use object::Object;
use node::Node;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RefKey(String);

impl fmt::Display for RefKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RefKey<{}>", self.0)
    }
}


pub struct Instance {
    pub id: RefKey,
    pub classname: String,
    pub fields: HashMap<String, Object>,
    pub marked: bool
}


impl Instance {

    pub fn new(id: RefKey, classname: String) -> Instance {
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
        return Instance::new(RefKey(nuid::next()), self.name.clone());
    }
}


pub struct ObjSys {
    classmap: HashMap<String, Class>,
    instancemap: HashMap<RefKey, Instance>,
    this: RefKey,
}


impl ObjSys {

    pub fn new() -> ObjSys {
        ObjSys {
            classmap: HashMap::new(),
            instancemap: HashMap::new(),
            this: RefKey(String::from("")),
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


    pub fn get_instance(&self, id: &RefKey) -> &Instance {

        if self.instancemap.contains_key(id) {
            return self.instancemap.get(id).unwrap()
        }
        panic!("Instance not found: {}", id);
    }


    pub fn get_instance_mut(&mut self, id: &RefKey) -> &mut Instance {
        return self.instancemap.get_mut(id).unwrap()
    }


    pub fn has_instance(&self, id: &RefKey) -> bool {
        self.instancemap.contains_key(id)
    }


    pub fn has_this(&self) -> bool {
        self.has_instance(&self.this)
    }


    pub fn get_this_instance_mut(&mut self) -> &mut Instance {
        return self.instancemap.get_mut(&self.this).unwrap();
    }


    pub fn get_this(&self) -> RefKey {
        return self.this.clone();
    }


    pub fn set_this(&mut self, instance_id: RefKey) {
        self.this = instance_id;
    }


    pub fn mark(&mut self, refid: &RefKey) {

        let p = self.instancemap.get_mut(refid).unwrap();

        if p.marked {
            return;
        }
        p.marked = true;

        let mut childs: Vec<RefKey> = Vec::new();
        for (_, obj) in p.fields.iter() {
            if let Object::Reference(refid) = obj {
                childs.push(refid.clone());
            }
        }

        for cid in childs {
            self.mark(&cid);
        }
    }


    pub fn sweep(&mut self) {

        // FOR EASIER DEBUG USE THIS INSTEAD:

        let mut del: Vec<RefKey> = Vec::new();
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

        let mut clear: Vec<RefKey> = Vec::new();

        for k in self.instancemap.keys() {
            clear.push(k.clone());
        }

        for k in clear {
            self.instancemap.get_mut(&k).unwrap().marked = false;
        }
    }



}

