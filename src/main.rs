use std::io;
use rand::Rng;
use colored::*;
use std::{thread, time};

pub mod NPC;
pub mod weapons;

extern crate rand;
extern crate colored;
//fail is red, sucess is green and Orange is an option could also have the option only appear if level required.
//Player needs to select a starting weapon

struct Character{
    //create character fields that can be filled in like DnD stats.
    name: String,
    armour_class: i32,  //the calculation for this can be implemented more later.
    experince_points: i32,
    level: i32,
    hitpoints: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    max_hitpoints:i32,
    weapon: weapons::MeleeWeapon,  //might have to have a trait above MeleeWeapon like 'Weapon' which meleeweapon extends
    //if lie could have an alias to use?
    //could have an inventory class, which holds gold, silver, copper. and items, like health pots, etc..
}

impl Character{
    //methods which the player character can do.

    //create new player character with base stats of 8 for everything.
       pub fn new_player_character(name: String,selected_weapon: weapons::MeleeWeapon) -> Self {
        Character {
            name: name,
            armour_class: 15,
            experince_points: 0,
            level: 1, //update this by calling the experince
            hitpoints: 13,   //this will be updated based off level.
            strength: 15,
            dexterity: 14,
            constitution: 13,
            intelligence: 12,
            wisdom: 10,
            charisma: 8,
            max_hitpoints: 15,  //will need to update this later. Used to restore health when resting.
            weapon:selected_weapon,
        }
    }

//place holder values for now. Call this to update the level of the player with no gained xp.
    fn calculate_level(&mut self){
        let xp = self.experince_points;
        match xp{
             xp if xp < 300 => self.level = 1,
             xp if xp > 300 && xp < 900 => self.level = 2,
             xp if xp > 900 && xp < 2700 => self.level = 3,
             xp if xp > 2700 => self.level = 4,
             _ => return self.level = 0
        }
    }

    pub fn print_values(& self){
        println!("Name: {} Armour Class: {} Str: {} Dex: {} Con: {} Int: {} Wis: {} Char {} Level: {} Weapon: {}\n",
         self.name, self.armour_class, self.strength, self.dexterity, self.constitution, 
         self.intelligence, self.wisdom, self.charisma, self.level, self.weapon.name);
    }

    pub fn gain_xp(&mut self, xp_to_gain: i32){  //pass the defeated event/NPC, it will have an experince field.
        self.experince_points = self.experince_points + xp_to_gain;
        println!("{} has gained {}xp",self.name, xp_to_gain);
        self.calculate_level(); //checks to see if a level has been gained.
    }

    pub fn take_damage(&mut self, damage_to_take: i32){
        self.hitpoints = self.hitpoints - damage_to_take as i32;
        println!("{} Damage was dealt to {}",damage_to_take, self.name);
        println!("{} Hitpoints {} \n",self.name, self.hitpoints);
    }

    pub fn restore_hitpoints(&mut self){
        self.hitpoints = self.max_hitpoints;
        //println!("While resting you restore your hitpoints back to your max of {} ".green(), self.hitpoints);
         println!("{}", "While resting you restore your hitpoints back to full: ".green().bold());
         println!("Hitpoints {}/{}\n",self.hitpoints,self.max_hitpoints);
    }
}

//rolls random number and returns a value of damage to deal.                    need to roll to hit. 
fn roll_damage(max_roll: i32, dice_to_roll: i32) -> i32{
    thread::sleep(time::Duration::from_millis(3000));  //slight pause in the combat
    let mut damage_to_deal = 0;
    let mut rng = rand::thread_rng();
    for x in 0..dice_to_roll {
        //so say the max_roll is 6 and the dice_to_roll is 2 this will roll a d6 2 times and add the damage then return it.
        damage_to_deal = damage_to_deal +  rng.gen_range(1, max_roll);  //might need to be 1 - max_roll
    }
    return damage_to_deal;
}

//should be able to pass any type as long as it has the opponenet trait. ie Guard has the trait so it can be accepted.
//combat function which simulates the fight between 2 characters? eg guard and player returns a victor. has combat choices etc..
//This should be a extremly basic version. Designed just for the guard. Could have specific AI per opponenet.

