#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }  // may be written without ,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    {
        let coin = Coin::Quarter(UsState::Alabama);
        value_in_cents(&coin);

        println!("{:?}", coin);
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => println!("add fancy hat"),
            7 => println!("remove fancy hat"),
            other => println!("move player to {}", other),
        }
    }

    {
        let dice_roll = 9;
        match dice_roll {
            3 => println!("add fancy hat"),
            7 => println!("remove fancy hat"),
            _ => println!("reroll"),
        }
    }
}
