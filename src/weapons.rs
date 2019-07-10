//weapons will all have their own actions.
//eg spears have standard spear, and throw the spear.

pub trait MeleeWeaponTraits{
    fn new_spear() -> Self;
    fn new_hand_axe() -> Self;
    fn new_longsword() -> Self;
}

pub struct MeleeWeapon{
    pub name: String,
    two_handed: bool,
    throwable: bool,
    pub dice_to_roll: i32, //this is the amount of times to roll the roll_max eg 2d8 will roll a 8 sided die 2 times.
    pub roll_max: i32,  //this is the max its damage can roll. eg 1d6 will roll between a 0-6
    damage_type: String,
    cost: i32,
}

impl MeleeWeaponTraits for MeleeWeapon{ 
//spear, 2h, axe, 1h, etc..
fn new_spear() -> Self{
    MeleeWeapon{
        name: String::from("Spear"),
        two_handed: false,
        throwable: true,
        dice_to_roll: 1,
        roll_max: 6,
        damage_type: String::from("Piercing"), 
        cost: 1,
        }
    }

    fn new_hand_axe()-> Self{
    MeleeWeapon{
        name: String::from("Hand Axe"),
        two_handed: false,
        throwable: true,
        dice_to_roll: 1,
        roll_max: 6,
        damage_type: String::from("Slashing"), 
        cost: 5,
        }   
    }

    fn new_longsword() -> Self{
    MeleeWeapon{
        name: String::from("Longsword"),
        two_handed: false,
        throwable: false,
        dice_to_roll: 1,
        roll_max: 8,
        damage_type: String::from("Slashing"), 
        cost: 15,
        }
    }
}

