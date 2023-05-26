use std::collections::HashMap;
use evaluator::*;
use utils::dprint;


// #[derive(Clone)]
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


    // pub fn set_field(&mut self, name: String, value: Object) {
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


pub struct InstanceMap {
    pub instance: HashMap<String, Instance>,
    pub this: String
}


impl InstanceMap {

    pub fn new() -> InstanceMap {

        InstanceMap {
            instance: HashMap::new(),
            this: String::from("")
        }
    }


    pub fn add(&mut self, id: String, inst: Instance) {
        self.instance.insert(id, inst);
    }


    pub fn get(&mut self, id: &str) -> &Instance {
        self.instance.get(id).unwrap()
    }


    pub fn has_this(&self) -> bool {
        self.instance.contains_key(self.this.as_str())
    }


    pub fn get_this(&mut self) -> &mut Instance {
        println!("Get this: {}", self.this);

        if self.instance.contains_key(self.this.as_str()) {
            let thisinst = self.instance.get_mut(self.this.as_str()).unwrap();
            return thisinst;
        }

        println!("Registered instances: ");
        for (k, _) in &self.instance {
            println!("    {}", k);
        }
        panic!("Could not get this instance: {}", self.this);

    }

}


pub struct ClassMap {
    pub class: HashMap<String, Class>
}


impl ClassMap {

    pub fn new() -> ClassMap {

        ClassMap {
            class: HashMap::new()
        }
    }


    pub fn get(&mut self, name: &str) -> &Class {
        println!("getting class: {}", name);
        self.class.get(name).unwrap()
    }


    pub fn add(&mut self, c: Class) {
        self.class.insert(c.name.clone(), c);
    }

}


pub struct Class {
    pub name: String,
    pub classid: String,
    pub constructors: HashMap<String, Object>,
    pub fields: Vec<(String, String, Object)>,
    pub methods: HashMap<String, Object>
}


impl Class {

    pub fn new(name: String) -> Class {

        Class {
            name,
            classid: nuid::next(),
            constructors: HashMap::new(),
            fields: Vec::new(),
            methods: HashMap::new()
        }
    }


    pub fn add_constructor(&mut self, name: String, m: Object) {
        self.constructors.insert(name.clone(), m);
        dprint(format!("Inserted to constructortable: {}", name));
    }


    pub fn add_field(&mut self, ftype: String, fname: String, initval: Object) {
        self.fields.push((ftype.clone(), fname.clone(), initval));
        dprint(format!("Inserted to fieldtable: {}", fname));
    }


    pub fn add_method(&mut self, name: String, m: Object) {
        self.methods.insert(name.clone(), m);
        dprint(format!("Inserted to methodtable: {}", name));
    }


    pub fn get_method(&self, methname: &str) -> Object {

        if let Object::Function(_, _, _) = &self.methods[methname] {
            return self.methods[methname].clone();
        }
        panic!("No such method")
    }


    pub fn instantiate(&self, instmap: &mut InstanceMap) -> Object {
        let id= nuid::next();

        let mut instance = Instance::new(id.clone(), self.name.clone());


        for (ftype, fname, initval) in &self.fields {
            instance.set_field(fname.clone(), initval.clone())
        }

        instmap.add(id.clone(), instance);

        Object::Reference(id)
    }


}