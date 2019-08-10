if (a < 0)
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
y += vsp*ss;