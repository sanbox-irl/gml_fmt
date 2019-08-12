var argi = 0,
    sprite = argument[argi++],
    sub_img = argument[argi++],
    width = (argument_count > 2) ? argument[argi++] : undefined,
    height = (argument_count > 3) ? argument[argi++] : undefined,
    frame_padding = (argument_count > 4) ? argument[argi++] : -1,
    bgr = (argument_count > 5) ? argument[argi++] : 0,
    bgg = (argument_count > 6) ? argument[argi++] : 0,
    bgb = (argument_count > 7) ? argument[argi++] : 0,
    bga = (argument_count > 8) ? argument[argi++] : 0, 
    tintr = (argument_count > 9)  ? argument[argi++] : 1.0, 
    tintg = (argument_count > 10) ? argument[argi++] : 1.0, 
    tintb = (argument_count > 11) ? argument[argi++] : 1.0, 
    tinta = (argument_count > 12) ? argument[argi++] : 1.0;


if is_undefined(width) width = sprite_get_width(sprite);
if is_undefined(height) height = sprite_get_height(sprite);

var ret = false;
