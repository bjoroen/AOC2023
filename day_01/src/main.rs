use std::{env::args, usize};

fn main() -> Result<(), std::io::Error> {
    let file_name: Vec<String> = args().collect();

    let input_string = std::fs::read_to_string(&file_name[1])?;
    let result = do_the_thing(input_string);

    println!("{result}");

    Ok(())
}

fn do_the_thing(input: String) -> usize {
    let mut c = 0;
    let source: Vec<char> = input.chars().collect();
    let input_len = source.len();

    let mut result = vec![0];
    let mut first = '0';
    let mut last = '0';

    while c != input_len {
        if source[c] == '\n' {
            let concat_string = first.to_string() + &last.to_string();
            let as_int = concat_string.parse::<usize>().unwrap();
            result.push(as_int);
            first = '0';
            last = '0';
            c += 1;
            continue;
        }

        match source[c] {
            _ if source[c].is_numeric() => {
                if first == '0' {
                    first = source[c];
                    last = source[c];
                    c += 1;
                    continue;
                } else {
                    last = source[c];
                    c += 1;
                    continue;
                }
            }
            _ if source[c].is_alphabetic() => {
                let mut buffer = String::new();
                buffer.push(source[c]);
                c += 1;
                while source[c].is_alphabetic() {
                    buffer.push(source[c]);
                    match string_to_number(&buffer) {
                        Some(v) => {
                            if first == '0' {
                                first = v;
                                last = v;
                                break;
                            } else {
                                last = v;
                                break;
                            }
                        }
                        None => {}
                    }

                    c += 1;
                }
            }
            _ => c += 1,
        }
    }

    result.iter().sum()
}

fn string_to_number(num_str: &str) -> Option<char> {
    let nums = vec![
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for num in nums {
        if num_str.contains(num.0) {
            return Some(num.1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two() {
        let test_data = String::from(
            "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            ",
        );
        let result = do_the_thing(test_data);

        assert_eq!(result, 281)
    }

    #[test]
    fn part_one() {
        let test_data = String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n");
        let result = do_the_thing(test_data);

        assert_eq!(result, 142)
    }
}
