#region generate room structure
switch(_type)
    {
    #region Plains of Existence
   
    #region -1 Plains of Existence - Starting Room
    case -1: //Starting room
        {
        var _roomx1 = 5;
        var _roomy1 = 3;
        var _roomx2 = roomWidth-6;
        var _roomy2 = roomHeight-4;
       
        ds_grid_set_region(_grid,_roomx1,_roomy1,_roomx2,_roomy2,FREE);
        _grid[# _roomx1,_roomy1] = BLOCKED;
        _grid[# _roomx2,_roomy1] = BLOCKED;
        _grid[# _roomx1,_roomy2] = BLOCKED;
        _grid[# _roomx2,_roomy2] = BLOCKED;
       
        _listDecals = ds_list_create();
        ds_list_add(_listDecals,Spr_Entity_Starting_Altar_Shadow,0,roomWidth*8,roomHeight*8);
        ds_list_add(_listDecals,Spr_Entity_Starting_Altar,0,roomWidth*8,roomHeight*8);
       
        var _spriteWall = Spr_Wall_Stone;
        var _spriteFloor = Spr_Floor_Stone_Tile;
        }break;
    #endregion
   
   
    #region 0 Plains of Existence - Ash Rooom
    case 0: //Ash room
        {
        var _roomx1 = 2;
        var _roomy1 = 3;
        var _roomx2 = roomWidth-3;
        var _roomy2 = roomHeight-3;
        var _firstLoop = true;
        repeat(4)
            {
            var _xx = roomWidth div 2;
            var _yy = roomHeight div 2;
            var _dir = irandom(3);
            var _break = irandom_range(60,120)+100*(!_firstLoop);
       
            while(_break > 0)
                {
                _break--;
                if(_firstLoop)
                    {
                    _grid[# _xx-1, _yy] = FREE;
                    _grid[# _xx, _yy-1] = FREE;
                    _grid[# _xx+1, _yy] = FREE;
                    _grid[# _xx, _yy+1] = FREE;
                    }
                _grid[# _xx, _yy] = FREE;
                _xx += lengthdir_x(1,_dir*90);
                _yy += lengthdir_y(1,_dir*90);
           
                _dir = irandom(3);
               
                if(_xx <= _roomx1) _dir = 0;
                if(_xx >= _roomx2) _dir = 2;
                if(_yy <= _roomy1) _dir = 3;
                if(_yy >= _roomy2) _dir = 1;
                }
            _firstLoop = false;
            }
       
       
       
        var _spriteWall = Spr_Wall_Ash;
        var _spriteFloor = Spr_Floor_Ash;
        }break;
    #endregion
   
   
    #region 1 Plains of Existence - Stone Room
    case 1: //Stone room
        {
        var _roomx1 = 1;
        var _roomy1 = 2;
        var _roomx2 = roomWidth-2;
        var _roomy2 = roomHeight-2;
       
        repeat(2)
            {
            var _firstLoop = true;
       
            repeat(2)
                {
                var _xx = roomWidth div 2;
                var _yy = roomHeight div 2;
                var _break = 250;
                var _rooms = irandom_range(4,6);
                var _roomscurrent = 0;
       
                while(_break > 0 && _roomscurrent < _rooms)
                    {
                    _break--;
               
                    if(/*(*/_grid[# _xx, _yy] == FREE /*&& ((_grid[# _xx-1, _yy] == BLOCKED
                        or _grid[# _xx, _yy-1] == BLOCKED or _grid[# _xx+1, _yy] == BLOCKED
                        or _grid[# _xx, _yy+1] == BLOCKED) or !_firstLoop))*/ or (_roomscurrent == 0 && _firstLoop))
                        {
                        _roomscurrent++;
                        if(!_firstLoop)
                            {
                            var x1 = max(_xx - irandom_range(0,1),3);
                            var x2 = min(_xx + irandom_range(0,1),roomWidth-4);
                            var y1 = max(_yy - irandom_range(0,1),4);
                            var y2 = min(_yy + irandom_range(0,1),roomHeight-4);
                            ds_grid_set_region(_grid,x1-2,y1-2,x2+2,y2+2,FREE);
                            ds_grid_set_region(_grid,x1,y1,x2,y2,BLOCKED); 
                            }
                        else
                            {
                            var x1 = max(_xx - irandom_range(4,7),1);
                            var x2 = min(_xx + irandom_range(4,7),roomWidth-2);
                            var y1 = max(_yy - irandom_range(2,5),2);
                            var y2 = min(_yy + irandom_range(2,5),roomHeight-2);
                            ds_grid_set_region(_grid,x1,y1,x2,y2,FREE);
                            }
                        }
                    _xx = irandom_range(_roomx1+1, _roomx2-1);
                    _yy = irandom_range(_roomy1+1, _roomy2-1);
                    }
                _firstLoop = false;
                }
            }
       
        var _spriteWall = Spr_Wall_Stone;
        var _spriteFloor = Spr_Floor_Stone_Tile;
        }break;
    #endregion
   
   
    #region 5/6 Plains of Existence - Treasure Room/Store
    case 5: case 6:
        {
        var _roomx1 = 5;
        var _roomy1 = 3;
        var _roomx2 = roomWidth-6;
        var _roomy2 = roomHeight-4;
       
        ds_grid_set_region(_grid,_roomx1,_roomy1,_roomx2,_roomy2,FREE);
        _grid[# _roomx1,_roomy1] = BLOCKED;
        _grid[# _roomx2,_roomy1] = BLOCKED;
        _grid[# _roomx1,_roomy2] = BLOCKED;
        _grid[# _roomx2,_roomy2] = BLOCKED;
       
        if(_type == 5)
            {
            _listDecals = ds_list_create();
            ds_list_add(_listDecals,Spr_Treasure_Altar,0,(roomWidth div 2 - 2) * 16,(roomHeight div 2 - 2) * 16);
       
            ds_list_add(_listDecals,Spr_Prop_Shadows,1,(roomWidth div 2 - 1) * 16,(roomHeight div 2 - 1) * 16);
            }
        var _spriteWall = Spr_Wall_Stone;
        var _spriteFloor = Spr_Floor_Stone_Tile;
        }break;
    #endregion
   
    #endregion Plains of Existence
    }
#endregion generate room structure
