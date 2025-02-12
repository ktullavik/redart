use crate::state::State;
use crate::object::Object;
use crate::node::Node;
use crate::evalhelp::argnodes_to_argobjs;
use crate::api;


pub fn has_function(name: &str) -> bool {
    match name {
        "assert" |
        "print" |
        "__IO_FILE_CONSTRUCT" |
        "__IO_FILE_READ_AS_STRING" |
        "__LIST_ADD" |
        "__LIST_ADDALL" |
        "__LIST_CLEAR" |
        "__LIST_GET_FIRST"  |
        "__LIST_GET_LAST"   |
        "__LIST_GET_LENGTH" |
        "__LIST_INSERT" |
        "__LIST_REMOVEAT" |
        "__LIST_REMOVELAST" |
        "__LIST_REMOVERANGE" |
        "__LIST_SHUFFLE" |
        "__LIST_TOSTRING" |
        "__MATH_ACOS" |
        "__MATH_ASIN" |
        "__MATH_ATAN" |
        "__MATH_ATAN2" |
        "__MATH_COS" |
        "__MATH_EXP" |
        "__MATH_LOG" |
        "__MATH_MAX" |
        "__MATH_MIN" |
        "__MATH_POW" |
        "__MATH_SIN" |
        "__MATH_SQRT" |
        "__MATH_TAN" |
        "__MATH_NEXT_BOOL" |
        "__MATH_NEXT_DOUBLE" |
        "__MATH_NEXT_INT"
        => true,
        _ => false
    }
}


pub fn call(fnode: &Node, name: &str, state: &mut State) -> Object {

    let argnodes = &fnode.children[0].children;
    let args = argnodes_to_argobjs(argnodes, state);

    return match name {

        "assert" => {
            api::top::assert(fnode, argnodes, args, state)
        }
        "print" => {
            api::top::print(fnode, argnodes, args, state)
        }
        "__IO_FILE_CONSTRUCT" => {
            api::io::file_construct(fnode, argnodes, args, state)
        }
        "__IO_FILE_READ_AS_STRING" => {
            api::io::file_read_as_string(fnode, args, state)
        }
        "__LIST_ADD" => {
            api::list::add(fnode, args, state)
        }
        "__LIST_ADDALL" => {
            api::list::add_all(fnode, argnodes, args, state)
        }
        "__LIST_CLEAR" => {
            api::list::clear(fnode, args, state)
        }
        "__LIST_GET_FIRST" => {
            api::list::get_first(args, state)
        }
        "__LIST_GET_LAST" => {
            api::list::get_last(args, state)
        }
        "__LIST_GET_LENGTH" => {
            api::list::get_length(args, state)
        }
        "__LIST_INSERT" => {
            api::list::insert(fnode, argnodes, args, state)
        }
        "__LIST_REMOVEAT" => {
            api::list::remove_at(fnode, argnodes, args, state)
        }
        "__LIST_REMOVELAST" => {
            api::list::remove_last(fnode, args, state)
        }
        "__LIST_REMOVERANGE" => {
            api::list::remove_range(fnode, argnodes, args, state)
        }
        "__LIST_SHUFFLE" => {
            api::list::shuffle(fnode, args, state)
        }
        "__LIST_TOSTRING" => {
            api::list::to_string(fnode, args, state)
        }
        "__MATH_ACOS" => {
            api::math::acos(fnode, argnodes, args, state)
        }
        "__MATH_ASIN" => {
            api::math::asin(fnode, argnodes, args, state)
        }
        "__MATH_ATAN" => {
            api::math::atan(fnode, argnodes, args, state)
        }
        "__MATH_ATAN2" => {
            api::math::atan2(fnode, argnodes, args, state)
        }
        "__MATH_COS" => {
            api::math::cos(fnode, argnodes, args, state)
        }
        "__MATH_EXP" => {
            api::math::exp(fnode, argnodes, args, state)
        }
        "__MATH_LOG" => {
            api::math::log(fnode, argnodes, args, state)
        }
        "__MATH_MAX" => {
            api::math::max(fnode, argnodes, args, state)
        }
        "__MATH_MIN" => {
            api::math::min(fnode, argnodes, args, state)
        }
        "__MATH_POW" => {
            api::math::pow(fnode, argnodes, args, state)
        }
        "__MATH_SIN" => {
            api::math::sin(fnode, argnodes, args, state)
        }
        "__MATH_SQRT" => {
            api::math::sqrt(fnode, argnodes, args, state)
        }
        "__MATH_TAN" => {
            api::math::tan(fnode, argnodes, args, state)
        }
        "__MATH_NEXT_BOOL" => {
            api::math::next_bool(fnode, args, state)
        }
        "__MATH_NEXT_DOUBLE" => {
            api::math::next_double(fnode, args, state)
        }
        "__MATH_NEXT_INT" => {
            api::math::next_int(fnode, argnodes, args, state)
        }

        _ => panic!("Unknown command: {}", name)
    }
}