fn combat_sim<T: NPC::Opponents> (mut player: Character, mut opp: T){   //Player is mutable as damage will be taken.  Maybe things like spell slots
    println!("<<STARTING FIGHT>>");
    let mut rng = rand::thread_rng();
    let opp_initiative = rng.gen_range(1, 20);
    let player_initiative = rng.gen_range(1, 20);
     //roll initiative, could also add a modifier here.

while player.hitpoints > 0 || opp.get_hp() > 0 {  //obviously stop the loop if either HP is 0.
    //implement Armour Class in the damage check.
    //took away the initative check, come back with a better idea.

            //player attacks first
            let damage_to_deal = roll_damage(player.weapon.roll_max, player.weapon.dice_to_roll);
            opp.take_damage(damage_to_deal);

            //player defated the guard
            if opp.get_hp() <= 0{
                opp.say_name();    //fix this up. get the actual string.
                print!(" has died\n");
                player.gain_xp(opp.get_xp());
                break;
            }

        //opp attacks first, need to get the rolls of the weapon the opp is using. 
        //more advanced later if no weapon and spells. But this is just a basic melee attack.
        let damage_to_deal = roll_damage(opp.get_weapon().roll_max, opp.get_weapon().dice_to_roll);
        player.take_damage(damage_to_deal);

        //guarded defeated the player. First time just knock unconcious dont die for good.
        if player.hitpoints <= 0{
            println!("{} {} ", player.name.bright_red(),"has passed out\n");
            break;
        }
  }

    //player lost the fight wont die as its the first fight. But go to jail alternative.
    if player.hitpoints <= 0{
        //go to jail but unconcious so take a different jail scene.
        jail_scene(player, false);
    }else{

    println!("As you walk over the body of the dead guard you see cluster of torches coming towards you");
    println!("Before you know it you are surronded by guards, aswell as archers pointing their bows directly at you");
    println!("As you go to speak one of the guards knocks you over the head and you black out..");
    jail_scene(player, true);
    //player won the fight more guards come and arresty the player.
    }
}

fn main() {
    //render second screen which displays information about the character
    let mut user_input = String::new();
    println!("Enter Name of your Character..");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    let character_name = user_input.trim().to_string();

    println!("Select a starting weapon");
    println!("(1). Longsword");
    println!("(2). Hand Axe");
    println!("(3). Spear");
    let mut answer = question_answer_function(3);

    let mut personal_weapon;
    if  answer == 1{
        personal_weapon = weapons::MeleeWeaponTraits::new_longsword();
    } else if answer == 2{
        personal_weapon = weapons::MeleeWeaponTraits::new_hand_axe();
    } else{
        personal_weapon = weapons::MeleeWeaponTraits::new_spear();
    }

    let mut player = Character::new_player_character(character_name, personal_weapon);
    player.restore_hitpoints();
    player.print_values();
    //new character setup finished.

    //move the entire player object into the town gate function.
    town_gate(player);
}
//pass a number which says how many questions there are, eg 3 so if a number 4 is entered it asks again
//easy to reuse code which returns a number from the terminal.
fn question_answer_function(num_of_questions: u64)-> u64{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    //throw an error the user inputted text isn't a unsigned int.
    let number: u64 = input.trim().parse().expect("Expected a Number");

    if number > num_of_questions || number <= 0{
        println!("Please enter a number between 1-{}",num_of_questions);
        question_answer_function(num_of_questions);
    }
    return number;
}

