use gml_fmt;

#[test]
fn regions() {
    let input = "#region Test Test  Test


#endregion Okay
";
    let format = "#region Test Test Test

#endregion Okay

";
    assert_eq!(gml_fmt::run_test(input), format);
}

#[test]
fn multiline_string() {
    let input = "@\"Test sure  yup\"";
    let format = "@\"Test sure  yup\"
";

    assert_eq!(gml_fmt::run_test(input), format);
}

#[test]
fn else_if() {
    let input = "if (xx < (1  2.75)) {
    return x;
} else if (xx < (2 / 2.75)) {
   return z;
}";
    let format = "if (xx < (1 2.75)) {
    return x;
} else if (xx < (2 / 2.75)) {
    return z;
}
";

    assert_eq!(gml_fmt::run_test(input), format);
}

#[test]
fn series_of_declarations() {
    let input = "var function, xx, xx2, xxm1;
var x = 2, y, var q";
    let format = "var function, xx, xx2, xxm1;
var x = 2, y, var q
";

    assert_eq!(gml_fmt::run_test(input), format);
}
