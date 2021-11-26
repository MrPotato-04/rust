use rand::Rng;
use std::cmp::Ordering;
use std::io;



fn main() {
    let mut heal_count = 3;
    let moves = ["Attack", "Flee", "Heal"];
    
    'main:loop {
        let hit = rand::thread_rng().gen_range(1..=100);
        
        println!("You found a monster what will you do?");
        
        let max_damage: i16 = 100;
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("invalid input");
        
        let input: i16 = input.trim().parse::<i16>().unwrap();
        
        match &input {
            1 => println!("test"),
            2 => println!("no"),
            _ => println!("noooooo"),
        };
        if input <= (&hit+100) && input >= (&hit-100) {
            continue;
        } else {
            let damage = recieve_damage();
            println!("bad hit you took {:?} damage", &damage);
        }
    }
}

fn recieve_damage() -> i16 {
    let damage: i16 = rand::thread_rng().gen_range(1..=100);
    return damage;
}

