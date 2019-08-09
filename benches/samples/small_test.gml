do {
    
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
