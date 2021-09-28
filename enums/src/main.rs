enum IpVer {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Drink {
    CossacksVodka,
    EnergyDrink,
}

enum Consumable {
    MedicalSupplies,
    Food,
    Drink(Drink),
    Drugs,
}

fn use_consumable(consumable: Consumable) -> u32 {
    match consumable {
        Consumable::Food => 40,
        Consumable::MedicalSupplies => 500,
        Consumable::Drugs => 1000,
        Consumable::Drink(drink) => {
            println!("You drank {:?}!", drink);
            999
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn route(ip_ver: &IpVer) {}

fn main() {
    let four = IpVer::V4(127, 0, 0, 4);
    let six = IpVer::V6("::1".to_owned());
    route(&four);
    route(&six);
    let some_number = Some(5);
    let some_string = Some("a string");
    use_consumable(Consumable::Drink(Drink::CossacksVodka));
    let absent_number: Option<i32> = None;
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    let mut count = 0;
    let drink = Consumable::Drink(Drink::CossacksVodka);
    if let Consumable::Drink(drink) = drink {
        println!("You drank {:?}!", drink);
    } else {
        count += 1;
    }
    println!("{}", "Hello, World");
}
