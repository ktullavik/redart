

static TESTPATH: &str = "/usr/home/kt/devel/redart/test";
static FAILTESTPATH: &str = "/usr/home/kt/devel/redart/testfail";



pub fn get_failfilepath(s: String) -> String {

    let filename = match s.as_str() {
        "1" => "1.cross_function_leak.dart",
        "2" => "2.double_declaration.dart",
        "3" => "3.forgotten_paramlist.dart",
        "4" => "4.plus_is_not_prefix.dart",
        x => panic!("Unknown failtest: {}", x)
    };
    return format!("{}/{}", FAILTESTPATH, filename);
}


pub fn get_filepath(s: String) -> String {

    let filename = match s.as_str() {
        "1" => "1.hello.dart",
        "2" => "2.variable.dart",
        "3" => "3.addition.dart",
        "4" => "4.subtraction.dart",
        "5" => "5.multiplication.dart",
        "6" => "6.division.dart",
        "7" => "7.funcall.dart",
        "8" => "8.argpass.dart",
        "9" => "9.evaled_argpass.dart",
        "10" => "10.arithmetic.dart",
        "11" => "11.conditional.dart",
        "12" => "12.conditional2.dart",
        "13" => "13.conditional3.dart",
        "14" => "14.conditional4.dart",
        "15" => "15.conditional5.dart",
        "16" => "16.mutate.dart",
        "17" => "17.mutate_self.dart",
        "18" => "18.post_increment.dart",
        "19" => "19.post_decrement.dart",
        "20" => "20.pre_increment.dart",
        "21" => "21.pre_decrement.dart",
        "22" => "22.returnvalue.dart",
        "23" => "23.logical_or.dart",
        "24" => "24.logical_and.dart",
        "25" => "25.logical_expr.dart",
        "26" => "26.less_than.dart",
        "27" => "27.greater_than.dart",
        "28" => "28.less_or_equal.dart",
        "29" => "29.greater_or_equal.dart",
        "30" => "30.equality.dart",
        "31" => "31.equality2.dart",
        "32" => "32.equality3.dart",
        "33" => "33.recursion.dart",
        "34" => "34.unary_minus.dart",
        "35" => "35.arg_expression.dart",
        "36" => "36.arg_expression2.dart",
        "37" => "37.arg_expression3.dart",
        "38" => "38.arg_expression4.dart",
        "39" => "39.not.dart",
        "40" => "40.fibonacci.dart",
        "41" => "41.difficult_return.dart",
        "42" => "42.bitand.dart",
        "43" => "43.bitor.dart",
        "44" => "44.bitxor.dart",
        "45" => "45.left_associative_sum.dart",
        "46" => "46.hard_expression.dart",
        "47" => "47.left_associative_product.dart",
        "48" => "48.string_concat.dart",
        "49" => "49.lexical_scope.dart",
        "50" => "50.method_reading_field.dart",
        "51" => "51.constructor_setting_field.dart",
        "52" => "52.constructor_setting_field_from_arg.dart",
        "53" => "53.method_postincrementing_field.dart",
        "54" => "54.method_postdecrementing_field.dart",
        "55" => "55.method_preincrementing_field.dart",
        "56" => "56.method_predecrementing_field.dart",
        "57" => "57.string_interpolation.dart",
        "58" => "58.string_interpolation2.dart",
        "59" => "59.string_interpolation3.dart",
        "60" => "60.string_interpolation4.dart",
        "61" => "61.semicolon_king.dart",

        s => s
    };

    return format!("{}/{}", TESTPATH, filename);
}
