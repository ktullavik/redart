use dirs::Dirs;


pub const TESTS: &'static [&'static str] = &[
    "0.void.dart",
    "1.hello.dart",
    "2.variable.dart",
    "3.addition.dart",
    "4.subtraction.dart",
    "5.multiplication.dart",
    "6.division.dart",
    "7.funcall.dart",
    "8.argpass.dart",
    "9.evaled_argpass.dart",
    "10.arithmetic.dart",
    "11.conditional.dart",
    "12.conditional2.dart",
    "13.conditional3.dart",
    "14.conditional4.dart",
    "15.conditional5.dart",
    "16.mutate.dart",
    "17.mutate_self.dart",
    "18.post_increment.dart",
    "19.post_decrement.dart",
    "20.pre_increment.dart",
    "21.pre_decrement.dart",
    "22.returnvalue.dart",
    "23.logical_or.dart",
    "24.logical_and.dart",
    "25.logical_expr.dart",
    "26.less_than.dart",
    "27.greater_than.dart",
    "28.less_or_equal.dart",
    "29.greater_or_equal.dart",
    "30.equality.dart",
    "31.equality2.dart",
    "32.equality3.dart",
    "33.recursion.dart",
    "34.unary_minus.dart",
    "35.arg_expression.dart",
    "36.arg_expression2.dart",
    "37.arg_expression3.dart",
    "38.arg_expression4.dart",
    "39.not.dart",
    "40.fibonacci.dart",
    "41.difficult_return.dart",
    "42.bitand.dart",
    "43.bitor.dart",
    "44.bitxor.dart",
    "45.left_associative_sum.dart",
    "46.hard_expression.dart",
    "47.left_associative_product.dart",
    "48.string_concat.dart",
    "49.lexical_scope.dart",
    "50.method_reading_field.dart",
    "51.constructor_setting_field.dart",
    "52.constructor_setting_field_from_arg.dart",
    "53.method_postincrementing_field.dart",
    "54.method_postdecrementing_field.dart",
    "55.method_preincrementing_field.dart",
    "56.method_predecrementing_field.dart",
    "57.string_interpolation.dart",
    "58.string_interpolation2.dart",
    "59.string_interpolation3.dart",
    "60.string_interpolation4.dart",
    "61.semicolon_king.dart",
    "62.while.dart",
    "63.dowhile.dart",
    "64.for.dart",
    "65.for2.dart",
    "66.this_constructor_arg.dart",
    "67.access_object_field.dart",
    "68.chained_access.dart",
    "69.chained_methods.dart",
    "70.nested_function.dart",
    "71.nested_functions.dart",
    "72.bodyless_constructor.dart",
    "73.implicit_constructor.dart",
    "74.external_setget.dart",
    "75.external_setget2.dart",
    "76.change_field_instance.dart",
    "77.shadow_field.dart",
    "78.busy_garbageman.dart",
    "79.string_interpolation_with_dot.dart",
    "80.read_file_as_string.dart",
    "81.list_add_element.dart",
    "82.list_clear.dart",
    "83.list_remove_last.dart",
    "84.list_forloop.dart",
    "85.while_counter.dart",
    "86.forloop_counter.dart",
    "500.multifile/main.dart",
    "501.external_constructor/main.dart"
];


pub const FAILTESTS: &'static [&'static str] = &[
    "0.cross_function_leak.dart",
    "1.double_declaration.dart",
    "2.forgotten_paramlist.dart",
    "3.plus_is_not_prefix.dart",
    "4.var_not_declared.dart",
    "5.var_not_declared2.dart",
    "6.extraneous_comma.dart",
    "7.illegal_operand_pluss.dart",
    "8.illegal_operand_minus.dart",
    "500.non_transitive_imports/main.dart",
];


pub fn get_failfilepath(s: String, paths: &Dirs) -> String {

    return match s.parse::<usize>() {
        Ok(i) => {
            format!("{}/{}", paths.failtestdir(), FAILTESTS[i])
        },
        Err(_e) => {
            format!("{}/{}", paths.failtestdir(), s)
        },
    };
}


pub fn get_filepath(s: String, paths: &Dirs) -> String {

    return match s.parse::<usize>() {
        Ok(i) => {
            format!("{}/{}", paths.testdir(), TESTS[i])
        },
        Err(_e) => {
            format!("{}/{}", paths.testdir(), s)
        },
    };
}
