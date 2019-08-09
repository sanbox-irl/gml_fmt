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
    let input = "var x = .3; var z = 3.;";

    let format = "var x = 0.3; var z = 3.0;
";

    assert_eq!(gml_fmt::run_test(input), format);
}

#[test]
fn trailing_commas() {
    let input = "func(a,b,c,);";
    let format = "func(a, b, c);
";

    assert_eq!(gml_fmt::run_test(input), format);
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

    assert_eq!(gml_fmt::run_test(input), format);
}

#[test]
fn do_until_double_loop() {
    let input = "do {
    
    //If not already visited
    if (_grid[# _x, _y] != _val){
        
        if (array_find_index(_immunes,_val) == -1){
            _grid[# _x, _y] = _val;
        
            ++_carvedCells;
        }
    }
    
    //Wander cell
    var _dir = irandom(3) * 90;
    _x += lengthdir_x(1,_dir);
    _y += lengthdir_y(1,_dir);
    
}
until(_carvedCells / _cells > _carveRatio);
";

    let output = "do {
    //If not already visited
    if (_grid[# _x, _y] != _val) {
        if (array_find_index(_immunes, _val) == -1) {
            _grid[# _x, _y] = _val;
        
            ++_carvedCells;
        }
    }
    
    //Wander cell
    var _dir = irandom(3) * 90;
    _x += lengthdir_x(1, _dir);
    _y += lengthdir_y(1, _dir);
} until(_carvedCells / _cells > _carveRatio);
";

    assert_eq!(gml_fmt::run_test(input), output);
}

#[test]
fn if_with_line() {
    let input = "if (delay < 0)
{
    var exists = false;
    with (target)
    {
        var dir = point_direction(other.x, other.y, mid_x(), mid_y());
        with (other)
        {
            hspeed = hsp;
            vspeed = vsp;
            direction = angle_approach(direction, dir, rot);
            if (collision_circle(x, y, 2, target, false, false))
            {
                event_user(0);
                instance_destroy();
            }
            speed = approach(speed, max_spd, acc);
            hsp = hspeed;
            vsp = vspeed;
            speed = 0;
            rot += rot_add * global.delta;
        }
        exists = true;
    }
    life -= global.delta;
    if (life < 0 || !exists)
    {
        instance_destroy();
    }
    perform_destroy_event = true;
}
// Movement
x += hsp*global.delta;
y += vsp*global.delta;
";

    let output = "if (delay < 0) {
    var exists = false;
    with (target) {
        var dir = point_direction(other.x, other.y, mid_x(), mid_y());
        with (other) {
            hspeed = hsp;
            vspeed = vsp;
            direction = angle_approach(direction, dir, rot);
            if (collision_circle(x, y, 2, target, false, false)) {
                event_user(0);
                instance_destroy();
            }
            speed = approach(speed, max_spd, acc);
            hsp = hspeed;
            vsp = vspeed;
            speed = 0;
            rot += rot_add * global.delta;
        }
        exists = true;
    }
    life -= global.delta;
    if (life < 0 || !exists) {
        instance_destroy();
    }
    perform_destroy_event = true;
}
// Movement
x += hsp * global.delta;
y += vsp * global.delta;
";
    assert_eq!(gml_fmt::run_test(input), output);
}

#[test]
fn do_access_cascading() {
    let input = "if (instance_exists(shields[i])&& shields[i] .charge < 0)
{
    var c = shields[i].laser_charge_max;
    var l = shields[i].laser_max;
    shields[i].laser_charge = c;
    shields[i].delta_alarm[0] = 30;
}
";
    let output = "if (instance_exists(shields[i]) && shields[i].charge < 0) {
    var c = shields[i].laser_charge_max;
    var l = shields[i].laser_max;
    shields[i].laser_charge = c;
    shields[i].delta_alarm[0] = 30;
}
";
    assert_eq!(gml_fmt::run_test(input), output);
}
