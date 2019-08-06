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
