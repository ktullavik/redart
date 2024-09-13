use std::fmt;
use std::collections::HashMap;
use utils::dprint;
use object::Object;
use node::Node;
use crate::heapobjs::{
    Instance,
    InternalFile,
    InternalList
};


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RefKey(pub String);

impl fmt::Display for RefKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RefKey<{}>", self.0)
    }
}



pub struct Class {
    pub name: String,
    pub fields: Vec<(String, String, Node)>,
    pub methods: HashMap<String, Object>,
}


impl Class {

    pub fn new(name: String) -> Class {

        Class {
            name,
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
    filemap: HashMap<RefKey, Box::<InternalFile>>,
    this: RefKey,
}


impl ObjSys {

    pub fn new() -> ObjSys {
        ObjSys {
            classmap: HashMap::new(),
            instancemap: HashMap::new(),
            listmap: HashMap::new(),
            filemap: HashMap::new(),
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


    pub fn register_file(&mut self, file: InternalFile) -> Object {
        let boxed = Box::new(file);
        let rk = boxed.id.clone();
        self.filemap.insert(rk.clone(), boxed);
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


    pub fn get_file(&self, id: &RefKey) -> &InternalFile {
        if self.filemap.contains_key(id) {
            return &self.filemap.get(id).unwrap();
        }
        panic!("InternalFile not found: {}", id);
    }


    pub fn get_instance_mut(&mut self, id: &RefKey) -> &mut Instance {
        return self.instancemap.get_mut(id).unwrap();
    }


    pub fn get_list_mut(&mut self, id: &RefKey) -> &mut InternalList {
        return self.listmap.get_mut(id).unwrap();
    }


    pub fn get_file_mut(&mut self, id: &RefKey) -> &mut InternalFile {
        return self.filemap.get_mut(id).unwrap();
    }


    pub fn has_instance(&self, id: &RefKey) -> bool {
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


}


pub mod trashman {
    use crate::object::Object;
    use super::ObjSys;
    use super::RefKey;

    
    pub fn mark(obs: &mut ObjSys, rk: &RefKey) {

        let mut childs: Vec<RefKey> = Vec::new();

        if obs.instancemap.contains_key(rk) {

            let p = obs.instancemap.get_mut(rk).unwrap();

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
        else if obs.listmap.contains_key(rk) {
            let p = obs.listmap.get_mut(rk).unwrap();

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
        else if obs.filemap.contains_key(rk) {
            let p = obs.filemap.get_mut(rk).unwrap();
            p.marked = true;
        }
        else {
            panic!("GC could not find heap object: {}", rk)
        }

        for cid in childs {
            mark(obs, &cid);
        }
    }


    pub fn sweep(obs: &mut ObjSys) {

        // FOR EASIER DEBUG USE THIS:

        let mut del_instances: Vec<RefKey> = Vec::new();
        let mut del_lists: Vec<RefKey> = Vec::new();
        let mut del_files: Vec<RefKey> = Vec::new();

        for (k, v) in obs.instancemap.iter() {
            if !v.marked {
                del_instances.push(k.clone());
            }
        }
        for (k, v) in &obs.listmap {
            if !v.marked {
                del_lists.push(k.clone());
            }
        }
        for (k, v) in &obs.filemap {
            if !v.marked {
                del_files.push(k.clone());
            }
        }

        for k in del_instances {
            println!("GC instance: {}", k);
            obs.instancemap.remove(&k);
        }
        for k in del_lists {
            println!("GC list: {}", k);
            obs.listmap.remove(&k);
        }
        for k in del_files {
            println!("GC file: {}", k);
            obs.filemap.remove(&k);
        }


        // ELSE USE THIS:

        // self.instancemap.retain(|_, v| {
            // v.marked
        // });

        // self.listmap.retain(|_, v| {
            // v.marked
        // });
    }


    pub fn clearmark(obs: &mut ObjSys) {

        let mut clear_instances: Vec<RefKey> = Vec::new();
        let mut clear_lists: Vec<RefKey> = Vec::new();
        let mut clear_files: Vec<RefKey> = Vec::new();

        for k in obs.instancemap.keys() {
            clear_instances.push(k.clone());
        }
        for k in obs.listmap.keys() {
            clear_lists.push(k.clone());
        }
        for k in obs.filemap.keys() {
            clear_files.push(k.clone());
        }

        for k in clear_instances {
            obs.instancemap.get_mut(&k).unwrap().marked = false;
        }
        for k in clear_lists {
            obs.listmap.get_mut(&k).unwrap().marked = false;
        }
        for k in clear_files {
            obs.filemap.get_mut(&k).unwrap().marked = false;
        }
    } 
}
