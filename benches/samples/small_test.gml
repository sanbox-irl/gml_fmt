


///@func switch_room
///@desc Switches rooms and updates internal room system. 
///@param room_to_switch_to The name of the room to switch to.
///@param {string|array} Use a string or an array to indicate the side
///@param previous_room? Set a room as the previous room. If not put here, previous room will be this room.

vertical = 20;

// Go to the "next" room in this case.
if (argument_count == 0) {
    show_error("No room set as destination room.", true);
}

// Set the Next Room
__room_set_next_room(argument[0]);

// Set the position of the next room
global.room_system[RoomSystem.PlayerLocation] = argument[1];

// Set the Previous room
var _this_tuple = [0, 0];
if (instance_exists(objPlayer)) {
    _this_tuple[VEC2_X] = objPlayer.x;
    _this_tuple[VEC2_Y] = objPlayer.y;
}

if (argument_count == 3) {
    __room_set_previous_room(argument[1], [_this_tuple[VEC2_X], _this_tuple[VEC2_Y]]);
} else {
    // Set the current room as the next previous room
    __room_set_previous_room(__room_get_current_room(), [_this_tuple[VEC2_X     ], _this_tuple[VEC2_Y       ]]);
}