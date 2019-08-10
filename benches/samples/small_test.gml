var _break = 0;
var _rooms = 1; //because we already have the starting room
do
{
ds_grid_copy(_tempGrid,floorGrid);

for(var _x = 0; _x < floorWidth; _x++)
    {
    if(_rooms >= roomAmount) break; // make sure we can even add a room
    for(var _y = 0; _y < floorHeight; _y++)
        {
        if(_rooms >= roomAmount) break; // make sure we can even add a room
        if(floorGrid[# _x, _y] != 0) // check if this isn't an empty room
            {
            if(_rooms >= roomAmount) break; // make sure we can even add a room
            for(var _x2 = -1; _x2 <= 1; _x2++)
                {
                if(_rooms >= roomAmount) break; // make sure we can even add a room
                for(var _y2 = -1; _y2 <= 1; _y2++)
                    {
                    if(_rooms >= roomAmount) break; // make sure we can even add a room
                   
                    if(random(1) < _roomChance) // random chance
                        {
                        if((_x2 = 0 && _y2 != 0) or (_y2 = 0 && _x2 != 0)) //check if it's an adjacent space (not diagonal)
                            {
                            if(_x+_x2 == median(0,_x+_x2,floorWidth-1) && _y+_y2 == median(0,_y+_y2,floorHeight-1)) //check if it's inside the grid
                                {
                                if(floorGrid[# _x+_x2, _y+_y2] == 0 && _tempGrid[# _x+_x2, _y+_y2] == 0) // check to see if the space is empty
                                    {
                                    _rooms++;
                                    if(_rooms = roomAmount)
                                        {
                                        _tempGrid[# _x+_x2, _y+_y2] = 3; // Boss room
                                        }
                                    else
                                        {//if this isn't the last room it's just a normal room
                                        _tempGrid[# _x+_x2, _y+_y2] = 1;
                                       
                                       
                                        //we add the coordinates to these lists, later this will be used for selecting special rooms like:
                                        //treasure rooms/stores/secrets/mini bosses
                                        ds_list_add(_tempListx,_x+_x2);
                                        ds_list_add(_tempListy,_y+_y2);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
ds_grid_copy(floorGrid,_tempGrid);
_break++;
}
until (_rooms >= roomAmount) or (_break == 50*roomAmount);
