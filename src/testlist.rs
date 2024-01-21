
pub static TESTPATH: &str = "/usr/home/kt/devel/redart/test";
pub static FAILTESTPATH: &str = "/usr/home/kt/devel/redart/testfail";

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
    "100.multifile/main.dart",
    "101.external_constructor/main.dart"
];


pub const FAILTESTS: &'static [&'static str] = &[
    "0.cross_function_leak.dart",
    "1.double_declaration.dart",
    "2.forgotten_paramlist.dart",
    "3.plus_is_not_prefix.dart",
];


pub fn get_failfilepath(s: String) -> String {

    return match s.parse::<usize>() {
        Ok(i) => {
            format!("{}/{}", FAILTESTPATH, FAILTESTS[i])
        },
        Err(_e) => {
            format!("{}/{}", FAILTESTPATH, s)
        },
    };
}


pub fn get_filepath(s: String) -> String {

    return match s.parse::<usize>() {
        Ok(i) => {
            format!("{}/{}", TESTPATH, TESTS[i])
        },
        Err(_e) => {
            format!("{}/{}", TESTPATH, s)
        },
    };
}
