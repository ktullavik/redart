use std::io::Read;
use crate::{heapobjs::InternalFile, node::Node, object::Object, state::State};
use crate::error::{check_argc, err_arg_type};


pub fn file_construct(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    // First arg is internal and hidden from user.
    check_argc("File", 1, args.len() - 1, fnode, state);

    if let Object::Reference(rk) = &args[0] {

        if let Object::String(s) = &args[1] {
            let ifile = InternalFile::new(s.to_string());
            let internal_rk = state.objsys.register_file(ifile);

            let dfile = state.objsys.get_instance_mut(rk);
            dfile.set_field(String::from("_internalFile"), internal_rk);
            return Object::Reference(rk.clone());
        }
        err_arg_type(
            "File",
            "String",
            &args[1],
            &argnodes[1],
            state
        )
    }
    panic!("Unexpected internal arg: {}", &args[0])
}


pub fn file_read_as_string(
    fnode: &Node,
    args: Vec<Object>,
    state: &mut State) -> Object {

    // First arg is internal and hidden from user.
    check_argc("File.readAsString", 0, args.len() - 1, fnode, state);

    if let Object::Reference(rk) = &args[0] {
        let ifile = state.objsys.get_file_mut(rk);
        let mut content = String::new();
        ifile.file.read_to_string(&mut content).unwrap();
        return Object::String(content);
    }
    panic!("Unexpected internal arg: {}", &args[0])
}

