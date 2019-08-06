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

#[test]
fn do_until() {
    let input = "do {
// whatever
show_debug_message(x);
} until (test);";

    let format = "do {
    // whatever
    show_debug_message(x);
} until (test);
";

    assert_eq!(gml_fmt::run_test(input), format);
}

#[test]
fn for_loops() {
    let input = "for (var x = 0; i < 10; i++) {
show_debug_message(\"test\");
for (; i < 10; i++) {
show_debug_message(3);
}
}
for (var i;; i++) {
	show_debug_message(3);	
}

for (var i;;) {
	show_debug_message(4);
}
";

    let format = "for (var x = 0; i < 10; i++) {
    show_debug_message(\"test\");
    for (; i < 10; i++) {
        show_debug_message(3);
    }
}
for (var i;; i++) {
    show_debug_message(3);
}

for (var i;;) {
    show_debug_message(4);
}

";

    assert_eq!(gml_fmt::run_test(input), format);
}

#[test]
fn decimal_number() {
    let input = "var x = .3;";

    let format = "var x = 0.3;
";

    assert_eq!(gml_fmt::run_test(input), format);
}
