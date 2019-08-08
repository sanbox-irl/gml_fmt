/// @Param type
/// @Param x
/// @Param y
/// @Param arg3

switch(argument[0])
    {
    case "impact blood":
        {
        repeat(4+irandom(5)) particle_add_blood(argument[1],argument[2],-8,random(10),-argument[3]+random_range(-10,10),-random(20));
        repeat(4+irandom(5)) particle_add_blood(argument[1],argument[2],-8,random(30),argument[3]+random_range(-10,10),-random(20));
        repeat(4+irandom(5)) particle_add_blood(argument[1],argument[2],-8,random(10),argument[3]+random_range(-25,25),-random(20));
        repeat(4+irandom(5)) particle_add_blood(argument[1],argument[2],-8,random(10),random(360),-random(20));
        }break;
    
    case "drip blood":
        {
        particle_add_blood(argument[1],argument[2],argument[3],0,0,0,choose(1,2,4,5));
        }break;
    
    case "skeleton gibs":
        {
        repeat(argument[3]) particle_add_type(argument[1],argument[2],-8,1+random(3),random(360),random(-1),Spr_Skeleton_Gibs,irandom(sprite_get_number(Spr_Skeleton_Gibs)-1),random(360),random_range(-10,10),"outline");
        }break;
    
    case "weapon drop":
        {
        particle_add_ext(argument[1],argument[2],-8,3+random(3),random(360),-2-random(1),argument[3],0,random(360),random_range(-30,30),1,2,1,1,1,"outline");
        }break;
    
    case "bullet casing":
        {
        particle_add_ext(argument[1],argument[2],-8,4.5+random(2.5),argument[3],-2-random(1),Spr_BulletCasing,0,random(360),random_range(-360,360),1,2.25,1,.8,1,"");
        }break;
    }