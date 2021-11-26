use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut heal_count = 3;
    let healt_up = 100;
    let moves = ["Attack", "Flee", "Heal"];
    let moves_response = [
        "You choose too attack, how brave!",
        "Coward",
        "You healed for {:?}",
    ];
    let mut response = 0;

    'main: loop {
        let hit = rand::thread_rng().gen_range(1..=100);

        println!("You found a monster what will you do?");
        println!("1){:?}", moves[0]);
        println!("2){:?}", moves[1]);
        println!("1){:?}", moves[2]);
        println!("");
        //choose move loop
        'outer: loop {
            let max_damage: i16 = 100;
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("invalid input\n");

            let input: i16 = input.trim().parse::<i16>().unwrap();

            match &input {
                1 => response = 0,
                2 => response = 0,
                3 => response = 0,
                _ => {
                    println!("invalid response");
                    continue;
                }
            };
            println!("{:?}", moves_response[response]);
            println!("");
            if input <= (&hit + 100) && input >= (&hit - 100) {
                
            } else {
                println!("bad hit you took {:?} damage", recieve_damage());
                
            }
        }
    }
}

fn recieve_damage() -> i16 {
    let damage: i16 = rand::thread_rng().gen_range(1..=100);
    return damage;
}
