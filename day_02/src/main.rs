use std::{env::args, usize};

fn main() -> Result<(), std::io::Error> {
    let file_name: Vec<String> = args().collect();

    let input_string = std::fs::read_to_string(&file_name[1])?;
    let result = do_the_thing(input_string);

    println!("{result}");

    Ok(())
}

fn do_the_thing(input: String) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two() {
        assert_eq!(1, 0)
    }

    #[test]
    fn part_one() {
        assert_eq!(1, 0)
    }
}
