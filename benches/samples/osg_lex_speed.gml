/*

OSG Metroidvania by Yosi!!!

*/
//Setup - Only happens 1 time!!!
#region Macros and Enums
#macro _debug true
enum Function{
	Default,
	Load,
	Display,
	World_Map,
}
//Level Constants
#macro level_width 640
#macro level_height 480
#macro size 32
//State Machine Enums
enum Player_State{
	normal,
	dead,
}
global.p_state=Player_State.normal;
enum Game_State{
	paused,
	level_start,
	level_change,
	level_end,
	start,
	main,
	stop,
	debug,
	load,
	next_level,
}
#endregion
#region Pseudo-Class
var arg=argument_count>0?argument[0]:Function.Default;
switch(arg)
	{
	default:
		#region MAIN
			#region Init
			if (!solid)
				{
				solid=true;
				#region Game Variables
				//Level Constants
				global.p_state=Player_State.normal;
				global.g_state=Game_State.load;
				//Create the drawing surface
				global.surf=surface_create(level_width,level_height);
				global.previous_level=surface_create(level_width,level_height);
				//Level ds_grid
				global.grid=ds_grid_create(level_width div size,level_height div size);
				//Player Vars
				global.player_x=0;
				global.player_y=0;
				global.player_hsp=0;
				global.player_vsp=0;
				global.wrap_player=false;	
				#endregion
				#region Level Data
				//Level global.grid x and y positions for world
				global.lg_x=0;
				global.lg_y=0;
				//Individual Level Strings
				global.level[0]="111111111111111111111000000000000000000110000000000000000001100000000000000000011000000000000000000110000000000000000001100000000000000000001000000000000000000010000000000000000000100000000000000111111000020000001001111110000000001110011111100000001111000111111011110000000001111111111111111111111111";
				global.level[1]="111111111111111111111000000000000000000110000000000000000001100000000000000000011000000000000000000110000000000000000001000000000000000000000000000000000000000000000000000000000000100000000000000000011000000000000000000110000000002000000001111110001111100111111110000001110000011110000000011100000001";
				global.level[2]="100000000111000000011000000001110000000110000000011100000001100000000111000000011000000001110000000110000000011100000001000000000111000000000000000001110000000000000000011100000000100002000111000000011001111111111111100110111111111111111101111111111111111111111111111111111111111111111111111111111111";
				global.level[3]="111111111000111111111000000010000000000110000000110000000001100000000001100000011000000000000011000110000000000000000001000000000000000001110000000000000001100100000000001111000001100000000110000000011000020011100000000111100001111000000001111000111110000000011110001111100000000111111111111111111111";
				global.level[4]="110000000011111111111100000000001111111111000000000011111111110000000000000111111100001110000000000111001111111000000001110011111110000000001100010000000000000011000000000011100000111100020000111011111111110000111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
				global.level[5]="111111111111111111111111110000000000011111000000000000000011100000000000000000011000000000000000000110000000000000000001000001111111110000010000011111111100000100000111000000000001100001100000000000011100011000000000001111001110000000000011111111100000000001111111111000000002011111111111100011111111";
				global.level[6]="111111111111111111111110000000000000011111000000000000000011100000000000000000011000000000000000000110000000000000000001100000000000000000001000000000000000000010000000000000000000100000000000000000011100000000000000001111000000000000000011110000000000002001111100000000011111111111000000001111111111";
				global.level[7]="111111111111111111111111111111111111111110111011111111011101101100011100110111011001000110001100110110000000100001001101000000001000000011010000000010000000110100000100000001001001100211000000010000011000110000000100000110011100010001100001100111001100111101111111110111111111111111111111111111111111";
				global.level_grid=ds_grid_create(3,3);
				global.level_grid[# 0,0]=0;
				global.level_grid[# 0,1]=6;
				global.level_grid[# 0,2]=4;
				global.level_grid[# 1,0]=1;
				global.level_grid[# 1,1]=2;
				global.level_grid[# 1,2]=7;
				global.level_grid[# 2,0]=5;
				global.level_grid[# 2,1]=3;
				global.level_grid[# 2,2]=0;
	
				global.current_level=global.level_grid[# global.lg_x,global.lg_y];
				#endregion
				if (_debug) show_debug_message("Init Complete");
				}
			#endregion
			#region Game States
			switch(global.g_state)
				{
				case Game_State.load:
					{
					osg_metroidvania(Function.Load);
					break;
					}
				case Game_State.main:
					{
					osg_metroidvania(Function.Display);
					#region Player
						#region State Machine
						switch(global.p_state)
							{
							case Player_State.normal:
								{
								//Input
								var rl,ud,jump;
								rl=sign(keyboard_check(vk_right)-keyboard_check(vk_left));
								ud=sign(keyboard_check(vk_down)-keyboard_check(vk_up));
								jump=keyboard_check_pressed(vk_up);
								//Friction
								//On ground
								show_debug_message(string(floor(global.player_x/size)) + "," + string(ceil(global.player_x/size)));
								if  (global.grid[# floor(global.player_x/size),clamp((global.player_y+size) div size,0,ds_grid_height(global.grid)-1)]!="0") ||
									(global.grid[# ceil(global.player_x/size),clamp((global.player_y+size) div size,0,ds_grid_height(global.grid)-1)]!="0")
									{
									if (rl==0)
										global.player_hsp=(global.player_hsp>0) ? max(global.player_hsp-1,0) : min(global.player_hsp+1,0);
									global.player_hsp=clamp(global.player_hsp+rl*3,-5,5);
									//Jumping
									global.player_vsp=(jump) ? -11 : global.player_vsp;
									}
								else
									{
									if (rl==0)
										global.player_hsp=(global.player_hsp>0) ? max(global.player_hsp-0.1,0) : min(global.player_hsp+0.1,0);
									global.player_hsp=clamp(global.player_hsp+rl/2,-5,5);
									}
								//Gravity
								global.player_vsp=min(global.player_vsp+0.5,10);
								break;
								}
							}
						#endregion
						#region Collision Checking
						var s=size;
						repeat(abs(global.player_hsp))
							{
							if (global.grid[# clamp((global.player_x+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp(global.player_y div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(((global.player_x+s-1)+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+s-1) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+s-1) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(((global.player_x+s-1)+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp(global.player_y div s,0,ds_grid_height(global.grid)-1)]=="0")
								{
								global.player_x+=sign(global.player_hsp);
								}
							else
								{
								global.player_hsp=0;
								}
							}
						repeat(abs(global.player_vsp))
							{
							if (global.grid[# clamp(global.player_x div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+s-1) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(global.player_x div s,0,ds_grid_width(global.grid)-1),clamp(((global.player_y+s-1)+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+s-1) div s,0,ds_grid_width(global.grid)-1),clamp(((global.player_y+s-1)+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0")
								{
								global.player_y+=sign(global.player_vsp);
								}
							else
								global.player_vsp=0;
							}
						#endregion
						#region Out of Bounds
						//If you are out of bounds...
						if (global.player_x>level_width-size || global.player_x<0 || global.player_y>level_height-size || global.player_y<0)
							{
							//If there is another room to move to, move!
							var gx=global.lg_x,gy=global.lg_y;
							if (global.player_x>=level_width-size)
								gx+=1;
							else if (global.player_x<=0)
								gx-=1;
							else if (global.player_y>=level_height-size)
								gy+=1;
							else if (global.player_y<=0)
								gy-=1;
							if (gx>=0 && gy>=0 && gx<ds_grid_width(global.level_grid) && gy<ds_grid_height(global.level_grid))
								{
								if (global.level_grid[# gx,gy]>=0)
									{
									if (_debug) show_debug_message("Next Room!");
									//Go to the next room over
									global.current_level=global.level_grid[# gx,gy];
									global.lg_x=gx;
									global.lg_y=gy;
									global.wrap_player=true;
									global.g_state=Game_State.load;
									}
								else
									{
									if (_debug) show_debug_message("Restarted!");
									//Restart Room
									global.wrap_player=true;
									global.g_state=Game_State.load;
									}
								}
							else 
								{
								if (_debug) show_debug_message("Restarted!");
								//Restart Room
								global.wrap_player=false;
								global.g_state=Game_State.load;
								}
							}
						#endregion
						#region Draw
						//Draw Player
						draw_rectangle_color(global.player_x,global.player_y,global.player_x+size-1,global.player_y+size-1,c_black,c_black,c_white,c_white,false);
						#endregion
					#endregion
					break;
					}
				default: show_debug_message("Game State Machine is broken!"); break;
				}
			#endregion
			osg_metroidvania(Function.World_Map);
		#endregion
		break;
	case Function.Load:
		#region Load the level into a global.grid
		ds_grid_clear(global.grid,0);
		surface_set_target(global.surf);
		//Draw the level
		var c=c_black;
		var str=global.level[@global.current_level];
		var pos=0;
		for(var i=0;i<(level_width div size);i++)
			{
			for(var m=0;m<(level_height div size);m++)
				{
				pos=i+(m*(level_width div size))+1;
				//Add to global.grid
				ds_grid_add(global.grid,i,m,string_char_at(str,pos));
				//Draw something different depending on what's in the string
				switch(string_char_at(str,pos))
					{
					case "0": 
						{
						c=c_white;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "1":
						{
						c=c_black;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "2":
						{
						//If it is "2", spawn the player and change it to be a "0"
						c=c_white;
						//Absolute player coordinates
						if (!global.wrap_player)
							{
							global.player_x=(i*size)-1;
							global.player_y=(m*size)-1;
							}
						global.grid[# i,m]="0";
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "3":
						{
						c=c_lime;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					default: break;
					}
				}
			}
			//Set the player position to loop the screen, if the player is being wrapped
			if (global.wrap_player)
				{
				if (global.player_x<=0)
					global.player_x=level_width-size-1;
				else if (global.player_x>=level_width-size)
					global.player_x=1;
				else if (global.player_y<=0)
					global.player_y=level_height-size;
				else if (global.player_y>=level_height-size-1)
					global.player_y=1;
				}
			surface_reset_target();
			global.g_state=Game_State.main;
			osg_metroidvania(Function.Display);
		#endregion
		break;
	case Function.Display:
		#region Draw the Game Surface
		draw_surface(global.surf,0,0);
		#endregion
		break;
	case Function.World_Map:
		#region Draw the World Map overlay
		for(var i=0;i<ds_grid_width(global.level_grid);i++)
			{
			for(var m=0;m<ds_grid_height(global.level_grid);m++)
				{
				var c=c_red;
				if (global.lg_x==i && global.lg_y==m)
					{
					c=c_lime;
					draw_set_alpha(0.4);
					draw_rectangle_color(i*size+1,m*size+1,i*size+size-2,m*size+size-2,c,c,c,c,false);
					}
				else
					{
					draw_set_alpha(0.4);
					draw_rectangle_color(i*size+1,m*size+1,i*size+size-2,m*size+size-2,c,c,c,c,false);
					}
				draw_set_alpha(1);
				}
			}
		#endregion
		break;
	}
#endregion
/*END*/
/*

OSG Metroidvania by Yosi!!!

*/
//Setup - Only happens 1 time!!!
#region Macros and Enums
#macro _debug true
enum Function{
	Default,
	Load,
	Display,
	World_Map,
}
//Level Constants
#macro level_width 640
#macro level_height 480
#macro size 32
//State Machine Enums
enum Player_State{
	normal,
	dead,
}
global.p_state=Player_State.normal;
enum Game_State{
	paused,
	level_start,
	level_change,
	level_end,
	start,
	main,
	stop,
	debug,
	load,
	next_level,
}
#endregion
#region Pseudo-Class
var arg=argument_count>0?argument[0]:Function.Default;
switch(arg)
	{
	default:
		#region MAIN
			#region Init
			if (!solid)
				{
				solid=true;
				#region Game Variables
				//Level Constants
				global.p_state=Player_State.normal;
				global.g_state=Game_State.load;
				//Create the drawing surface
				global.surf=surface_create(level_width,level_height);
				global.previous_level=surface_create(level_width,level_height);
				//Level ds_grid
				global.grid=ds_grid_create(level_width div size,level_height div size);
				//Player Vars
				global.player_x=0;
				global.player_y=0;
				global.player_hsp=0;
				global.player_vsp=0;
				global.wrap_player=false;	
				#endregion
				#region Level Data
				//Level global.grid x and y positions for world
				global.lg_x=0;
				global.lg_y=0;
				//Individual Level Strings
				global.level[0]="111111111111111111111000000000000000000110000000000000000001100000000000000000011000000000000000000110000000000000000001100000000000000000001000000000000000000010000000000000000000100000000000000111111000020000001001111110000000001110011111100000001111000111111011110000000001111111111111111111111111";
				global.level[1]="111111111111111111111000000000000000000110000000000000000001100000000000000000011000000000000000000110000000000000000001000000000000000000000000000000000000000000000000000000000000100000000000000000011000000000000000000110000000002000000001111110001111100111111110000001110000011110000000011100000001";
				global.level[2]="100000000111000000011000000001110000000110000000011100000001100000000111000000011000000001110000000110000000011100000001000000000111000000000000000001110000000000000000011100000000100002000111000000011001111111111111100110111111111111111101111111111111111111111111111111111111111111111111111111111111";
				global.level[3]="111111111000111111111000000010000000000110000000110000000001100000000001100000011000000000000011000110000000000000000001000000000000000001110000000000000001100100000000001111000001100000000110000000011000020011100000000111100001111000000001111000111110000000011110001111100000000111111111111111111111";
				global.level[4]="110000000011111111111100000000001111111111000000000011111111110000000000000111111100001110000000000111001111111000000001110011111110000000001100010000000000000011000000000011100000111100020000111011111111110000111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
				global.level[5]="111111111111111111111111110000000000011111000000000000000011100000000000000000011000000000000000000110000000000000000001000001111111110000010000011111111100000100000111000000000001100001100000000000011100011000000000001111001110000000000011111111100000000001111111111000000002011111111111100011111111";
				global.level[6]="111111111111111111111110000000000000011111000000000000000011100000000000000000011000000000000000000110000000000000000001100000000000000000001000000000000000000010000000000000000000100000000000000000011100000000000000001111000000000000000011110000000000002001111100000000011111111111000000001111111111";
				global.level[7]="111111111111111111111111111111111111111110111011111111011101101100011100110111011001000110001100110110000000100001001101000000001000000011010000000010000000110100000100000001001001100211000000010000011000110000000100000110011100010001100001100111001100111101111111110111111111111111111111111111111111";
				global.level_grid=ds_grid_create(3,3);
				global.level_grid[# 0,0]=0;
				global.level_grid[# 0,1]=6;
				global.level_grid[# 0,2]=4;
				global.level_grid[# 1,0]=1;
				global.level_grid[# 1,1]=2;
				global.level_grid[# 1,2]=7;
				global.level_grid[# 2,0]=5;
				global.level_grid[# 2,1]=3;
				global.level_grid[# 2,2]=0;
	
				global.current_level=global.level_grid[# global.lg_x,global.lg_y];
				#endregion
				if (_debug) show_debug_message("Init Complete");
				}
			#endregion
			#region Game States
			switch(global.g_state)
				{
				case Game_State.load:
					{
					osg_metroidvania(Function.Load);
					break;
					}
				case Game_State.main:
					{
					osg_metroidvania(Function.Display);
					#region Player
						#region State Machine
						switch(global.p_state)
							{
							case Player_State.normal:
								{
								//Input
								var rl,ud,jump;
								rl=sign(keyboard_check(vk_right)-keyboard_check(vk_left));
								ud=sign(keyboard_check(vk_down)-keyboard_check(vk_up));
								jump=keyboard_check_pressed(vk_up);
								//Friction
								//On ground
								show_debug_message(string(floor(global.player_x/size)) + "," + string(ceil(global.player_x/size)));
								if  (global.grid[# floor(global.player_x/size),clamp((global.player_y+size) div size,0,ds_grid_height(global.grid)-1)]!="0") ||
									(global.grid[# ceil(global.player_x/size),clamp((global.player_y+size) div size,0,ds_grid_height(global.grid)-1)]!="0")
									{
									if (rl==0)
										global.player_hsp=(global.player_hsp>0) ? max(global.player_hsp-1,0) : min(global.player_hsp+1,0);
									global.player_hsp=clamp(global.player_hsp+rl*3,-5,5);
									//Jumping
									global.player_vsp=(jump) ? -11 : global.player_vsp;
									}
								else
									{
									if (rl==0)
										global.player_hsp=(global.player_hsp>0) ? max(global.player_hsp-0.1,0) : min(global.player_hsp+0.1,0);
									global.player_hsp=clamp(global.player_hsp+rl/2,-5,5);
									}
								//Gravity
								global.player_vsp=min(global.player_vsp+0.5,10);
								break;
								}
							}
						#endregion
						#region Collision Checking
						var s=size;
						repeat(abs(global.player_hsp))
							{
							if (global.grid[# clamp((global.player_x+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp(global.player_y div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(((global.player_x+s-1)+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+s-1) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+s-1) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(((global.player_x+s-1)+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp(global.player_y div s,0,ds_grid_height(global.grid)-1)]=="0")
								{
								global.player_x+=sign(global.player_hsp);
								}
							else
								{
								global.player_hsp=0;
								}
							}
						repeat(abs(global.player_vsp))
							{
							if (global.grid[# clamp(global.player_x div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+s-1) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(global.player_x div s,0,ds_grid_width(global.grid)-1),clamp(((global.player_y+s-1)+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+s-1) div s,0,ds_grid_width(global.grid)-1),clamp(((global.player_y+s-1)+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0")
								{
								global.player_y+=sign(global.player_vsp);
								}
							else
								global.player_vsp=0;
							}
						#endregion
						#region Out of Bounds
						//If you are out of bounds...
						if (global.player_x>level_width-size || global.player_x<0 || global.player_y>level_height-size || global.player_y<0)
							{
							//If there is another room to move to, move!
							var gx=global.lg_x,gy=global.lg_y;
							if (global.player_x>=level_width-size)
								gx+=1;
							else if (global.player_x<=0)
								gx-=1;
							else if (global.player_y>=level_height-size)
								gy+=1;
							else if (global.player_y<=0)
								gy-=1;
							if (gx>=0 && gy>=0 && gx<ds_grid_width(global.level_grid) && gy<ds_grid_height(global.level_grid))
								{
								if (global.level_grid[# gx,gy]>=0)
									{
									if (_debug) show_debug_message("Next Room!");
									//Go to the next room over
									global.current_level=global.level_grid[# gx,gy];
									global.lg_x=gx;
									global.lg_y=gy;
									global.wrap_player=true;
									global.g_state=Game_State.load;
									}
								else
									{
									if (_debug) show_debug_message("Restarted!");
									//Restart Room
									global.wrap_player=true;
									global.g_state=Game_State.load;
									}
								}
							else 
								{
								if (_debug) show_debug_message("Restarted!");
								//Restart Room
								global.wrap_player=false;
								global.g_state=Game_State.load;
								}
							}
						#endregion
						#region Draw
						//Draw Player
						draw_rectangle_color(global.player_x,global.player_y,global.player_x+size-1,global.player_y+size-1,c_black,c_black,c_white,c_white,false);
						#endregion
					#endregion
					break;
					}
				default: show_debug_message("Game State Machine is broken!"); break;
				}
			#endregion
			osg_metroidvania(Function.World_Map);
		#endregion
		break;
	case Function.Load:
		#region Load the level into a global.grid
		ds_grid_clear(global.grid,0);
		surface_set_target(global.surf);
		//Draw the level
		var c=c_black;
		var str=global.level[@global.current_level];
		var pos=0;
		for(var i=0;i<(level_width div size);i++)
			{
			for(var m=0;m<(level_height div size);m++)
				{
				pos=i+(m*(level_width div size))+1;
				//Add to global.grid
				ds_grid_add(global.grid,i,m,string_char_at(str,pos));
				//Draw something different depending on what's in the string
				switch(string_char_at(str,pos))
					{
					case "0": 
						{
						c=c_white;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "1":
						{
						c=c_black;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "2":
						{
						//If it is "2", spawn the player and change it to be a "0"
						c=c_white;
						//Absolute player coordinates
						if (!global.wrap_player)
							{
							global.player_x=(i*size)-1;
							global.player_y=(m*size)-1;
							}
						global.grid[# i,m]="0";
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "3":
						{
						c=c_lime;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					default: break;
					}
				}
			}
			//Set the player position to loop the screen, if the player is being wrapped
			if (global.wrap_player)
				{
				if (global.player_x<=0)
					global.player_x=level_width-size-1;
				else if (global.player_x>=level_width-size)
					global.player_x=1;
				else if (global.player_y<=0)
					global.player_y=level_height-size;
				else if (global.player_y>=level_height-size-1)
					global.player_y=1;
				}
			surface_reset_target();
			global.g_state=Game_State.main;
			osg_metroidvania(Function.Display);
		#endregion
		break;
	case Function.Display:
		#region Draw the Game Surface
		draw_surface(global.surf,0,0);
		#endregion
		break;
	case Function.World_Map:
		#region Draw the World Map overlay
		for(var i=0;i<ds_grid_width(global.level_grid);i++)
			{
			for(var m=0;m<ds_grid_height(global.level_grid);m++)
				{
				var c=c_red;
				if (global.lg_x==i && global.lg_y==m)
					{
					c=c_lime;
					draw_set_alpha(0.4);
					draw_rectangle_color(i*size+1,m*size+1,i*size+size-2,m*size+size-2,c,c,c,c,false);
					}
				else
					{
					draw_set_alpha(0.4);
					draw_rectangle_color(i*size+1,m*size+1,i*size+size-2,m*size+size-2,c,c,c,c,false);
					}
				draw_set_alpha(1);
				}
			}
		#endregion
		break;
	}
#endregion
/*END*/
/*

OSG Metroidvania by Yosi!!!

*/
//Setup - Only happens 1 time!!!
#region Macros and Enums
#macro _debug true
enum Function{
	Default,
	Load,
	Display,
	World_Map,
}
//Level Constants
#macro level_width 640
#macro level_height 480
#macro size 32
//State Machine Enums
enum Player_State{
	normal,
	dead,
}
global.p_state=Player_State.normal;
enum Game_State{
	paused,
	level_start,
	level_change,
	level_end,
	start,
	main,
	stop,
	debug,
	load,
	next_level,
}
#endregion
#region Pseudo-Class
var arg=argument_count>0?argument[0]:Function.Default;
switch(arg)
	{
	default:
		#region MAIN
			#region Init
			if (!solid)
				{
				solid=true;
				#region Game Variables
				//Level Constants
				global.p_state=Player_State.normal;
				global.g_state=Game_State.load;
				//Create the drawing surface
				global.surf=surface_create(level_width,level_height);
				global.previous_level=surface_create(level_width,level_height);
				//Level ds_grid
				global.grid=ds_grid_create(level_width div size,level_height div size);
				//Player Vars
				global.player_x=0;
				global.player_y=0;
				global.player_hsp=0;
				global.player_vsp=0;
				global.wrap_player=false;	
				#endregion
				#region Level Data
				//Level global.grid x and y positions for world
				global.lg_x=0;
				global.lg_y=0;
				//Individual Level Strings
				global.level[0]="111111111111111111111000000000000000000110000000000000000001100000000000000000011000000000000000000110000000000000000001100000000000000000001000000000000000000010000000000000000000100000000000000111111000020000001001111110000000001110011111100000001111000111111011110000000001111111111111111111111111";
				global.level[1]="111111111111111111111000000000000000000110000000000000000001100000000000000000011000000000000000000110000000000000000001000000000000000000000000000000000000000000000000000000000000100000000000000000011000000000000000000110000000002000000001111110001111100111111110000001110000011110000000011100000001";
				global.level[2]="100000000111000000011000000001110000000110000000011100000001100000000111000000011000000001110000000110000000011100000001000000000111000000000000000001110000000000000000011100000000100002000111000000011001111111111111100110111111111111111101111111111111111111111111111111111111111111111111111111111111";
				global.level[3]="111111111000111111111000000010000000000110000000110000000001100000000001100000011000000000000011000110000000000000000001000000000000000001110000000000000001100100000000001111000001100000000110000000011000020011100000000111100001111000000001111000111110000000011110001111100000000111111111111111111111";
				global.level[4]="110000000011111111111100000000001111111111000000000011111111110000000000000111111100001110000000000111001111111000000001110011111110000000001100010000000000000011000000000011100000111100020000111011111111110000111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
				global.level[5]="111111111111111111111111110000000000011111000000000000000011100000000000000000011000000000000000000110000000000000000001000001111111110000010000011111111100000100000111000000000001100001100000000000011100011000000000001111001110000000000011111111100000000001111111111000000002011111111111100011111111";
				global.level[6]="111111111111111111111110000000000000011111000000000000000011100000000000000000011000000000000000000110000000000000000001100000000000000000001000000000000000000010000000000000000000100000000000000000011100000000000000001111000000000000000011110000000000002001111100000000011111111111000000001111111111";
				global.level[7]="111111111111111111111111111111111111111110111011111111011101101100011100110111011001000110001100110110000000100001001101000000001000000011010000000010000000110100000100000001001001100211000000010000011000110000000100000110011100010001100001100111001100111101111111110111111111111111111111111111111111";
				global.level_grid=ds_grid_create(3,3);
				global.level_grid[# 0,0]=0;
				global.level_grid[# 0,1]=6;
				global.level_grid[# 0,2]=4;
				global.level_grid[# 1,0]=1;
				global.level_grid[# 1,1]=2;
				global.level_grid[# 1,2]=7;
				global.level_grid[# 2,0]=5;
				global.level_grid[# 2,1]=3;
				global.level_grid[# 2,2]=0;
	
				global.current_level=global.level_grid[# global.lg_x,global.lg_y];
				#endregion
				if (_debug) show_debug_message("Init Complete");
				}
			#endregion
			#region Game States
			switch(global.g_state)
				{
				case Game_State.load:
					{
					osg_metroidvania(Function.Load);
					break;
					}
				case Game_State.main:
					{
					osg_metroidvania(Function.Display);
					#region Player
						#region State Machine
						switch(global.p_state)
							{
							case Player_State.normal:
								{
								//Input
								var rl,ud,jump;
								rl=sign(keyboard_check(vk_right)-keyboard_check(vk_left));
								ud=sign(keyboard_check(vk_down)-keyboard_check(vk_up));
								jump=keyboard_check_pressed(vk_up);
								//Friction
								//On ground
								show_debug_message(string(floor(global.player_x/size)) + "," + string(ceil(global.player_x/size)));
								if  (global.grid[# floor(global.player_x/size),clamp((global.player_y+size) div size,0,ds_grid_height(global.grid)-1)]!="0") ||
									(global.grid[# ceil(global.player_x/size),clamp((global.player_y+size) div size,0,ds_grid_height(global.grid)-1)]!="0")
									{
									if (rl==0)
										global.player_hsp=(global.player_hsp>0) ? max(global.player_hsp-1,0) : min(global.player_hsp+1,0);
									global.player_hsp=clamp(global.player_hsp+rl*3,-5,5);
									//Jumping
									global.player_vsp=(jump) ? -11 : global.player_vsp;
									}
								else
									{
									if (rl==0)
										global.player_hsp=(global.player_hsp>0) ? max(global.player_hsp-0.1,0) : min(global.player_hsp+0.1,0);
									global.player_hsp=clamp(global.player_hsp+rl/2,-5,5);
									}
								//Gravity
								global.player_vsp=min(global.player_vsp+0.5,10);
								break;
								}
							}
						#endregion
						#region Collision Checking
						var s=size;
						repeat(abs(global.player_hsp))
							{
							if (global.grid[# clamp((global.player_x+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp(global.player_y div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(((global.player_x+s-1)+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+s-1) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+s-1) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(((global.player_x+s-1)+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp(global.player_y div s,0,ds_grid_height(global.grid)-1)]=="0")
								{
								global.player_x+=sign(global.player_hsp);
								}
							else
								{
								global.player_hsp=0;
								}
							}
						repeat(abs(global.player_vsp))
							{
							if (global.grid[# clamp(global.player_x div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+s-1) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(global.player_x div s,0,ds_grid_width(global.grid)-1),clamp(((global.player_y+s-1)+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+s-1) div s,0,ds_grid_width(global.grid)-1),clamp(((global.player_y+s-1)+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0")
								{
								global.player_y+=sign(global.player_vsp);
								}
							else
								global.player_vsp=0;
							}
						#endregion
						#region Out of Bounds
						//If you are out of bounds...
						if (global.player_x>level_width-size || global.player_x<0 || global.player_y>level_height-size || global.player_y<0)
							{
							//If there is another room to move to, move!
							var gx=global.lg_x,gy=global.lg_y;
							if (global.player_x>=level_width-size)
								gx+=1;
							else if (global.player_x<=0)
								gx-=1;
							else if (global.player_y>=level_height-size)
								gy+=1;
							else if (global.player_y<=0)
								gy-=1;
							if (gx>=0 && gy>=0 && gx<ds_grid_width(global.level_grid) && gy<ds_grid_height(global.level_grid))
								{
								if (global.level_grid[# gx,gy]>=0)
									{
									if (_debug) show_debug_message("Next Room!");
									//Go to the next room over
									global.current_level=global.level_grid[# gx,gy];
									global.lg_x=gx;
									global.lg_y=gy;
									global.wrap_player=true;
									global.g_state=Game_State.load;
									}
								else
									{
									if (_debug) show_debug_message("Restarted!");
									//Restart Room
									global.wrap_player=true;
									global.g_state=Game_State.load;
									}
								}
							else 
								{
								if (_debug) show_debug_message("Restarted!");
								//Restart Room
								global.wrap_player=false;
								global.g_state=Game_State.load;
								}
							}
						#endregion
						#region Draw
						//Draw Player
						draw_rectangle_color(global.player_x,global.player_y,global.player_x+size-1,global.player_y+size-1,c_black,c_black,c_white,c_white,false);
						#endregion
					#endregion
					break;
					}
				default: show_debug_message("Game State Machine is broken!"); break;
				}
			#endregion
			osg_metroidvania(Function.World_Map);
		#endregion
		break;
	case Function.Load:
		#region Load the level into a global.grid
		ds_grid_clear(global.grid,0);
		surface_set_target(global.surf);
		//Draw the level
		var c=c_black;
		var str=global.level[@global.current_level];
		var pos=0;
		for(var i=0;i<(level_width div size);i++)
			{
			for(var m=0;m<(level_height div size);m++)
				{
				pos=i+(m*(level_width div size))+1;
				//Add to global.grid
				ds_grid_add(global.grid,i,m,string_char_at(str,pos));
				//Draw something different depending on what's in the string
				switch(string_char_at(str,pos))
					{
					case "0": 
						{
						c=c_white;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "1":
						{
						c=c_black;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "2":
						{
						//If it is "2", spawn the player and change it to be a "0"
						c=c_white;
						//Absolute player coordinates
						if (!global.wrap_player)
							{
							global.player_x=(i*size)-1;
							global.player_y=(m*size)-1;
							}
						global.grid[# i,m]="0";
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "3":
						{
						c=c_lime;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					default: break;
					}
				}
			}
			//Set the player position to loop the screen, if the player is being wrapped
			if (global.wrap_player)
				{
				if (global.player_x<=0)
					global.player_x=level_width-size-1;
				else if (global.player_x>=level_width-size)
					global.player_x=1;
				else if (global.player_y<=0)
					global.player_y=level_height-size;
				else if (global.player_y>=level_height-size-1)
					global.player_y=1;
				}
			surface_reset_target();
			global.g_state=Game_State.main;
			osg_metroidvania(Function.Display);
		#endregion
		break;
	case Function.Display:
		#region Draw the Game Surface
		draw_surface(global.surf,0,0);
		#endregion
		break;
	case Function.World_Map:
		#region Draw the World Map overlay
		for(var i=0;i<ds_grid_width(global.level_grid);i++)
			{
			for(var m=0;m<ds_grid_height(global.level_grid);m++)
				{
				var c=c_red;
				if (global.lg_x==i && global.lg_y==m)
					{
					c=c_lime;
					draw_set_alpha(0.4);
					draw_rectangle_color(i*size+1,m*size+1,i*size+size-2,m*size+size-2,c,c,c,c,false);
					}
				else
					{
					draw_set_alpha(0.4);
					draw_rectangle_color(i*size+1,m*size+1,i*size+size-2,m*size+size-2,c,c,c,c,false);
					}
				draw_set_alpha(1);
				}
			}
		#endregion
		break;
	}
#endregion
/*END*/
/*

OSG Metroidvania by Yosi!!!

*/
//Setup - Only happens 1 time!!!
#region Macros and Enums
#macro _debug true
enum Function{
	Default,
	Load,
	Display,
	World_Map,
}
//Level Constants
#macro level_width 640
#macro level_height 480
#macro size 32
//State Machine Enums
enum Player_State{
	normal,
	dead,
}
global.p_state=Player_State.normal;
enum Game_State{
	paused,
	level_start,
	level_change,
	level_end,
	start,
	main,
	stop,
	debug,
	load,
	next_level,
}
#endregion
#region Pseudo-Class
var arg=argument_count>0?argument[0]:Function.Default;
switch(arg)
	{
	default:
		#region MAIN
			#region Init
			if (!solid)
				{
				solid=true;
				#region Game Variables
				//Level Constants
				global.p_state=Player_State.normal;
				global.g_state=Game_State.load;
				//Create the drawing surface
				global.surf=surface_create(level_width,level_height);
				global.previous_level=surface_create(level_width,level_height);
				//Level ds_grid
				global.grid=ds_grid_create(level_width div size,level_height div size);
				//Player Vars
				global.player_x=0;
				global.player_y=0;
				global.player_hsp=0;
				global.player_vsp=0;
				global.wrap_player=false;	
				#endregion
				#region Level Data
				//Level global.grid x and y positions for world
				global.lg_x=0;
				global.lg_y=0;
				//Individual Level Strings
				global.level[0]="111111111111111111111000000000000000000110000000000000000001100000000000000000011000000000000000000110000000000000000001100000000000000000001000000000000000000010000000000000000000100000000000000111111000020000001001111110000000001110011111100000001111000111111011110000000001111111111111111111111111";
				global.level[1]="111111111111111111111000000000000000000110000000000000000001100000000000000000011000000000000000000110000000000000000001000000000000000000000000000000000000000000000000000000000000100000000000000000011000000000000000000110000000002000000001111110001111100111111110000001110000011110000000011100000001";
				global.level[2]="100000000111000000011000000001110000000110000000011100000001100000000111000000011000000001110000000110000000011100000001000000000111000000000000000001110000000000000000011100000000100002000111000000011001111111111111100110111111111111111101111111111111111111111111111111111111111111111111111111111111";
				global.level[3]="111111111000111111111000000010000000000110000000110000000001100000000001100000011000000000000011000110000000000000000001000000000000000001110000000000000001100100000000001111000001100000000110000000011000020011100000000111100001111000000001111000111110000000011110001111100000000111111111111111111111";
				global.level[4]="110000000011111111111100000000001111111111000000000011111111110000000000000111111100001110000000000111001111111000000001110011111110000000001100010000000000000011000000000011100000111100020000111011111111110000111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
				global.level[5]="111111111111111111111111110000000000011111000000000000000011100000000000000000011000000000000000000110000000000000000001000001111111110000010000011111111100000100000111000000000001100001100000000000011100011000000000001111001110000000000011111111100000000001111111111000000002011111111111100011111111";
				global.level[6]="111111111111111111111110000000000000011111000000000000000011100000000000000000011000000000000000000110000000000000000001100000000000000000001000000000000000000010000000000000000000100000000000000000011100000000000000001111000000000000000011110000000000002001111100000000011111111111000000001111111111";
				global.level[7]="111111111111111111111111111111111111111110111011111111011101101100011100110111011001000110001100110110000000100001001101000000001000000011010000000010000000110100000100000001001001100211000000010000011000110000000100000110011100010001100001100111001100111101111111110111111111111111111111111111111111";
				global.level_grid=ds_grid_create(3,3);
				global.level_grid[# 0,0]=0;
				global.level_grid[# 0,1]=6;
				global.level_grid[# 0,2]=4;
				global.level_grid[# 1,0]=1;
				global.level_grid[# 1,1]=2;
				global.level_grid[# 1,2]=7;
				global.level_grid[# 2,0]=5;
				global.level_grid[# 2,1]=3;
				global.level_grid[# 2,2]=0;
	
				global.current_level=global.level_grid[# global.lg_x,global.lg_y];
				#endregion
				if (_debug) show_debug_message("Init Complete");
				}
			#endregion
			#region Game States
			switch(global.g_state)
				{
				case Game_State.load:
					{
					osg_metroidvania(Function.Load);
					break;
					}
				case Game_State.main:
					{
					osg_metroidvania(Function.Display);
					#region Player
						#region State Machine
						switch(global.p_state)
							{
							case Player_State.normal:
								{
								//Input
								var rl,ud,jump;
								rl=sign(keyboard_check(vk_right)-keyboard_check(vk_left));
								ud=sign(keyboard_check(vk_down)-keyboard_check(vk_up));
								jump=keyboard_check_pressed(vk_up);
								//Friction
								//On ground
								show_debug_message(string(floor(global.player_x/size)) + "," + string(ceil(global.player_x/size)));
								if  (global.grid[# floor(global.player_x/size),clamp((global.player_y+size) div size,0,ds_grid_height(global.grid)-1)]!="0") ||
									(global.grid[# ceil(global.player_x/size),clamp((global.player_y+size) div size,0,ds_grid_height(global.grid)-1)]!="0")
									{
									if (rl==0)
										global.player_hsp=(global.player_hsp>0) ? max(global.player_hsp-1,0) : min(global.player_hsp+1,0);
									global.player_hsp=clamp(global.player_hsp+rl*3,-5,5);
									//Jumping
									global.player_vsp=(jump) ? -11 : global.player_vsp;
									}
								else
									{
									if (rl==0)
										global.player_hsp=(global.player_hsp>0) ? max(global.player_hsp-0.1,0) : min(global.player_hsp+0.1,0);
									global.player_hsp=clamp(global.player_hsp+rl/2,-5,5);
									}
								//Gravity
								global.player_vsp=min(global.player_vsp+0.5,10);
								break;
								}
							}
						#endregion
						#region Collision Checking
						var s=size;
						repeat(abs(global.player_hsp))
							{
							if (global.grid[# clamp((global.player_x+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp(global.player_y div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(((global.player_x+s-1)+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+s-1) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+s-1) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(((global.player_x+s-1)+sign(global.player_hsp)) div s,0,ds_grid_width(global.grid)-1),clamp(global.player_y div s,0,ds_grid_height(global.grid)-1)]=="0")
								{
								global.player_x+=sign(global.player_hsp);
								}
							else
								{
								global.player_hsp=0;
								}
							}
						repeat(abs(global.player_vsp))
							{
							if (global.grid[# clamp(global.player_x div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+s-1) div s,0,ds_grid_width(global.grid)-1),clamp((global.player_y+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp(global.player_x div s,0,ds_grid_width(global.grid)-1),clamp(((global.player_y+s-1)+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0") &&
								(global.grid[# clamp((global.player_x+s-1) div s,0,ds_grid_width(global.grid)-1),clamp(((global.player_y+s-1)+sign(global.player_vsp)) div s,0,ds_grid_height(global.grid)-1)]=="0")
								{
								global.player_y+=sign(global.player_vsp);
								}
							else
								global.player_vsp=0;
							}
						#endregion
						#region Out of Bounds
						//If you are out of bounds...
						if (global.player_x>level_width-size || global.player_x<0 || global.player_y>level_height-size || global.player_y<0)
							{
							//If there is another room to move to, move!
							var gx=global.lg_x,gy=global.lg_y;
							if (global.player_x>=level_width-size)
								gx+=1;
							else if (global.player_x<=0)
								gx-=1;
							else if (global.player_y>=level_height-size)
								gy+=1;
							else if (global.player_y<=0)
								gy-=1;
							if (gx>=0 && gy>=0 && gx<ds_grid_width(global.level_grid) && gy<ds_grid_height(global.level_grid))
								{
								if (global.level_grid[# gx,gy]>=0)
									{
									if (_debug) show_debug_message("Next Room!");
									//Go to the next room over
									global.current_level=global.level_grid[# gx,gy];
									global.lg_x=gx;
									global.lg_y=gy;
									global.wrap_player=true;
									global.g_state=Game_State.load;
									}
								else
									{
									if (_debug) show_debug_message("Restarted!");
									//Restart Room
									global.wrap_player=true;
									global.g_state=Game_State.load;
									}
								}
							else 
								{
								if (_debug) show_debug_message("Restarted!");
								//Restart Room
								global.wrap_player=false;
								global.g_state=Game_State.load;
								}
							}
						#endregion
						#region Draw
						//Draw Player
						draw_rectangle_color(global.player_x,global.player_y,global.player_x+size-1,global.player_y+size-1,c_black,c_black,c_white,c_white,false);
						#endregion
					#endregion
					break;
					}
				default: show_debug_message("Game State Machine is broken!"); break;
				}
			#endregion
			osg_metroidvania(Function.World_Map);
		#endregion
		break;
	case Function.Load:
		#region Load the level into a global.grid
		ds_grid_clear(global.grid,0);
		surface_set_target(global.surf);
		//Draw the level
		var c=c_black;
		var str=global.level[@global.current_level];
		var pos=0;
		for(var i=0;i<(level_width div size);i++)
			{
			for(var m=0;m<(level_height div size);m++)
				{
				pos=i+(m*(level_width div size))+1;
				//Add to global.grid
				ds_grid_add(global.grid,i,m,string_char_at(str,pos));
				//Draw something different depending on what's in the string
				switch(string_char_at(str,pos))
					{
					case "0": 
						{
						c=c_white;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "1":
						{
						c=c_black;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "2":
						{
						//If it is "2", spawn the player and change it to be a "0"
						c=c_white;
						//Absolute player coordinates
						if (!global.wrap_player)
							{
							global.player_x=(i*size)-1;
							global.player_y=(m*size)-1;
							}
						global.grid[# i,m]="0";
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					case "3":
						{
						c=c_lime;
						draw_rectangle_color(i*size,m*size,(i*size)+size-1,(m*size)+size-1,c,c,c,c,false);
						break;
						}
					default: break;
					}
				}
			}
			//Set the player position to loop the screen, if the player is being wrapped
			if (global.wrap_player)
				{
				if (global.player_x<=0)
					global.player_x=level_width-size-1;
				else if (global.player_x>=level_width-size)
					global.player_x=1;
				else if (global.player_y<=0)
					global.player_y=level_height-size;
				else if (global.player_y>=level_height-size-1)
					global.player_y=1;
				}
			surface_reset_target();
			global.g_state=Game_State.main;
			osg_metroidvania(Function.Display);
		#endregion
		break;
	case Function.Display:
		#region Draw the Game Surface
		draw_surface(global.surf,0,0);
		#endregion
		break;
	case Function.World_Map:
		#region Draw the World Map overlay
		for(var i=0;i<ds_grid_width(global.level_grid);i++)
			{
			for(var m=0;m<ds_grid_height(global.level_grid);m++)
				{
				var c=c_red;
				if (global.lg_x==i && global.lg_y==m)
					{
					c=c_lime;
					draw_set_alpha(0.4);
					draw_rectangle_color(i*size+1,m*size+1,i*size+size-2,m*size+size-2,c,c,c,c,false);
					}
				else
					{
					draw_set_alpha(0.4);
					draw_rectangle_color(i*size+1,m*size+1,i*size+size-2,m*size+size-2,c,c,c,c,false);
					}
				draw_set_alpha(1);
				}
			}
		#endregion
		break;
	}
#endregion
/*END*/