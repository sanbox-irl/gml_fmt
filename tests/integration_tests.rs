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
