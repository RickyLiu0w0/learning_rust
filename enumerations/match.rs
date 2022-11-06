#[derive(Debug)]

enum CnState {
    GZ,
    BJ,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(CnState),
}

fn val_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }

        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
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

fn deplay(x: Option<i32>) {
    /*
    match x {
        Some(i) => println!("The num is {}", i),
        _ => (),
    }
    */
    if let Some(i) = x {
        println!("The num is {}", i);
    } else {
        println!("None");
    }
}

fn main() {
    val_in_coin(Coin::Quarter(CnState::BJ));
    val_in_coin(Coin::Penny);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    deplay(six);
    deplay(none);
}
