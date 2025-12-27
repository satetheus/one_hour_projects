use nom::character::complete::{digit1,one_of};
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    todo!();
}


fn operation(num1: i32, operator: char, num2: i32) -> i32 {
    match operator {
        '+' => num1+num2,
        '-' => num1-num2,
        '*' => num1*num2,
        '/' => num1/num2,
        _ => todo!(), // this should never happen because of parsing
    }
}


fn simple_calc(input: String) -> i32 {
    let op = parse_calc(&input).expect("error parsing string").1;

    operation(op.0.parse::<i32>().unwrap_or(0), op.1, op.2.parse::<i32>().unwrap_or(0))
}


fn parse_calc(input: &str) -> IResult<&str, (&str, char, &str)> {
    tuple((digit1,one_of("+-/*"),digit1))(input)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_calc() {
        assert_eq!(simple_calc("1+1".to_string()), 2);
        assert_eq!(simple_calc("100-1".to_string()), 99);
        assert_eq!(simple_calc("37*73".to_string()), 2701);
        assert_eq!(simple_calc("35/5".to_string()), 7);
        assert_eq!(simple_calc("22/5".to_string()), 4);
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        simple_calc("22/0".to_string());
    }
}
