use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {}

fn problem_1() {
    let f = File::open("not_found.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("not_found.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Ongelmia luomisessa {:?}", e),
            },
            other_error => panic!("Ongelmia avaamisessa {:?}", other_error),
        },
    };
}

//Long version
fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//Short version
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
