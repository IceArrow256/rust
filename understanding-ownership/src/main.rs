fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    println!("{}, {}", s1, s2);
    let mut x = (1, 3);
    let y = x;
    x.0 = 6;
    println!("x: {:?}, y: {:?}", x, y);
    s2 = takes_and_gives_back(s2);
    println!("{}", s2);
    takes_ownership(s2);
    // println!("{}", s2);
    let mut s1 = String::from("f-word");
    {
        let s3 = &mut s1;
        s3.push_str("you can because mutable refence");
    }
    let s2 = &s1; // if you have 1 mut ref you, cant make more borrow, but if borrow in other scope you can
    println!("{}", s2.len());
    // s2.push_str("you just can't because immutable refence")
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    let text = String::from("You idiot!");
    let you = first_word(&text);
    // text.clear();
    // [..2]     0 -> 2
    // [..value] 0 -> value
    // [..]      0 -> len
    

    println!("{}", you);
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// fn dangle() -> &String {
fn dangle() -> String {
    let s = String::from("hello");

    // &s
    s
}

fn takes_and_gives_back(string: String) -> String {
    string
}

fn takes_ownership(string: String) {
    println!("{} it's mine not your", string);
}
