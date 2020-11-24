use gml_fmt_lib::*;

const LANG_CONFIG: LangConfig = LangConfig {
    use_spaces: true,
    space_size: 4,
    newlines_at_end: 1,
};

fn run_test(input: &str) -> String {
    run(input, &LANG_CONFIG, None).expect("Panicked during Integration Test!")
}

#[test]
fn regions() {
    let input = "#region Test Test  Test


#endregion Okay
";
    let format = "#region Test Test  Test

#endregion Okay
";
    assert_eq!(run_test(input), format);
}

#[test]
fn multiline_string() {
    let input = "@\"Test sure  yup\"";
    let format = "@\"Test sure  yup\";
";

    assert_eq!(run_test(input), format);
}

#[test]
fn else_if() {
    let input = "if (xx < (1  2.75)) {
    return x;
}
else if (xx < (2 / 2.75)) {
   return z;
}";
    let format = "if (xx < (1 2.75)) {
    return x;
} else if (xx < (2 / 2.75)) {
    return z;
}
";

    assert_eq!(run_test(input), format);
}

#[test]
fn series_of_declarations() {
    let input = "var fn, xx, xx2, xxm1;
var x = 2, y, var q";
    let format = "var fn, xx, xx2, xxm1;
var x = 2, y, var q;
";

    assert_eq!(run_test(input), format);
}

#[test]
fn do_until() {
    let input = "do {
// whatever
show_debug_message(x);
} until (test);

do
{
    // gah
}
until (true);
";

    let format = "do {
    // whatever
    show_debug_message(x);
} until (test);

do {
    // gah
} until (true);
";

    assert_eq!(run_test(input), format);
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

    assert_eq!(run_test(input), format);
}

#[test]
fn function_definition() {
    let input = "function fn_name(arg1,arg2){
show_debug_message(0);
}
";
    let format = "function fn_name(arg1, arg2) {
    show_debug_message(0);
}
";

    assert_eq!(run_test(input), format);
}

#[test]
fn function_var_assignment() {
    let input = "fn_name=function(arg1,arg2){
show_debug_message(0);
}
var fn_var=function(arg1,arg2){
show_debug_message(0);
}
";
    let format = "fn_name = function(arg1, arg2) {
    show_debug_message(0);
}
var fn_var = function(arg1, arg2) {
    show_debug_message(0);
}
";

    assert_eq!(run_test(input), format);
}

#[test]
fn function_constructor() {
    let input = "function fn_name(arg1,arg2)constructor{
fn_debug=function(arg1,arg2){
show_debug_message(0);
}
}

fn_var=function(arg1,arg2)constructor{
show_debug_message(0);
}
";
    let format = "function fn_name(arg1, arg2) constructor {
    fn_debug = function(arg1, arg2) {
        show_debug_message(0);
    }
}

fn_var = function(arg1, arg2) constructor {
    show_debug_message(0);
}
";

    assert_eq!(run_test(input), format);
}

#[test]
fn function_constructor_call() {
    let input = "_structObj=new fn_name(0,0);
_varObj=new fn_var(0,0);
";

    let format = "_structObj = new fn_name(0, 0);
_varObj = new fn_var(0, 0);
";
    assert_eq!(run_test(input), format);
}

#[test]
fn function_lambda() {
    let input = "fn(arg1,function(i) { show_debug_message(0) })

fn(arg1,function(i) { show_debug_message(0); show_debug_message(1); })

function fn_name(arg1, arg2) constructor {
fn(arg1,function(i) { show_debug_message(0) })

fn(arg1,function(i) { show_debug_message(0); show_debug_message(1); })
}
";
    let format = "fn(arg1, function(i) { show_debug_message(0) });

fn(arg1, function(i) {
    show_debug_message(0);
    show_debug_message(1);
});

function fn_name(arg1, arg2) constructor {
    fn(arg1, function(i) { show_debug_message(0) });
    
    fn(arg1, function(i) {
        show_debug_message(0);
        show_debug_message(1);
    });
}
";

    assert_eq!(run_test(input), format);
}

#[test]
fn struct_new_argument() {
    let input = "fn(argument,new _struct(property1,property2))";
    let format = "fn(argument, new _struct(property1, property2));
";
    assert_eq!(run_test(input), format);
}

