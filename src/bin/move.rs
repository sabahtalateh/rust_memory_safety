use std::ops::Deref;

struct Player {
    name: String,
    age: u8,
    description: String,
}

fn main() {
    let mut our_player = Player {
        name: "Ivan".to_string(),
        age: 25,
        description: "Hohen Mohen".to_string(),
    };

    mover(our_player);

//    Error. Use our_player after move.
//    println!("My name is {}, and I am being used after move.", our_player.name)

    let mut our_player = Player {
        name: "Ivan".to_string(),
        age: 25,
        description: "Hohen Mohen".to_string(),
    };

    let player_return = immutable_borrow(&our_player);
    change_name(&mut our_player);
    println!("My name is {}, and I am being used after mutable borrow.", our_player.name);

    let age = 8;
    age_mover(age);
    println!("Age {} used after move.", age);
}

fn mover(p: Player) -> Player {
    println!("I am {}. I have been moved into mover.", p.name);
    p
}

fn immutable_borrow(p: &Player) -> &Player {
    println!("I am {}. I have been immutably borrowed.", p.name);
    p
}

fn change_name(p: &mut Player) {
    p.name = "New Name".to_string()
}

fn age_mover(age: u8) -> u8 {
    println!("Age {} has been moved.", age);
    age
}

