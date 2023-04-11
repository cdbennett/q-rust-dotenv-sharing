pub fn say_hello() {
    let name = get_name();
    println!("Hello, {name} from the Library Project");
}

pub fn do_stuff() {
    let count = get_count();
    for i in 1..=count {
        println!("  Looping {i} of {count} times...");
    }
    println!("Goodbye, {}", get_name());
}

fn get_name() -> String {
    std::env::var("MY_NAME").unwrap_or("Anonymous".to_owned())
}

fn get_count() -> u32 {
    std::env::var("COUNT")
        .map(|s| s.parse().expect("should parse as number"))
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name_without_var() {
        std::env::remove_var("MY_NAME");

        assert_eq!(get_name(), "Anonymous");
    }

    #[test]
    fn test_get_name_with_var_set() {
        std::env::set_var("MY_NAME", "Roberto");

        assert_eq!(get_name(), "Roberto");
    }
}
