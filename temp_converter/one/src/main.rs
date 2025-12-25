use std::env;

/// when using this as a stand alone program, you must supply a single
/// temperature number & measure character (f, c, or k) in that order.
/// example:
/// ```
/// ./target/release/one 85 f
/// ```
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
        let temp = args[1].parse::<f32>().expect("temp not a number");
        let measure = args[2].chars().next().expect("couldn't find measure");
        println!("{:?}", convert_temp(temp, measure));
    } else {
        println!("Please supply a temperature & a measure type (f, c, or k)");
    }
}

/// convert_temp accepts a temp (f32) & a measure (char) and then returns
/// an array of tuples of the temperature in all 3 measures (f, c, & k).
fn convert_temp(temp: f32, measure: char) -> [(f32, char); 3] {
    match measure {
        'f' => [
            (temp, measure),
            ((temp - 32.0) / 1.8, 'c'),
            ((temp - 32.0) / 1.8 + 273.15, 'k'),
        ],
        'c' => [
            (temp * 1.8 + 32.0, 'f'),
            (temp, measure),
            (temp + 273.15, 'k'),
        ],
        'k' => [
            ((temp - 273.15) * 1.8 + 32.0, 'f'),
            (temp - 273.15, 'c'),
            (temp, measure),
        ],
        _ => {
            println!("please supply measure of 'f', 'c', or 'k'");
            [(0.0, ' '); 3]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_temp() {
        assert_eq!(
            convert_temp(85.0, 'f'),
            [(85.0, 'f'), (29.444445, 'c'), (302.59445, 'k')]
        );
        assert_eq!(
            convert_temp(100.0, 'c'),
            [(212.0, 'f'), (100.0, 'c'), (373.15, 'k')]
        );
    }
}