//first scene will always be a town gate. guard hails to you ask you're business here.
fn town_gate(player: Character){
    println!("You arrive outside of town after a long journey\nA guard calls down to you and asks what your business is, you reply:");
    //lie would require higher skills.
    println!("(1). {} {}", "(Lie)".cyan() , "Hello my name is Hagar, im a traveling merchant looking to purchase goods from the town.");
    println!("(2). I have no idea where I am, but my name is {}.\n", player.name);
    let mut answer = question_answer_function(2);

    //if option is lie can roll a random number for the guard and the player see who wins.
    if answer == 1{
        let mut rng = rand::thread_rng();
        let mut guard_roll = rng.gen_range(0, 10); //ignore stats for the moment just roll pure RNG
        let mut player_roll = rng.gen_range(0, 10);

            if guard_roll >= player_roll{  //could have genders.
                println!("{} {}", "(Fail)".bright_red() ,"You think im a fool boy? Do you think you look like a traveling merchant?");
                //guard knows its a lie so call them out on it.
                println!("I'll ask one more time or i'll have to deal with you, Whats your name and what is your business?\n");

                println!("(1). {} {}", "(Lie again)".cyan(), "I swear im a traveling merchant, I was robbed on the way..");
                println!("(2). Sorry sir I was scared, Im not sure where I am my name is {}\n", player.name);

                answer = question_answer_function(2);

                if answer == 1{
                    //roll against guard, this time with advantage as you've been caught once.
                    guard_roll = rng.gen_range(0, 10); 
                    player_roll = rng.gen_range(0, 8);

                    //lost another roll.
                    if guard_roll >= player_roll{
                        println!("{} {}", "(Fail)".bright_red() ,"Ahh thats it, im coming down to deal with you.");
                        //can run, fight or submit to arrest.
                         println!("The guard is coming down, what do you choose to do?\n");
                         println!("(1). Choose to run and hope nobody follows.");
                         println!("(2). Choose to submit to the guard, risking probable arrest.");
                         println!("(3). Fight the guard");
                         answer = question_answer_function(3);
                         //from here we will be changing scenes so regardless we will be moving from this function.

                         if answer == 1{
                            //run away new scene.
                         }else if answer == 2{
                            //jail, new scene didn't kill the guard.
                            jail_scene(player, false);
                         }else{
                             let guard: NPC::Guard =<NPC::Guard as NPC::Opponents>::new_easy_guard();
                             combat_sim(player, guard);
                             //we fight
                         }

                    }else {
                        println!("{} {}", "(Success)".green() ,"Ah perhaps you were right sorry about the miss understanding sir");
                        println!("The guard raises the gate and lets you into the town.\n");  
                    }

                }else if answer == 2{
                    //told the truth guard will deal with you normally.
                    println!("FILL IN ANSWER 2")                    
                }

            }else{  //have the alias here, allow for unique names.
                println!("{} {}", "(Success)".green() ,"Ahh yes welcome to the town of Gusthall sir, please enjoy your stay");
                //continue with the story, he lets the player in.
            }
         
    }else if answer == 2{
          println!("{} {}", "(Success)".green() ,"urgh another one fleeing, come on in then i'll set you up with someone who can help.");
    }

}

