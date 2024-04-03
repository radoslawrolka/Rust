fn trim_me(input: &str) -> String {
    //input.trim().to_string()
    let mut start = 0;
    let mut end = input.len();
    for (i, c) in input.chars().enumerate() {
        if !c.is_whitespace() {
            start = i;
            break;
        }
    }
    for (i, c) in input.chars().rev().enumerate() {
        if !c.is_whitespace() {
            end = input.len() - i;
            break;
        }
    }
    String::from(&input[start..end])
}

fn compose_me(input: &str) -> String {
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

fn main() {
    assert_eq!(trim_me("Hello!     "), "Hello!");
    assert_eq!(trim_me("  What's up!"), "What's up!");
    assert_eq!(trim_me("   Hola!  "), "Hola!");


    assert_eq!(compose_me("Hello"), "Hello world!");
    assert_eq!(compose_me("Goodbye"), "Goodbye world!");


    assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
    assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
}