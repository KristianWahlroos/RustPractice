fn main() {
    tuple_test((1,2,3));
    refutable_some(Some(3));
    funny_unintended_accident();
    multi_pattern();
    the_at_annotation();
    println!("Hello, world!");
}

fn while_basic() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_basic() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn tuple_test((a, b, c): (i32, i32, i32)) {
    let (x, y, z) = (a, b, c);
    //let (x, y) = (1, 2, 3); //Can't do this
    let k = (1, 2, 3);
    println!("x {}, y {}, z {}, k {:?}", x, y ,z, k);
}

fn refutable_some(some_option_value: Option<i32>) {   
    if let Some(x) = some_option_value {
        println!("Jou dj {}", x);
    }
}

fn funny_unintended_accident(){
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched?? or did we, y = {:?}", y),
        Some(not_y) => println!("Matched, not_y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multi_pattern() {
    let x = 2;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    4...6 => println!("four through six"),
    _ => println!("anything"),
    }

    let y = 'c';
    match y {
        'a'...'j' => println!("a to j"),
        'k'...'z' => println!("k to z"),
        _ => println!("Something something"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn struct_pattern() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn in_case_of_none(){
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_),Some(_)) => {
            println!("Can't do. These are set alreadyw");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
}

fn useless_anti_warning_variable_yay() {
    //NO WARNING HERE!
    let _x = 5;
}

fn match_guard() {
    let num = Some(4);
    let y = false;

    match num {
        Some(x) if x < 5 => println!("Jou jou under 5"),
        Some(x) => println!("Jou jou not under 5"),
        None => (),
    }

    let thing = 4;

    match thing {
        4 | 5 | 6 if y => println!("jou jou"),
        _ => println!("no")
    }
}

enum Message {
    Hello { id: i32 },
}

fn the_at_annotation() {
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10...12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
    }
}