#[test]
fn struct_new_return() {
    let input = "fn_debug = function(arg1,arg2) {
show_debug_message(0)
return new _struct
}
";
    let format = "fn_debug = function(arg1, arg2) {
    show_debug_message(0);
    return new _struct;
}
";
    assert_eq!(run_test(input), format);
}

#[test]
fn struct_delete() {
    let input = "delete _struct";
    let format = "delete _struct;
";
    assert_eq!(run_test(input), format);
}

#[test]
fn decimal_number() {
    let input = "var x = .3; var z = 3.;";

    let format = "var x = 0.3;
var z = 3.0;
";

    assert_eq!(run_test(input), format);
}

#[test]
fn trailing_commas() {
    let input = "func(a,b,c,);";
    let format = "func(a, b, c,);
";

    assert_eq!(run_test(input), format);
}

#[test]
fn not_trailing_commas() {
    let input = "fun(
    _e[Foo.bar],
    _e[Foo.bar],
    _e[Foo.bar]
);";

    let output = "fun(
    _e[Foo.bar],
    _e[Foo.bar],
    _e[Foo.bar]
);
";
    assert_eq!(run_test(input), output);
}

#[test]
fn enum_test() {
    let input = "enum EInputConstants{//Starts at high negative number to not interfere with other input constants
//P - Positive axis (Axis is regular coordinates with 0;0 being bottom left)
//N - Negative axis
//Note that return is always positive
GP_AXISLV_P = -100,
GP_AXISLV_N,   GP_AXISLH_P,
GP_AXISLH_N,
GP_AXISRV_P, //down
GP_AXISRV_N, //up
    
GP_AXISRH_P, /* gah a test */ GP_AXISRH_N,
SCROLL_DOWN,
SCROLL_UP,ANY,NONE
}";

    let format = "enum EInputConstants { //Starts at high negative number to not interfere with other input constants
    //P - Positive axis (Axis is regular coordinates with 0;0 being bottom left)
    //N - Negative axis
    //Note that return is always positive
    GP_AXISLV_P = -100,
    GP_AXISLV_N,
    GP_AXISLH_P,
    GP_AXISLH_N,
    GP_AXISRV_P, //down
    GP_AXISRV_N, //up
    
    GP_AXISRH_P, /* gah a test */
    GP_AXISRH_N,
    SCROLL_DOWN,
    SCROLL_UP,
    ANY,
    NONE
}
";

    assert_eq!(run_test(input), format);
}

#[test]
fn do_until_double_loop() {
    let input = "do {
    
    //If foo
    if (_a[# _x, _y] != _val){
        
        if (array_find_index(_gah,_goo) == -1){
            _a[# _x, _y] = _val;
            ++sha;
        }
    }
    
    //bar
    var _dir = irandom(3) * 200;
    _x += lengthdir_x(1,_dir);
    _y += lengthdir_y(1,_dir);
    
}
until(_gah / _boo > _bah);
";

    let output = "do {
    //If foo
    if (_a[# _x, _y] != _val) {
        if (array_find_index(_gah, _goo) == -1) {
            _a[# _x, _y] = _val;
            ++sha;
        }
    }
    
    //bar
    var _dir = irandom(3) * 200;
    _x += lengthdir_x(1, _dir);
    _y += lengthdir_y(1, _dir);
} until (_gah / _boo > _bah);
";

    assert_eq!(run_test(input), output);
}

#[test]
fn if_with_line() {
    let input = "if (a < 0)
{
    var b = false;
    with (ident)
    {
        var dir = point_direction(other.x, other.y, a(), b());
        with (other)
        {
            hspeed = hsp;
            vspeed = vsp;
            direction = angle_approach(direction, dir, rot);
            if (a(x, y, 2, target, false, false))
            {
                z(0);
                e();
            }
            speed = approach(speed, max_spd, acc);
            hsp = hspeed;
            vsp = vspeed;
            speed = 0;
            a += t * global.delta;
        }
        exists = true;
    }
    g -= global.delta;
    if (g < 0 || !exists)
    {
        a();
    }
    val = true;
}
// Foo
x += hsp*ss;
y += vsp*ss;";

    let output = "if (a < 0) {
    var b = false;
    with (ident) {
        var dir = point_direction(other.x, other.y, a(), b());
        with (other) {
            hspeed = hsp;
            vspeed = vsp;
            direction = angle_approach(direction, dir, rot);
            if (a(x, y, 2, target, false, false)) {
                z(0);
                e();
            }
            speed = approach(speed, max_spd, acc);
            hsp = hspeed;
            vsp = vspeed;
            speed = 0;
            a += t * global.delta;
        }
        exists = true;
    }
    g -= global.delta;
    if (g < 0 || !exists) {
        a();
    }
    val = true;
}
// Foo
x += hsp * ss;
y += vsp * ss;
";
    assert_eq!(run_test(input), output);
}

#[test]
fn do_access_cascading() {
    let input = "if (a(b[i])&& b[i] .c < 0)
{
    var c = b[i].q;
    var l = b[i].q;
    b[i].q = c;
    b[i].q[0] = 30;
}
";
    let output = "if (a(b[i]) && b[i].c < 0) {
    var c = b[i].q;
    var l = b[i].q;
    b[i].q = c;
    b[i].q[0] = 30;
}
";
    assert_eq!(run_test(input), output);
}

