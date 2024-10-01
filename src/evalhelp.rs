use crate::{object::Object, state::State, utils::dart_evalerror};



pub fn set_list_element(ulist_ref: Object, index: Object, value: Object, state: &mut State) -> Object {
    
    println!("set_list_element");
    match ulist_ref {

        Object::Reference(ulist_rk) => {

            let ulist = state.objsys.get_instance(&ulist_rk);
            let ilist_ref = ulist.get_field(String::from("__list")).clone();

            if let Object::Reference(ilist_rk) = ilist_ref {
                let ilist = state.objsys.get_list_mut(&ilist_rk);


                if let Object::Int(i) = index {
                    if i < 0 {
                        dart_evalerror("Index must be positive.", state)
                    }

                    println!("Setting internal list element {}", value);
                    ilist.set_el(i as usize, value);
                    return Object::Null;
                }
            }
            panic!("Expected reference when setting list element.")
        }

        x => panic!("Unexpected token: {}", x)
    }
}

