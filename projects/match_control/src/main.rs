
#[derive(Debug)] // 这样可以可以立刻看到州的名称
enum UsState {
    _Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
    let some_u8_value = 1u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);

    let coin = Coin::Penny;
    value_in_cents(coin);

    let some_u8_value = Some(0u8);
    if let Some(3) =  some_u8_value {
        println!("three");
    }

    let coin = Coin::Penny;
    let mut _count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        _count += 1;
    }

} 
 
