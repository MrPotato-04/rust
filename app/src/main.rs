use rand::Rng;
use std::{thread, time, process, io, cmp::Ordering};
use std::io::Write; 

fn main() {
    let space = "-------------------------------------------------------------------------------------";
    let won = "You won";
    //player-----------------------------------------------
    let mut healt: i8 = 100;
    let mut attack = 0.0;
    let mut response = 0;
    //---player-fleeing------------------------------------
    let flee = "well you got away with ";
    let mut flee_damage = 0;
    
    //healing---------------------------------------------
    let mut heal_count = 3;
    let healt_up = 100;
    
    //moves-----------------------------------------------
    let moves = ["Attack", "Flee", "Heal"];
    let moves_response = [
        "You choose too attack, how brave!",
        "Coward",
        "You healed for ",
    ];
    let mut previous_move = 0;
    
    
    
    //monster
    let mut monster_healt:i16 = 100;
    let mut monster_attack = 0.0;
    println!("{}",space);
    println!("You found a monster what will you do?");
    
    'main: loop {
        sleep();
        let hit = rand::thread_rng().gen_range(1..=100);
        println!("{}",space);
        println!("Your stats, healt: {}, Heal potions: {}",healt, heal_count );
        println!("{}",space);
        //if monster_healt < 0 {
        if monster_healt == 0 {println!("The monster is dead you won");  monster_healt = new_monster()};
        if monster_healt > 0 {println!("The monster is {} hp", monster_healt);};
        
        
        println!("1){}", moves[0]);
        println!("2){}", moves[1]);
        println!("3){}", moves[2]);
        println!("{}",space);
        //choose move loop
        'outer: loop {
            
            //player attack
            attack = rand::thread_rng().gen_range(1..=100) as f32;
            let crit = rand::thread_rng().gen_range(1..=5);
            let max_damage: i16 = 100;
            let mut input = String::new();
            
            //monster attack ----------------------------------
            monster_attack = rand::thread_rng().gen_range(1..=100) as f32;
            let monster_crit = rand::thread_rng().gen_range(1..=5);

            io::stdin().read_line(&mut input).expect("invalid input\n");
            
            let mut tmp_input = input.trim();
            match tmp_input {
                "exit" => {
                    process::exit(0);
                },
                "quit" => {
                    process::exit(0);
                }
                _ => {
                    
                },
                
            }
            let input: i16 = match input.trim().parse::<i16>() {
                Ok(num) => num,
                Err(_) => {
                    println!("invalid input please enter a number");
                    continue;},
            };
            
            
            match &input {
                1 => {
                    response = 0;
                    println!("{}", moves_response[response]);
                    sleep();
                    
                },
                2 => {
                    flee_damage = rand::thread_rng().gen_range(0..=100);
                    response = 1;
                    healt -= flee_damage;
                    println!("{}", moves_response[response]);
                    sleep();
                    println!("");
                    println!("{}{} damage", flee, flee_damage);
                    break 'main;
                },
                3 => {
                    if heal_count != 0 {
                        response = 2;
                        println!("{}{}",moves_response[response], healt_up);
                        sleep();
                        healt = 100;
                        heal_count -= 1;
                        healt -= monster_attack as i8;
                        println!("\n the monster hit you for {}", monster_attack);
                    } else {
                        println!("not enough healt potions left");
                    }
                    
                },
                _ => {
                    println!("invalid response");
                    continue;
                },
            };
            
            println!("");
            if response == 0 {
                if input <= (&hit + 100) && input >= (&hit - 100) {
                    
                    if crit == 3 {
                        attack = attack*1.5;
                        println!("OMG You hit a crit\n")
                    };
                    println!("you hit for {} damage", attack);
                    
                    monster_healt -= attack as i16;
                    
                    if monster_healt < 0 {
                        println!("");
                        println!("Yes you killed it", );
                        break 'main;
                    };
                    
                    sleep();
                    println!("");
                    //monster Attack---------------------------------------------------------------
                    
                    
                    
                    
                    if crit == 3 {
                        attack = attack*1.5;
                        println!("OMFG no the monster hit a crit")
                    };
                    healt -= monster_attack as i8;
                    
                    println!("The monster hit you for {} damage", monster_attack);
                    
                    if healt <= 0 {
                        break 'main;
                    }
                } else {
                    println!("bad hit you took {:?} damage", recieve_damage());
                }
            }
            break 'outer;
        }
        
    }
    println!("{}", space);
    if response == 1 {}
    else if healt > 0 {println!("You won");}
    else {println!("You lost");}
    println!("\nTime till quit");
    for i in (0..15).rev() {
        sleep();
        println!("{}", i);
    }
    process::exit(0);
}

fn recieve_damage() -> i16 {
    let damage: i16 = rand::thread_rng().gen_range(1..=100);
    return damage;
}

fn new_monster() -> i16 {
    let monster_healt = rand::thread_rng().gen_range(100..=300);
    return monster_healt;
}
fn sleep() {
    let one_sec = time::Duration::from_secs(1);
    let now = time::Instant::now();
    thread::sleep(one_sec)
}
