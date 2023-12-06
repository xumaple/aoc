use std::collections::HashMap;
use util::*;

pub fn run(filename: &str) -> Result<i32, BoxError> {
    let maxes: HashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    let mut sum = 0;
    for l in read_lines(filename)? {
        let l = l?;
        let (game_num_str, hands) = l.split_once(": ").expect("Did not find ':' in game line");
        let game_num: i32 = unsafe { game_num_str.get_unchecked(5..) }.parse()?;

        let mut on_num = true;
        let mut prev_num: i32 = -1;
        let mut game_success = true;
        for token in hands.split(" ") {
            match on_num {
                true => {
                    prev_num = token
                        .parse::<i32>()
                        .expect(format!("Unable to parse int: {token}").as_str());
                    on_num = false;
                }
                false => {
                    if prev_num
                        > *maxes
                            .get(&token.trim_end_matches(&[',', ';']))
                            .expect(format!("Unable to identify color {token}").as_str())
                    {
                        game_success = false;
                        break;
                    }
                    on_num = true;
                }
            }
        }
        if game_success {
            sum += game_num;
        }
    }
    Ok(sum)
}

#[allow(dead_code)]
fn main() -> NulBoxError {
    println!("{}", run("src/d02/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod test_02a {
    use super::run;

    #[test]
    fn official() {
        assert_eq!(run("src/d02/input.txt").unwrap(), 2476)
    }
}
