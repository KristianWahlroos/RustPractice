use std::collections::HashMap;

fn main() {}

fn create_simple_hashmap() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn cool_hashmap_word_counter() {
    let text = "nelkyt viis ja nelkyt kaks ja siitä viis";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)
}

fn overwriting_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(25);

    println!("{:?}", scores);
}

fn overwriting_maybe_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn vector_enum() {
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(42),
        SpreadSheetCell::Text(String::from("Ja siitä")),
        SpreadSheetCell::Float(5.0),
    ];
}

fn vector_bug() {
    let mut v = vec![1, 2];

    let first = &v[0];

    //Can't do this. Pushing might change vector's place in memory. (If it doesn't have space in heap.)
    //"first" would in that case point to empty place in memory.
    //v.push(3);
}

fn vector_for() {

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn string_combine() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; //Compiler will turn &s2 into &s2[..]
}

fn string_magic() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

