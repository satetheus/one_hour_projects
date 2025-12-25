fn main() {
    todo!();
}


fn check_armstrong_number(number: i32) -> bool {
    let len = number.checked_ilog10().unwrap_or(0)+1;

    let digits: i32 = number.to_string().chars().map(|n| (n.to_digit(10).expect("not a number") as i32).pow(len)).sum();

    digits == number
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_armstrong_number() {
        assert!(check_armstrong_number(153));
        assert!(check_armstrong_number(371));
        assert!(!check_armstrong_number(61));
        assert!(!check_armstrong_number(1750));
    }
}
