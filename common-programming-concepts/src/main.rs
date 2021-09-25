const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Max points is: {}", MAX_POINTS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let y = 5;
    let y = y + 1;
    println!("{}", y);
    let spaces = "      ";
    let spaces = spaces.len();
    println!("Spaces: {}",  spaces);
    let t = true;
    let f: bool = false;
    println!("{} != {}", t, f);
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("x = {}, y = {}, z = {}", tup.0, tup.1, tup.2);
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let a  = [3; 5];
    println!("{:?}", a);
    let first = a[0];
    println!("first = {}", first);
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}, {:?}", x, y);
    let number = 6;
    if number == 4 {
        println!("number is 4");
    } else if number == 6 {
        println!("number is 6");
    } else {
        println!("number ?");
    }
    let contition = true;
    let test = if contition {5} else {6};
    println!("{}", test);
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..=4).rev() { 
        println!("{}!", number);
    }
    let test = [("a", 1), ("b", 2)];
    for (a, b) in test.iter() {
        println!("{} and {}", a, b);
    }

}
