fn main() {
    let greet = say_hello("John");
    println!("{}", greet);
}

fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello("John"), "Hello, John!");
    }
}
