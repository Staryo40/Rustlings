fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
    let trim_result_1 = trim_me("Hello!     ");
    println!("Trim result 1: {}", trim_result_1);
    let trim_result_2 = trim_me("  What's up!");
    println!("Trim result 2: {}", trim_result_2);
    let trim_result_3 = trim_me("   Hola!  ");
    println!("Trim result 3: {}", trim_result_3);

    // Compose a string
    let compose_result_1 = compose_me("Hello");
    println!("Compose result 1: {}", compose_result_1);
    let compose_result_2 = compose_me("Goodbye");
    println!("Compose result 2: {}", compose_result_2);

    // Replace a string
    let replace_result_1 = replace_me("I think cars are cool");
    println!("Replace result 1: {}", replace_result_1);
    let replace_result_2 = replace_me("I love to look at cars");
    println!("Replace result 2: {}", replace_result_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