#[test]
fn ending_delimiter_enum() {
    let input = "colour = choose(
4, // foo
5, // foor
6, // foo,
7, // bar
)";

    let output = "colour = choose(
    4, // foo
    5, // foor
    6, // foo,
    7, // bar
);
";

    assert_eq!(run_test(input), output);
}

#[test]
fn double_call() {
    let input = "func(
    a,
    b,
    c[a],
    d[Y],
    5,
    4,
    1,
    global.q * 0.13,
    between(a, c * 0.72, u * 0.76) ||
    between(b, d * 0.82, u * 0.86) ||
    between(c, c * 0.92, u * 0.96) ? c_white : c_red
);
";

    assert_eq!(run_test(input), input);
}

#[test]
fn non_block_if_else() {
    let input = "if (true)
    return false;
else if (true)
    return false;
if (true)
    return false;
else if (true)
    return false;

if (true)
    return false;

if (true) return false;
";

    assert_eq!(run_test(input), input);
}

#[test]
fn horrible_multiline_string() {
    let input = "var _bulletNormal = string_join(@\'{
    \"type\": \"normal\",
    \"object\": \', Obj_Bullet, @\',
    \"speed\": 260,
    \"count\": 1,
    \"damage\": 1,
    \"knockback\": 0.1,
    \"lifetime\": 8000
}\');
";
    assert_eq!(run_test(input), input);
}

#[test]
fn nice_macro() {
    let input = "#macro give_me_five x =5+5;";
    let output = "#macro give_me_five x =5+5;
";

    assert_eq!(run_test(input), output);
}

#[test]
fn whitesmith_enum() {
    let input = "enum YosiFunction
    {
    init,
    main,
    new_player,
    new_zapper,
    new_laser,
    move_ground,
    rect,
    blueprint_read,
    player_die,
    }
";

    let output = "enum YosiFunction {
    init,
    main,
    new_player,
    new_zapper,
    new_laser,
    move_ground,
    rect,
    blueprint_read,
    player_die,
}
";

    assert_eq!(run_test(input), output);
}

#[test]
fn whitesmith_control_statement() {
    let input = "while true
    {
    // who would format like this
    }
";

    let output = "while true {
    // who would format like this
}
";

    assert_eq!(run_test(input), output);
}

#[test]
fn bad_for_loop() {
    let input = "for (var xx = clamp((global.cameraLeft - 25) div 192, 0, 6); xx <= clamp((global.cameraRight + 25) div 192, 0, 6); ++xx;) {
// comments
}";

    let output = "for (var xx = clamp((global.cameraLeft - 25) div 192, 0, 6); xx <= clamp((global.cameraRight + 25) div 192, 0, 6); ++xx) {
    // comments
}
";

    assert_eq!(run_test(input), output);
}

#[test]
fn expression_no_semicolon_test() {
    let input = "call(z)
call(q)
x = 20
y = 10
";
    let output = "call(z);
call(q);
x = 20;
y = 10;
";
    assert_eq!(run_test(input), output);
}

#[test]
fn mix_no_semicolon() {
    let input = "global.roundOver=true
alarm[3]=room_speed/10;
audio_stop_sound(aMusicTitle);";

    let output = "global.roundOver = true;
alarm[3] = room_speed / 10;
audio_stop_sound(aMusicTitle);
";

    assert_eq!(run_test(input), output);
}
