#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn this_is_an_awkward_silence() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    #[ignore]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 4, width: 5 };

        assert!(larger.can_hold(&smaller));
    }

        #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 4, width: 5 };

        assert!(!smaller.can_hold(&larger));
    }

        #[test]
        #[should_panic(expected = "Your test was f*cked, Oh no! value was 'Hello Matti&Teppo#JunaLife!'")]
        fn greetings_test() {
        let result = we_greet_or_something_idk_yet("Matti&Teppo#JunaLife");
        assert!(result.contains("Matti&Teppo#HarjoitusLife"), "Your test was f*cked, Oh no! value was '{}'", result);
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(3, add_two(2));
    }
    // #[test]
    // fn you_have() {
    //     panic!("FAILOR!");
    // }

}


pub fn we_greet_or_something_idk_yet(name: &str) -> String {
    format!("Hello {}!", name)
}
pub fn add_two(a: i32) -> i32{
    a + 2
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