//character moves from the front gate to the jail. This is the option taken when the player submits or kills the guard then gets arrested.
fn jail_scene(mut player: Character, did_player_kill_guard: bool){
    let player_weapon_taken = player.weapon;  //player original weapon. taken away when entering jail.
    //weapon of the player is removed when going to jail. have an option to get it back?
     player.weapon = weapons::MeleeWeaponTraits::unarmed();
     //0 will be the choice that means you killed the guard and were arrested.
    let mut answer = 0;
    if did_player_kill_guard == false{
     println!("The guard arrests you. He takes you through the town as you arrive to a large jail building he takes you in");
     println!("You overhear him talking to the Jailers, as one comes out from behind a desk and takes you down to the cells.");
     println!("The jailer goes to remove your weapons.\n");

     println!("(1). Let him have the weapons.");
     println!("(2). Give him some attitude, make it difficult for him to get your weapons.");
     println!("(3). Offer him your weapons willingly but attempt to headbutt him when he comes closer\n");

    answer = question_answer_function(3);
    }

    if answer == 1{
        println!("The jailer takes your weapons, as you watch him leave he appears to put them in a room furthur back in the Jail.");
        println!("He comes back, removes your cuffs ,leads you into the jail and closes the cell door behind you.\n");
    }else if answer == 2{
        println!("You inform the jailer that you have slept with his mother , as has the rest of the town.");
        println!("He looks up at you and smiles, as he spits in your face. He then takes your weapons and takes them into a room furthur back in the Jail.\n");
        println!("He takes your cuffs off and pushes you into the cell..");
    }else if answer == 3{
        println!("The jailer easily dodges the headbutt, he reminds you that you are in cuffs you fucking idiot.");
        println!("He then punches you in the face, knocking you out. When you wake up you're in the cell and your weapons are gone.\n");
    }else if answer == 0{
        println!("You wake up after who knows how long, the back of your head feels like its been split open");
        println!("You attempt to rise, but only manage to sit on the cold floor of what appears to be a town jail\n");
    }

    
    println!("You look around the dimly lit cell, it appears to be a group cell with 4 other people in it");
    println!("What do you want to do?");
    println!("(1). Explore the cell");
    println!("(2). Sit down on one of the benches and rest");
    println!("(3). Go up to the bar and threaten to jailers\n");
    answer = question_answer_function(3);  
    // if exploring the cell what are you looking for?

    //option 1 is explore
    if answer == 1{
        println!("You start to look around the jail room.");
        println!("You see a small barred window, which a mysterious man is sitting next to.");
        println!("There are 4 other people in here, most of whom appear to be quite beaten up like they've been in some type of fight.");
        println!("As you notice this you also notice that the mysterious man appears to be in perfect health.");
        println!("You find nothign else of interest so you go sit down next to the man with the intention of resting for abit...\n");
        //option 2 is sitting down
    }else if answer == 2{
        println!("You go to sit down on the only avaiable spot on the bench");
        println!("As you sit you notice that a man sitting to your right.");
        println!("(1). Ignore him");
        println!("(2). Ask him his name, whats hes in for.");
        println!("(3). Tell him to stop fucking staring at you, even though he wasn't.");
        answer = question_answer_function(3); 

        //1 is ignore
        if answer == 1{
            println!("You sit down, barely able to keep your eyes open. Its been a long day..\n");
        //2 is ask questions
        }else if answer == 2{
            println!("You look at the man, he appears to be a human. As you go to speak to him he looks at you and shakes his head.");
            println!("He doesn't appear to be disintereted and gives you the impression that now isn't a great time, perhaps try again later.\n");
        //3 is start shit with him.
        }else if answer == 3{
            if did_player_kill_guard{
                println!("Even though you have just been in a fight and are still quite injuryed, you attempt to start shit with this random stranger...");
                println!("You show him some attitude, although he smirks at you then shakes his head.\n");
            }else{
                println!("You show him some attitude, although he smirks at you then shakes his head.");
            }
            println!("You get the impression that this mysterious person isn't someone you can intimidate.\n");
            println!("(1). Back down");
            println!("(2). Attempt to hit him.");
            answer = question_answer_function(2); 
            //back down
            if answer == 1{
                println!("As you back down he nods, as if to say that was the wise choice.");
                //attack him 
            }else if answer == 2{
                println!("In your infinite wisdom you attempt to attack the random stranger");
                println!("As the punch is about to land the man effortlessly grabs your arm to stop your strike.");
                println!("He sighs deeply then looks at you, he shakes his head once again..\nand this time you get the impression that perhaps you should not mess with this guy.\n");
                
            }
        }
    //3 is threaten the jailers.
    }else if answer == 3{
        println!("You decide that you haven't had enough punishment for the day so you go up to the bars and start abusing the jailers");
        println!("After yelling all sorts of profanities at nobody, you seem quite satisfied, you go take a seat along the bench");
        println!("As you sit to your right you notice a man with a rather intimidating aura next to you\n");
    }
     println!("Judging by the glow of the moon coming through the bars you think its approaching midnight");
     println!("Against your better judement you start to nod off where you are sitting...");
     //sleeping restores hitpoints
     player.restore_hitpoints();

    //jail break?
}

//option 2 from front gate, choose to run away
fn run_away_scene(mut player:Character){
    //need to loop around to the prision scene kinda.
    println!("Hello World");
}



//make a dice rolling method take 2 parameters, could possible take a player and opponenet to do more
//complex equation on their stats.  2 params could be the range to roll between.

//death method

//colour the damage dealt and to who