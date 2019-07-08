mod client;

mod network;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        client::connect();
    }
}
