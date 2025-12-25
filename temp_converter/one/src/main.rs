fn main() {
    todo!();
}

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
