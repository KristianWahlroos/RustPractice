fn main() {
    println!("Siitä {}!", another_function(25));
    let s = String::from("jou");
    println!("{}{}", jou(&s), s);

    let s2 = "Hello World";

    let word = first_word(&s2[..]);

    let user = create_user(String::from("Matti"), String::from("Teppohotmailcom"));

    println!("{}",word);

}

fn another_function(x: i32) -> i32{
    println!("{}!", x);
    x - 20
}

fn jou(s: &String) -> String{
    s.to_string()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
        &s[..]
}

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn create_user(username: String, email: String) -> User{
    User {
        email,
        username,
        active : true,
        sign_in_count : 3,
    }
}