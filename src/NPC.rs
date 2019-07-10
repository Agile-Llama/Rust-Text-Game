use crate::weapons;
use crate::weapons::MeleeWeaponTraits;

pub trait Opponents {
    // Static method signature; `Self` refers to the implementor type.
    fn new_easy_guard() -> Guard;
    //about the Opponenet, all stats.
    fn say_name(&self); fn get_ac(&self) -> i32; fn get_xp(&self) -> i32; fn get_hp(&self) -> i32; fn get_str(&self) -> i32;
    fn get_dex(&self) -> i32; fn get_con(&self) -> i32; fn get_int(&self) -> i32; fn get_wis(&self) -> i32; fn get_char(&self) -> i32;
    fn print_values(&self);

    fn get_weapon(&self) -> &weapons::MeleeWeapon; 
    fn take_damage(&mut self, damage:i32);
}

pub struct Guard{
    armour_class: i32, 
    name: String, 
    experince_points: i32,  
    hitpoints: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    weapon: weapons::MeleeWeapon,
    //need an array of possible attacks? attack block etc.. gets more advanced with more advanced creatures..
}

//rather then make guard could make a human or something then have a make new guard inside of that.
impl Opponents for Guard{  //could have hard guards jailers etc...
    fn new_easy_guard() -> Self{
    Guard{
        armour_class: 12,  //the calculation for this can be implemented more later.
        name: String::from("Guard"),
        experince_points: 100,  //this is how much xp is awarded to the player when defeating this.
        hitpoints: 11,
        strength: 13,
        dexterity: 13,
        constitution: 12,
        intelligence: 12,
        wisdom: 10,
        charisma: 10,
        weapon: weapons::MeleeWeaponTraits::new_longsword(),  //could rol a random weapon
        //thins like weapon/ armor can be added later. Can be roleld against a table if you wish.
        }
    }

    fn get_ac(&self)->i32{
        return self.armour_class;
    }

    fn say_name(&self){
        print!("{}",self.name);
    }

     fn get_xp(&self)->i32{
        return self.experince_points;
    }

     fn get_hp(&self)->i32{
        return self.hitpoints;
    }
    //set hitpoints?

    fn get_str(&self) ->i32{
        return self.strength;
    }

    fn get_con(&self)->i32{
        return self.constitution;
    }

    fn get_dex(&self)->i32{
        return self.dexterity;
    }

    fn get_int(&self)->i32{
        return self.intelligence;
    }

    fn get_wis(&self)->i32{
        return self.wisdom;
    }

    fn get_char(&self)->i32{
        return self.charisma;
    }

    //prints all the values of the opponent.
    fn print_values(&self){
        println!("Name: {} Armour Class: {} Str: {} Dex: {} Con: {} Int: {} Wis: {} Char {} Weapon: {}",
         self.name, self.armour_class, self.strength, self.dexterity, self.constitution, 
         self.intelligence, self.wisdom, self.charisma, self.weapon.name);
    }

    fn get_weapon(&self) -> &weapons::MeleeWeapon{
        return &self.weapon;
    }

    fn take_damage(&mut self, damage: i32){
        self.hitpoints = self.hitpoints - damage as i32;
        println!("{} damage dealth to the {}",damage ,self.name);
        println!("{}'s Hitpoints {} \n",self.name,self.hitpoints);
    }
}

//opponenents need attacks they can do.Opponents
//attacks could be there own class, put them in a vector inside of the opponenet (eg guard)



