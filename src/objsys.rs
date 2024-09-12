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


pub struct Instance {
    pub id: RefKey,
    pub classname: String,
    pub fields: HashMap<String, Object>,
    pub marked: bool
}


impl Instance {

    pub fn new(classname: String) -> Instance {
        Instance {
            id: RefKey(nuid::next()),
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


    // pub fn print_fields(&self) {
    //     println!("FIELDS FOR {} of type {}", self.id, self.classname);
    //     for (fieldname, val) in &self.fields {
    //         println!("{} = {}", fieldname, val);
    //     }
    // }

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


    pub fn has_method(&self, methname: &str) -> bool {
        self.methods.contains_key(methname)
    }


    pub fn get_method(&self, methname: &str) -> Object {

        if let Object::Function(_, _, _, _) = &self.methods[methname] {
            return self.methods[methname].clone();
        }
        panic!("No such method")
    }


    pub fn instantiate(&self) -> Box<Instance> {
        return Box::new(Instance::new(self.name.clone()));
    }
}


pub struct ObjSys {
    classmap: HashMap<String, Class>,
    instancemap: HashMap<RefKey, Box::<Instance>>,
    listmap: HashMap<RefKey, Box::<InternalList>>,
    this: RefKey,
}


impl ObjSys {

    pub fn new() -> ObjSys {
        ObjSys {
            classmap: HashMap::new(),
            instancemap: HashMap::new(),
            listmap: HashMap::new(),
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
        println!("Get class: {}", name);
        self.classmap.get(name).unwrap()
    }


    pub fn register_instance(&mut self, instance: Instance) -> Object {
        let inst = Box::new(instance);
        let rk = inst.id.clone();
        self.instancemap.insert(rk.clone(), inst);
        return Object::Reference(rk);
    }


    pub fn register_list(&mut self, list: InternalList) -> Object {
        let inst = Box::new(list);
        let rk = inst.id.clone();
        self.listmap.insert(rk.clone(), inst);
        return Object::Reference(rk);
    }


    pub fn get_instance(&self, id: &RefKey) -> &Instance {
        if self.instancemap.contains_key(id) {
            return &self.instancemap.get(id).unwrap();
        }
        panic!("Instance not found: {}", id);
    }


    pub fn get_list(&self, id: &RefKey) -> &InternalList {
        if self.listmap.contains_key(id) {
            return &self.listmap.get(id).unwrap();
        }
        panic!("InternalList not found: {}", id);
    }


    pub fn get_instance_mut(&mut self, id: &RefKey) -> &mut Instance {
        return self.instancemap.get_mut(id).unwrap();
    }


    pub fn get_list_mut(&mut self, id: &RefKey) -> &mut InternalList {
        return self.listmap.get_mut(id).unwrap();
    }


    pub fn has_instance(&self, id: &RefKey) -> bool {
        self.instancemap.contains_key(id)
    }


    pub fn has_list(&self, id: &RefKey) -> bool {
        self.instancemap.contains_key(id)
    }

    // NB, only Instances can be THIS for now.

    // This could just check if the this string is empty?
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


    pub fn mark(&mut self, rk: &RefKey) {

        let mut childs: Vec<RefKey> = Vec::new();

        if self.instancemap.contains_key(rk) {

            let p = self.instancemap.get_mut(rk).unwrap();

            if p.marked {
                return;
            }
            p.marked = true;

            for (_, obj) in p.fields.iter() {
                if let Object::Reference(refid) = obj {
                    childs.push(refid.clone());
                }
            }
        }
        else if self.listmap.contains_key(rk) {
            let p = self.listmap.get_mut(rk).unwrap();

            if p.marked {
                return;
            }
            p.marked = true;


            for obj in p.els.iter() {
                if let Object::Reference(refid) = obj {
                    childs.push(refid.clone());
                }
            }
        }
        else {
            panic!("GC could not find heap object: {}", rk)
        }

        for cid in childs {
            self.mark(&cid);
        }
    }


    pub fn sweep(&mut self) {

        // FOR EASIER DEBUG USE THIS:

        let mut del_instances: Vec<RefKey> = Vec::new();
        let mut del_lists: Vec<RefKey> = Vec::new();

        for (k, v) in self.instancemap.iter() {
            if !v.marked {
                del_instances.push(k.clone());
            }
        }
        for (k, v) in &self.listmap {
            if !v.marked {
                del_lists.push(k.clone());
            }
        }
        for k in del_instances {
            println!("Garbagecollecting: {}", k);
            self.instancemap.remove(&k);
        }
        for k in del_lists {
            self.listmap.remove(&k);
        }

        // ELSE USE THIS:

        // self.instancemap.retain(|_, v| {
            // v.marked
        // });

        // self.listmap.retain(|_, v| {
            // v.marked
        // });
    }


    pub fn clearmark(&mut self) {

        let mut clear_instances: Vec<RefKey> = Vec::new();
        let mut clear_lists: Vec<RefKey> = Vec::new();

        for k in self.instancemap.keys() {
            clear_instances.push(k.clone());
        }
        for k in self.listmap.keys() {
            clear_lists.push(k.clone());
        }

        for k in clear_instances {
            self.instancemap.get_mut(&k).unwrap().marked = false;
        }
        for k in clear_lists {
            self.listmap.get_mut(&k).unwrap().marked = false;
        }
    }

}

