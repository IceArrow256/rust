#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        email: "icearrow256@gmail.com".to_owned(),
        username: "IceArrow256".to_owned(),
        active: false,
        sign_in_count: 0,
    };
    let user2 = User {        
        email: "icearrow128@gmail.com".to_owned(),
        username: "IceArrow128".to_owned(),
        ..user
    };
    println!("{:?}", user2);
}
