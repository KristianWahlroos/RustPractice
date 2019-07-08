enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
enum Thing {
    Titanium,
    Steel,
    Plant,
    Energy,
    Heat,
}

enum Coin {
    Bronze,
    Silver,
    Gold,
    Custom(Thing),
}

fn value_in_mega_credits(coin: Coin) -> u32 {
    match coin {
        Coin::Bronze => 1,
        Coin::Silver => 5,
        Coin::Gold => {
            println!("Nice!");
            10
        },
        Coin::Custom(thing) => {
            println!("Special type {:?}!", thing);
            25
        }
    }
}

fn count_customs(coin: Coin) -> u32 {
    let mut count = 0;
    match coin {
        Coin::Custom(thing) => println!("Custom's type is {:?}!", thing),
        _ => count += 1,
    }
    count
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

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
let six = plus_one(five);
let none = plus_one(None);
let home = IpAddr::V4(127,0,0,1);
let loopback = IpAddr::V6(String::from("::1"));    
let m = Message::Write(String::from("Hello"));
m.call();
    println!("Hello, world!");

let coin1 = Coin::Bronze;
let coin2 = Coin::Bronze;
let coin3 = Coin::Silver;
let coin4 = Coin::Gold;
let coin5 = Coin::Gold;
let coin6 = Coin::Custom(Thing::Energy);
let coin7 = Coin::Custom(Thing::Titanium);

println!("Hello, coin, {}", count_customs(coin1));
println!("Hello, coin, {}", count_customs(coin2));
println!("Hello, coin, {}", count_customs(coin3));
println!("Hello, coin, {}", count_customs(coin4));
println!("Hello, coin, {}", count_customs(coin5));
println!("Hello, coin, {}", count_customs(coin6));
println!("Hello, coin, {}", count_customs(coin7));
}