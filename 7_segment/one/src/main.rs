fn main() {
    println!("Hello, world!");
}

fn num_to_segment(input: String) -> String {
    let nums = [
        (0, [1, 1, 1, 0, 1, 1, 1]),
        (1, [0, 0, 1, 0, 0, 1, 0]),
        (2, [1, 0, 1, 1, 1, 0, 1]),
        (3, [1, 0, 1, 1, 0, 1, 1]),
        (4, [0, 1, 1, 1, 0, 1, 1]),
        (5, [1, 1, 0, 1, 0, 1, 1]),
        (6, [1, 1, 0, 1, 1, 1, 1]),
        (7, [1, 0, 1, 0, 0, 1, 0]),
        (8, [1, 1, 1, 1, 1, 1, 1]),
        (9, [1, 1, 1, 1, 0, 1, 1]),
    ];
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_num_to_segment() {
        assert_eq!(
            num_to_segment("00 00".to_string()),
            "-  -   -  -
| || | | || |
 -  -   -  -
| || | | || |
 -  -   -  -"
                .to_string()
        )
    }
}
