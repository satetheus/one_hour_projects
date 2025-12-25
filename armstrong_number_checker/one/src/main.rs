use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for arg in &args[1..] {
            println!("{}: {}", arg.clone(), check_armstrong_number(arg));
        }
    } else {
        println!("Please supply a number to check");
    }
}


fn check_armstrong_number(number: &String) -> bool {
    let len = number.len() as u32;

    let digits: i32 = number.chars().map(|n| (n.to_digit(10).expect("not a number") as i32).pow(len)).sum();

    digits == number.parse::<i32>().expect("not a number")
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_armstrong_number() {
        assert!(check_armstrong_number(&"153".to_string()));
        assert!(check_armstrong_number(&"371".to_string()));
        assert!(!check_armstrong_number(&"61".to_string()));
        assert!(!check_armstrong_number(&"1750".to_string()));
    }
}
