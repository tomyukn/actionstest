fn main() {
    let greet = say_hello();
    println!("{}", greet);
}

fn say_hello() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello(), "Hello, world!");
    }
}
