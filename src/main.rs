use std::io;

fn main() {
    println!("Welcome to the adventure!");

    let mut room = 1;

    loop {
        match room {
            1 => {
                println!("You are in a dark room.");
                println!("You see a door to the north.");
                println!("Enter 'north' to move to the next room.");

                loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    if input.trim() == "north" {
                        room = 2;
                        break;
                    } else {
                        println!("You can't go that way.");
                    }
                }
            }
            2 => {
                println!("You are in a bright room.");
                println!("You see a door to the south and a chest.");
                println!("Enter 'south' to go back to the previous room.");
                println!("Enter 'open chest' to open the chest.");

                loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    if input.trim() == "south" {
                        room = 1;
                        break;
                    } else if input.trim() == "open chest" {
                        println!("You found a key inside the chest!");
                    } else {
                        println!("You can't do that.");
                    }
                }
            }
            _ => {
                println!("You are lost in the adventure!");
                break;
            }
        }
    }

    println!("Thanks for playing!");
}
